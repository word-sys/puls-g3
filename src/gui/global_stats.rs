use gtk::prelude::*;
use gtk::{Box, Orientation, Label, ProgressBar, Frame, Widget};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;
use crate::utils::{format_size, format_rate};

pub fn build_global_stats() -> Widget {
    let container = Box::new(Orientation::Horizontal, 5);
    container.set_border_width(5);

    // CPU Frame
    let cpu_frame = Frame::new(Some("CPU"));
    let cpu_box = Box::new(Orientation::Horizontal, 5);
    cpu_box.set_border_width(2);
    let cpu_bar = ProgressBar::new();
    cpu_bar.set_widget_name("global_cpu_bar");
    cpu_bar.set_size_request(60, -1);
    let cpu_lbl = Label::new(Some("0.0% | 0°C | Load: 0.0"));
    cpu_lbl.set_widget_name("global_cpu_lbl");
    cpu_lbl.style_context().add_class("text-green");
    cpu_box.pack_start(&cpu_bar, false, false, 0);
    cpu_box.pack_start(&cpu_lbl, false, false, 0);
    cpu_frame.add(&cpu_box);
    container.pack_start(&cpu_frame, true, true, 0);

    // Memory Frame
    let mem_frame = Frame::new(Some("Memory"));
    let mem_box = Box::new(Orientation::Horizontal, 5);
    mem_box.set_border_width(2);
    let mem_bar = ProgressBar::new();
    mem_bar.set_widget_name("global_mem_bar");
    mem_bar.set_size_request(60, -1);
    let mem_lbl = Label::new(Some("0 B"));
    mem_lbl.set_widget_name("global_mem_lbl");
    mem_lbl.style_context().add_class("text-green");
    mem_box.pack_start(&mem_bar, false, false, 0);
    mem_box.pack_start(&mem_lbl, false, false, 0);
    mem_frame.add(&mem_box);
    container.pack_start(&mem_frame, true, true, 0);

    // GPU Frame
    let gpu_frame = Frame::new(Some("GPU"));
    let gpu_lbl = Label::new(Some("0%"));
    gpu_lbl.set_widget_name("global_gpu_lbl");
    gpu_lbl.style_context().add_class("text-green");
    gpu_lbl.set_margin_start(10);
    gpu_lbl.set_margin_end(10);
    gpu_frame.add(&gpu_lbl);
    container.pack_start(&gpu_frame, true, true, 0);

    // Network I/O Frame
    let net_frame = Frame::new(Some("Network I/O"));
    let net_lbl = Label::new(Some("▼0 B/s ▲0 B/s"));
    net_lbl.set_widget_name("global_net_lbl");
    net_lbl.style_context().add_class("text-magenta");
    net_lbl.set_margin_start(10);
    net_lbl.set_margin_end(10);
    net_frame.add(&net_lbl);
    container.pack_start(&net_frame, true, true, 0);

    // Disk I/O Frame
    let disk_frame = Frame::new(Some("Disk I/O"));
    let disk_lbl = Label::new(Some("R:0 B/s W:0 B/s"));
    disk_lbl.set_widget_name("global_disk_lbl");
    disk_lbl.style_context().add_class("text-orange");
    disk_lbl.set_margin_start(10);
    disk_lbl.set_margin_end(10);
    disk_frame.add(&disk_lbl);
    container.pack_start(&disk_frame, true, true, 0);

    container.upcast::<Widget>()
}

pub fn update_global_stats(container: &Widget, state: &Arc<Mutex<AppState>>) {
    let cont = match container.clone().downcast::<gtk::Container>() {
        Ok(c) => c,
        Err(_) => return,
    };

    let cpu_lbl = crate::gui::dashboard::find_widget_by_name(&cont, "global_cpu_lbl").and_then(|w| w.downcast::<Label>().ok());
    let cpu_bar = crate::gui::dashboard::find_widget_by_name(&cont, "global_cpu_bar").and_then(|w| w.downcast::<ProgressBar>().ok());
    
    let mem_lbl = crate::gui::dashboard::find_widget_by_name(&cont, "global_mem_lbl").and_then(|w| w.downcast::<Label>().ok());
    let mem_bar = crate::gui::dashboard::find_widget_by_name(&cont, "global_mem_bar").and_then(|w| w.downcast::<ProgressBar>().ok());

    let gpu_lbl = crate::gui::dashboard::find_widget_by_name(&cont, "global_gpu_lbl").and_then(|w| w.downcast::<Label>().ok());
    let net_lbl = crate::gui::dashboard::find_widget_by_name(&cont, "global_net_lbl").and_then(|w| w.downcast::<Label>().ok());
    let disk_lbl = crate::gui::dashboard::find_widget_by_name(&cont, "global_disk_lbl").and_then(|w| w.downcast::<Label>().ok());

    let s = state.lock();
    let usage = &s.dynamic_data.global_usage;

    if let Some(lbl) = cpu_lbl {
        let cpu_temp = s.dynamic_data.temperatures.cpu_temp.unwrap_or(0.0);
        lbl.set_text(&format!("{:.1}% | {:.0}°C | Load: {:.1}", usage.cpu, cpu_temp, usage.load_average.0));
    }
    if let Some(bar) = cpu_bar {
        bar.set_fraction((usage.cpu / 100.0).clamp(0.0, 1.0) as f64);
    }

    if let Some(lbl) = mem_lbl {
        lbl.set_text(&format!("{} ({}%)", format_size(usage.mem_used), (usage.mem_used as f64 / usage.mem_total.max(1) as f64 * 100.0) as u32));
    }
    if let Some(bar) = mem_bar {
        bar.set_fraction((usage.mem_used as f64 / usage.mem_total.max(1) as f64).clamp(0.0, 1.0));
    }

    if let Some(lbl) = gpu_lbl {
        lbl.set_text(&format!("{}%", usage.gpu_util.unwrap_or(0)));
    }

    if let Some(lbl) = net_lbl {
        lbl.set_text(&format!("▼{} ▲{}", format_rate(usage.net_down), format_rate(usage.net_up)));
    }

    if let Some(lbl) = disk_lbl {
        lbl.set_text(&format!("R:{} W:{}", format_rate(usage.disk_read), format_rate(usage.disk_write)));
    }
}
