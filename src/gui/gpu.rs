use gtk::prelude::*;
use gtk::{Box, Orientation, Label, ProgressBar, Frame, Widget, ScrolledWindow};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;
use crate::utils::{format_size, format_frequency};

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let scrolled = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let container = Box::new(Orientation::Vertical, 10);
    container.set_border_width(10);
    container.set_widget_name("gpu_container");

    let no_gpu_lbl = Label::new(Some("No GPU detected or loaded."));
    no_gpu_lbl.set_widget_name("no_gpu_lbl");
    container.pack_start(&no_gpu_lbl, true, true, 0);

    scrolled.add(&container);
    scrolled.upcast::<Widget>()
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
    
    if let Ok(gpus) = &s.dynamic_data.gpus {
        if !gpus.is_empty() {
            container.forall(|child| container.remove(child));

            for (i, gpu) in gpus.iter().enumerate() {
                let frame = Frame::new(Some(&format!(" GPU {} - {} ({}) - {}°C ", i, gpu.name, gpu.brand, gpu.temperature)));
                let vbox = Box::new(Orientation::Vertical, 5);
                vbox.set_border_width(5);

                let util_lbl = Label::new(Some(&format!("Utilization: {}%", gpu.utilization)));
                util_lbl.set_halign(gtk::Align::Start);
                util_lbl.style_context().add_class("text-green");
                let util_bar = ProgressBar::new();
                util_bar.set_fraction((gpu.utilization as f64 / 100.0).clamp(0.0, 1.0));
                vbox.pack_start(&util_lbl, false, false, 0);
                vbox.pack_start(&util_bar, false, false, 0);

                let mem_percent = if gpu.memory_total > 0 { (gpu.memory_used as f64 / gpu.memory_total as f64) * 100.0 } else { 0.0 };
                let mem_lbl = Label::new(Some(&format!("Memory Usage: {:.1}%", mem_percent)));
                mem_lbl.set_halign(gtk::Align::Start);
                mem_lbl.style_context().add_class("text-cyan");
                let mem_bar = ProgressBar::new();
                mem_bar.set_fraction((mem_percent / 100.0).clamp(0.0, 1.0));
                vbox.pack_start(&mem_lbl, false, false, 0);
                vbox.pack_start(&mem_bar, false, false, 0);

                let details = format!(
                    "Memory: {} / {}\nPower: {:.2} W\nGraphics Clock: {}\nMemory Clock: {}\nMemory Temp: {}\nFan Speed: {}\nPCIe: {}",
                    format_size(gpu.memory_used), format_size(gpu.memory_total),
                    gpu.power_usage as f64 / 1000.0,
                    format_frequency(gpu.graphics_clock as u64),
                    format_frequency(gpu.memory_clock as u64),
                    gpu.memory_temperature.map(|t| format!("{}°C", t)).unwrap_or_else(|| "N/A".to_string()),
                    gpu.fan_speed.map(|f| format!("{} RPM", f)).unwrap_or_else(|| "N/A".to_string()),
                    if let (Some(gen), Some(width)) = (gpu.pci_link_gen, gpu.pci_link_width) { format!("Gen {} x{}", gen, width) } else { "N/A".to_string() }
                );

                let details_lbl = Label::new(Some(&details));
                details_lbl.set_halign(gtk::Align::Start);
                vbox.pack_start(&details_lbl, false, false, 0);

                frame.add(&vbox);
                container.pack_start(&frame, false, false, 0);
            }
            container.show_all();
        }
    }
}
