use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Orientation};
use gtk::gdk;
use std::sync::Arc;
use parking_lot::Mutex;
use glib::ControlFlow;
use std::time::Duration;

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

pub fn build_ui(app: &Application, state: Arc<Mutex<AppState>>, config: AppConfig) {
    // Load custom CSS
    style::apply_styles();

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(1024)
        .default_height(768)
        .build();
        
    let header = gtk::HeaderBar::new();
    header.set_show_close_button(true);
    header.set_title(Some("PULS"));
    header.set_subtitle(Some("System Monitor"));
    window.set_titlebar(Some(&header));

    let vbox = GtkBox::new(Orientation::Vertical, 0);
    // ----------------------------------------------------------------
    // Setup tabs
    // ----------------------------------------------------------------
    
    // 1. Stack Switcher (Tabs)
    let switcher = gtk::StackSwitcher::new();
    switcher.set_halign(gtk::Align::Center);
    vbox.pack_start(&switcher, false, false, 5);

    // 2. Global Stats Frame
    let global_stats_widget = global_stats::build_global_stats();
    vbox.pack_start(&global_stats_widget, false, false, 5);

    // 3. Stack (Content)
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

    switcher.set_stack(Some(&stack));

    // Global Keyboard Shortcuts
    let stack_clone = stack.clone();
    window.connect_key_press_event(move |_, key| {
        use gdk::keys::constants as keys;
        let pval = key.keyval();
        
        if pval == keys::Tab {
            // Cycle forward
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

    window.add(&vbox);
    window.show_all();

    // ----------------------------------------------------------------
    // Periodic UI Refresh
    // ----------------------------------------------------------------
    let global_stats_widget_clone = global_stats_widget.clone();

    let refresh_ms = config.ui_refresh_rate_ms() as u32;
    glib::timeout_add_local(Duration::from_millis(refresh_ms as u64), move || {
        // Here we trigger updates to internal UI tabs by reading the `state`
        dashboard::update_tab(&dashboard_tab, &state);
        processes::update_tab(&processes_tab, &state);
        cpu::update_tab(&cpu_tab, &state);
        memory::update_tab(&memory_tab, &state);
        disks::update_tab(&disks_tab, &state);
        network::update_tab(&network_tab, &state);
        gpu::update_tab(&gpu_tab, &state);
        system::update_tab(&system_tab, &state);
        services::update_tab(&services_tab, &state);
        logs::update_tab(&logs_tab, &state);
        config::update_tab(&config_tab, &state);
        containers::update_tab(&containers_tab, &state);
        sensors::update_tab(&sensors_tab, &state);
        global_stats::update_global_stats(&global_stats_widget_clone, &state);

        ControlFlow::Continue
    });
}
