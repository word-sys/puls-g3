mod types;
mod utils;
mod config;
mod monitors;
pub mod gui;
mod language;
mod system_service;
mod error_logger;

use crate::types::AppState;
use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};


use parking_lot::Mutex;
use gtk::prelude::*;
use gtk::Application;
use tokio::time::sleep;

use clap::Parser;
use crate::config::{Cli};
use crate::monitors::DataCollector;
use crate::types::AppConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    init_logging(cli.verbose)?;
    let config = AppConfig::from(cli);
    
    let app_state = Arc::new(Mutex::new(AppState::default()));
    let data_collector = Arc::new(tokio::sync::Mutex::new(DataCollector::new(config.clone())));
    
    let system_info = {
        let collector = data_collector.try_lock().unwrap();
        collector.get_system_info()
    };
    
    {
        let mut state = app_state.lock();
        state.system_info = system_info;
        
        if config.safe_mode {
            state.system_info.push(("Mode".to_string(), "Safe Mode".to_string()));
        }
        
        let sys_mgr = system_service::SystemManager::new();
        state.has_sudo = sys_mgr.has_sudo_privileges();
        
        state.services = sys_mgr.get_services();
        state.logs = sys_mgr.get_logs(50, None, None);
        state.config_items = sys_mgr.get_grub_config();
        
        state.boots = sys_mgr.get_boots();
        if !state.boots.is_empty() {
            state.current_boot_idx = 0;
        }
    }
    
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let app_state_clone = app_state.clone();
    let data_collector_clone = data_collector.clone();
    let config_clone = config.clone();

    rt.spawn(async move {
        data_collection_loop(app_state_clone, data_collector_clone, config_clone).await;
    });

    gtk::init()?;

    let app = Application::builder()
        .application_id("com.github.word-sys.puls-g3")
        .build();

    let app_state_for_gui = app_state.clone();
    let config_for_gui = config.clone();
    
    app.connect_activate(move |a| {
        crate::gui::build_ui(&a, app_state_for_gui.clone(), config_for_gui.clone());
    });

    app.run();

    Ok(())
}

async fn data_collection_loop(
    app_state: Arc<Mutex<AppState>>,
    data_collector: Arc<tokio::sync::Mutex<DataCollector>>,
    config: AppConfig,
) {
    let mut interval = tokio::time::interval(config.get_collection_sleep_duration());
    let mut prev_global_usage = types::GlobalUsage::default();
    
    let mut cycle_count: u32 = 0;

    loop {
        interval.tick().await;
        
        let is_paused = {
            let state = app_state.lock();
            state.paused
        };
        
        if is_paused {
            continue;
        }
        
        let collection_start = Instant::now();
        
        let (selected_pid, show_system_processes, filter_text, sort_by, sort_ascending) = {
            let state = app_state.lock();
            (
                state.selected_pid,
                state.show_system_processes,
                state.filter_text.clone(),
                state.sort_by.clone(),
                state.sort_ascending,
            )
        };
        
        let new_data = {
            let mut collector = data_collector.lock().await;
            collector.collect_data(
                selected_pid,
                show_system_processes,
                &filter_text,
                &sort_by,
                sort_ascending,
                prev_global_usage.clone(),
            ).await
        };
        
        prev_global_usage = new_data.global_usage.clone();
        
        {
            let mut state = app_state.lock();
            // Preserve the user-selected process detail across data refreshes
            let preserved_detail = state.dynamic_data.detailed_process.take();
            state.dynamic_data = new_data;
            state.dynamic_data.detailed_process = preserved_detail;
        }

        // Refresh logs, services, and config periodically (every 10 cycles)
        cycle_count += 1;
        if cycle_count % 10 == 0 {
            let sys_mgr = system_service::SystemManager::new();
            let logs = sys_mgr.get_logs(100, None, None);
            let services = sys_mgr.get_services();
            let config_items = sys_mgr.get_grub_config();
            {
                let mut state = app_state.lock();
                state.logs = logs;
                state.services = services;
                state.config_items = config_items;
            }
        }
        
        let collection_duration = collection_start.elapsed();
        
        if collection_duration > Duration::from_millis(config.refresh_rate_ms / 2) {
            eprintln!("Slow data collection: {:?}", collection_duration);
        }
        
        let remaining_time = config.get_collection_sleep_duration().saturating_sub(collection_duration);
        if remaining_time > Duration::from_millis(10) {
            sleep(remaining_time).await;
        }
    }
}

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Config(String),
    Monitor(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO Error: {}", e),
            AppError::Config(e) => write!(f, "Configuration Error: {}", e),
            AppError::Monitor(e) => write!(f, "Monitoring Error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}



fn init_logging(verbose: bool) -> Result<(), AppError> {
    if verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_display() {
        let io_error = AppError::Io(io::Error::new(io::ErrorKind::NotFound, "test"));
        assert!(format!("{}", io_error).contains("IO Error"));
        
        let config_error = AppError::Config("test config error".to_string());
        assert!(format!("{}", config_error).contains("Configuration Error"));
        
        let monitor_error = AppError::Monitor("test monitor error".to_string());
        assert!(format!("{}", monitor_error).contains("Monitoring Error"));
    }
}