use gtk::prelude::*;
use gtk::{Box, Orientation, Label, ProgressBar, Widget, Frame, Grid, ScrolledWindow, FlowBox};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;
use crate::utils::{format_percentage, format_size, format_uptime};

pub fn build_tab(state: Arc<Mutex<AppState>>) -> Widget {
    let scrolled = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let container = Box::new(Orientation::Vertical, 10);
    container.set_border_width(10);
    
    // Status text block mimicking TUI 
    let status_frame = Frame::new(Some(" System Overview "));
    let status_box = Box::new(Orientation::Vertical, 5);
    status_box.set_border_width(8);
    
    let status_lbl = Label::new(Some("Status Loading... | CPU: ... | Load: ... | Mem: ... | Swap: ... | Up: ... | Procs: ..."));
    status_lbl.set_widget_name("dashboard_status_lbl");
    status_lbl.set_halign(gtk::Align::Start);
    status_lbl.style_context().add_class("text-green");
    status_box.pack_start(&status_lbl, false, false, 0);
    status_frame.add(&status_box);
    container.pack_start(&status_frame, false, false, 0);

    // Processes grid - re-using the processes logic or embedding it
    let proc_frame = Frame::new(Some(" Processes "));
    let proc_lbl = Label::new(Some("Use the Processes tab (Shortcut: 2) or advanced details."));
    proc_lbl.set_border_width(8);
    proc_frame.add(&proc_lbl);
    container.pack_start(&proc_frame, false, false, 0);

    // Containers grid - embedding containers preview
    let dock_frame = Frame::new(Some(" Containers "));
    let dock_lbl = Label::new(Some("Use the Containers tab (Shortcut: =) for details."));
    dock_lbl.set_border_width(8);
    dock_frame.add(&dock_lbl);
    container.pack_start(&dock_frame, false, false, 0);

    scrolled.add(&container);
    scrolled.upcast::<Widget>()
}

pub fn find_widget_by_name(container: &gtk::Container, name: &str) -> Option<Widget> {
    for child in container.children() {
        if child.widget_name() == name {
            return Some(child);
        }
        if let Ok(bin) = child.clone().downcast::<gtk::Container>() {
            if let Some(w) = find_widget_by_name(&bin, name) {
                return Some(w);
            }
        }
    }
    None
}

pub fn update_tab(tab: &Widget, state: &Arc<Mutex<AppState>>) {
    let scrolled = match tab.clone().downcast::<ScrolledWindow>() {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let container = match scrolled.child().and_then(|w| w.downcast::<gtk::Viewport>().ok()).and_then(|v| v.child()).and_then(|w| w.downcast::<Box>().ok()) {
        Some(b) => b,
        None => return,
    };
    
    let s = state.lock();
    let usage = &s.dynamic_data.global_usage;
    let system_info = &s.system_info;

    let cpu_cores = system_info.iter()
        .find(|(k, _)| k == "Cores")
        .and_then(|(_, v)| v.split_whitespace().next()?.parse::<usize>().ok())
        .unwrap_or(1);
    
    let eff = "Good"; // Simplified efficiency for GTK readout
    let mem_percent = if usage.mem_total > 0 { (usage.mem_used as f64 / usage.mem_total as f64) * 100.0 } else { 0.0 };
    let swap_percent = if usage.swap_total > 0 { (usage.swap_used as f64 / usage.swap_total as f64) * 100.0 } else { 0.0 };
    let load_per_core = usage.load_average.0 / (cpu_cores.max(1) as f64);
    
    let cpu_temp_str = s.dynamic_data.temperatures.cpu_temp.map(|t| format!(" | {:.0}°C", t)).unwrap_or_default();
    
    let status_text = format!(
        "Status Healthy | CPU: {:.0}% (Eff: {}){} | Load: {:.2}/core | Mem: {:.0}% ({}) | Swap: {:.0}% | Up: {} | Procs: {}",
        usage.cpu,
        eff,
        cpu_temp_str,
        load_per_core,
        mem_percent,
        format_size(usage.mem_total.saturating_sub(usage.mem_used)),
        swap_percent,
        format_uptime(usage.uptime),
        s.dynamic_data.processes.len()
    );

    if let Some(lbl) = find_widget_by_name(&container.upcast::<gtk::Container>(), "dashboard_status_lbl").and_then(|w| w.downcast::<Label>().ok()) {
        lbl.set_text(&status_text);
    }
}
