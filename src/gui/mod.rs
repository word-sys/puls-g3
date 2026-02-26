use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Orientation};
use gtk::gdk;
use std::sync::Arc;
use parking_lot::Mutex;
use glib::ControlFlow;
use std::time::Duration;
use std::cell::Cell;
use std::rc::Rc;

use crate::types::{AppState, AppConfig};

pub mod dashboard;
pub mod processes;
pub mod disks;
pub mod network;
pub mod containers;
pub mod services;
pub mod style;
pub mod system;
pub mod logs;
pub mod config;
pub mod global_stats;
pub mod cpu;
pub mod memory;
pub mod gpu;
pub mod sensors;
pub mod process_detail;

fn read_proc_details(pid: &str, proc_info: &crate::types::ProcessInfo) -> crate::types::DetailedProcessInfo {
    let proc_dir = format!("/proc/{}", pid);

    let command = std::fs::read_to_string(format!("{}/cmdline", proc_dir))
        .unwrap_or_default()
        .replace('\0', " ")
        .trim().to_string();

    let cwd = std::fs::read_link(format!("{}/cwd", proc_dir))
        .ok().map(|p| p.to_string_lossy().to_string());

    let environ: Vec<String> = std::fs::read_to_string(format!("{}/environ", proc_dir))
        .unwrap_or_default()
        .split('\0')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let status_content = std::fs::read_to_string(format!("{}/status", proc_dir)).unwrap_or_default();
    let mut threads: u32 = 0;
    let mut ppid: Option<String> = None;
    let mut vm_rss: u64 = proc_info.mem;
    let mut vm_size: u64 = 0;
    let mut status_str = "Running".to_string();

    for line in status_content.lines() {
        if line.starts_with("Threads:") {
            threads = line.split_whitespace().nth(1).and_then(|v| v.parse().ok()).unwrap_or(0);
        } else if line.starts_with("PPid:") {
            ppid = line.split_whitespace().nth(1).map(|s| s.to_string());
        } else if line.starts_with("VmRSS:") {
            vm_rss = line.split_whitespace().nth(1).and_then(|v| v.parse::<u64>().ok()).unwrap_or(0) * 1024;
        } else if line.starts_with("VmSize:") {
            vm_size = line.split_whitespace().nth(1).and_then(|v| v.parse::<u64>().ok()).unwrap_or(0) * 1024;
        } else if line.starts_with("State:") {
            let st = line.split_whitespace().nth(1).unwrap_or("?");
            status_str = match st {
                "S" => "Sleeping".to_string(),
                "R" => "Running".to_string(),
                "Z" => "Zombie".to_string(),
                "T" => "Stopped".to_string(),
                "D" => "Disk Sleep".to_string(),
                "I" => "Idle".to_string(),
                _ => st.to_string(),
            };
        }
    }

    let mut start_time = "N/A".to_string();
    if let Ok(stat) = std::fs::read_to_string(format!("{}/stat", proc_dir)) {
        let parts: Vec<&str> = stat.split_whitespace().collect();
        if parts.len() > 21 {
            if let Ok(ticks) = parts[21].parse::<u64>() {
                let uptime_secs = std::fs::read_to_string("/proc/uptime")
                    .unwrap_or_default()
                    .split_whitespace().next()
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0);
                let start_secs = ticks / 100;
                let elapsed = (uptime_secs as u64).saturating_sub(start_secs);
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default().as_secs();
                let started_at = now.saturating_sub(elapsed);
                start_time = chrono::DateTime::from_timestamp(started_at as i64, 0)
                    .map(|dt| dt.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| "N/A".to_string());
            }
        }
    }

    crate::types::DetailedProcessInfo {
        pid: proc_info.pid.clone(),
        name: proc_info.name.clone(),
        user: proc_info.user.clone(),
        status: status_str,
        cpu_usage: proc_info.cpu,
        memory_rss: vm_rss,
        memory_vms: vm_size,
        command: if command.is_empty() { proc_info.name.clone() } else { command },
        start_time,
        parent: ppid,
        environ,
        threads,
        file_descriptors: std::fs::read_dir(format!("{}/fd", proc_dir)).ok().map(|d| d.count() as u32),
        cwd,
    }
}

pub fn build_ui(app: &Application, state: Arc<Mutex<AppState>>, config: AppConfig) {
    style::apply_styles();

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(1024)
        .default_height(768)
        .build();
        
    let header = gtk::HeaderBar::new();
    header.set_show_close_button(true);
    header.set_title(Some("PULS-G3"));
    header.set_subtitle(Some("System Monitor & Admin Tool"));
    
    let version_lbl = gtk::Label::new(Some(&format!("v{}", env!("CARGO_PKG_VERSION"))));
    version_lbl.style_context().add_class("text-cyan");
    version_lbl.set_margin_start(6);
    header.pack_start(&version_lbl);

    let paused = Rc::new(Cell::new(false));
    let pause_btn = gtk::ToggleButton::with_label("Pause");
    pause_btn.style_context().add_class("suggested-action");
    let paused_clone = paused.clone();
    pause_btn.connect_toggled(move |btn| {
        paused_clone.set(btn.is_active());
        if btn.is_active() {
            btn.set_label("Resume");
        } else {
            btn.set_label("Pause");
        }
    });
    header.pack_end(&pause_btn);

    let dark_flag = style::dark_mode_flag();
    let theme_btn = gtk::Button::with_label(if dark_flag.get() { "Light" } else { "Dark" });
    theme_btn.connect_clicked(move |btn| {
        let is_dark = dark_flag.get();
        let new_dark = !is_dark;
        dark_flag.set(new_dark);
        style::apply_theme(new_dark);
        btn.set_label(if new_dark { "Light" } else { "Dark" });
    });
    header.pack_end(&theme_btn);

    window.set_titlebar(Some(&header));
    let vbox = GtkBox::new(Orientation::Vertical, 0);
    vbox.style_context().add_class("puls-content");
    let switcher = gtk::StackSwitcher::new();
    switcher.set_halign(gtk::Align::Center);
    let switcher_scroll = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    switcher_scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Never);
    switcher_scroll.set_min_content_height(45);
    switcher_scroll.add(&switcher);
    vbox.pack_start(&switcher_scroll, false, false, 2);
    let global_stats_widget = global_stats::build_global_stats();
    vbox.pack_start(&global_stats_widget, false, false, 5);
    let stack = gtk::Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::Crossfade);
    let dashboard_tab = dashboard::build_tab(state.clone());
    stack.add_titled(&dashboard_tab, "dashboard", "1:Dashboard");
    let processes_tab = processes::build_tab(state.clone());
    stack.add_titled(&processes_tab, "processes", "2:Processes");
    let cpu_tab = cpu::build_tab(state.clone());
    stack.add_titled(&cpu_tab, "cpu", "3:CPU");
    let memory_tab = memory::build_tab(state.clone());
    stack.add_titled(&memory_tab, "memory", "4:Memory");
    let disks_tab = disks::build_tab(state.clone());
    stack.add_titled(&disks_tab, "disks", "5:Disks");
    let network_tab = network::build_tab(state.clone());
    stack.add_titled(&network_tab, "network", "6:Network");
    let gpu_tab = gpu::build_tab(state.clone());
    stack.add_titled(&gpu_tab, "gpu", "7:GPU");
    let system_tab = system::build_tab(state.clone());
    stack.add_titled(&system_tab, "system", "8:System");
    let services_tab = services::build_tab(state.clone());
    stack.add_titled(&services_tab, "services", "9:Services");
    let logs_tab = logs::build_tab(state.clone());
    stack.add_titled(&logs_tab, "logs", "0:Logs");
    let config_tab = config::build_tab(state.clone());
    stack.add_titled(&config_tab, "config", "-:Config");
    let containers_tab = containers::build_tab(state.clone());
    stack.add_titled(&containers_tab, "containers", "=:Docker");
    let sensors_tab = sensors::build_tab(state.clone());
    stack.add_titled(&sensors_tab, "sensors", "+:Sensors");
    let process_detail_tab = process_detail::build_tab(state.clone());
    stack.add_titled(&process_detail_tab, "process_detail", "P:Details");
    switcher.set_stack(Some(&stack));

    let stack_clone = stack.clone();
    window.connect_key_press_event(move |_, key| {
        use gdk::keys::constants as keys;
        let pval = key.keyval();
        
        if pval == keys::Tab {
            let children = stack_clone.children();
            if let Some(current) = stack_clone.visible_child() {
                if let Some(pos) = children.iter().position(|w| w == &current) {
                    let next_pos = (pos + 1) % children.len();
                    stack_clone.set_visible_child(&children[next_pos]);
                }
            }
            return glib::Propagation::Stop;
        }

        let target_name = match pval {
            keys::_1 => Some("dashboard"),
            keys::_2 => Some("processes"),
            keys::_3 => Some("cpu"),
            keys::_4 => Some("memory"),
            keys::_5 => Some("disks"),
            keys::_6 => Some("network"),
            keys::_7 => Some("gpu"),
            keys::_8 => Some("system"),
            keys::_9 => Some("services"),
            keys::_0 => Some("logs"),
            keys::minus => Some("config"),
            keys::equal => Some("containers"),
            keys::plus => Some("sensors"),
            keys::p | keys::P => Some("process_detail"),
            _ => None,
        };

        if let Some(name) = target_name {
            stack_clone.set_visible_child_name(name);
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });

    vbox.pack_start(&stack, true, true, 0);

    {
        let state_sel = state.clone();
        let stack_sel = stack.clone();
        if let Some(tree) = dashboard::find_widget_by_name(
            &dashboard_tab.clone().downcast::<gtk::Container>().unwrap(),
            "dashboard_proc_tree"
        ).and_then(|w| w.downcast::<gtk::TreeView>().ok()) {
            tree.connect_row_activated(move |tv, _path, _col| {
                let sel = tv.selection();
                if let Some((model, iter)) = sel.selected() {
                    if let Ok(pid_str) = model.value(&iter, 0).get::<String>() {
                        let mut s = state_sel.lock();
                        if let Some(proc) = s.dynamic_data.processes.iter().find(|p| p.pid == pid_str) {
                            let details = read_proc_details(&pid_str, proc);
                            s.dynamic_data.detailed_process = Some(details);
                        }
                        drop(s);
                        stack_sel.set_visible_child_name("process_detail");
                    }
                }
            });
        }
    }

    {
        let state_sel = state.clone();
        let stack_sel = stack.clone();
        if let Some(tree) = dashboard::find_widget_by_name(
            &processes_tab.clone().downcast::<gtk::Container>().unwrap(),
            "process_tree"
        ).and_then(|w| w.downcast::<gtk::TreeView>().ok()) {
            tree.connect_row_activated(move |tv, _path, _col| {
                let sel = tv.selection();
                if let Some((model, iter)) = sel.selected() {
                    if let Ok(pid_str) = model.value(&iter, 0).get::<String>() {
                        let mut s = state_sel.lock();
                        if let Some(proc) = s.dynamic_data.processes.iter().find(|p| p.pid == pid_str) {
                            let details = read_proc_details(&pid_str, proc);
                            s.dynamic_data.detailed_process = Some(details);
                        }
                        drop(s);
                        stack_sel.set_visible_child_name("process_detail");
                    }
                }
            });
        }
    }

    window.add(&vbox);
    window.show_all();
    let global_stats_widget_clone = global_stats_widget.clone();

    let refresh_ms = config.ui_refresh_rate_ms() as u32;
    let paused_ref = paused.clone();
    let stack_ref = stack.clone();
    glib::timeout_add_local(Duration::from_millis(refresh_ms as u64), move || {
        if paused_ref.get() {
            return ControlFlow::Continue;
        }
        global_stats::update_global_stats(&global_stats_widget_clone, &state);
        let visible = stack_ref.visible_child_name().unwrap_or_default();
        match visible.as_str() {
            "dashboard" => dashboard::update_tab(&dashboard_tab, &state),
            "processes" => processes::update_tab(&processes_tab, &state),
            "cpu" => cpu::update_tab(&cpu_tab, &state),
            "memory" => memory::update_tab(&memory_tab, &state),
            "disks" => disks::update_tab(&disks_tab, &state),
            "network" => network::update_tab(&network_tab, &state),
            "gpu" => gpu::update_tab(&gpu_tab, &state),
            "system" => system::update_tab(&system_tab, &state),
            "services" => services::update_tab(&services_tab, &state),
            "logs" => logs::update_tab(&logs_tab, &state),
            "config" => config::update_tab(&config_tab, &state),
            "containers" => containers::update_tab(&containers_tab, &state),
            "sensors" => sensors::update_tab(&sensors_tab, &state),
            "process_detail" => process_detail::update_tab(&process_detail_tab, &state),
            _ => {},
        }

        ControlFlow::Continue
    });
}
