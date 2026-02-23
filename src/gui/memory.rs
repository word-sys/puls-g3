use gtk::prelude::*;
use gtk::{Box, Orientation, Label, ProgressBar, Frame, Widget};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;
use crate::utils::format_size;

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let container = Box::new(Orientation::Vertical, 10);
    container.set_border_width(10);

    // Box 1: RAM & Swap Gauges
    let gauges_box = Box::new(Orientation::Vertical, 10);
    
    // RAM
    let ram_frame = Frame::new(Some(" RAM Usage "));
    let ram_box = Box::new(Orientation::Vertical, 5);
    ram_box.set_border_width(8);
    let ram_lbl = Label::new(Some("0.0% (0 B / 0 B)"));
    ram_lbl.set_widget_name("ram_percent_lbl");
    ram_lbl.style_context().add_class("text-green");
    let ram_bar = ProgressBar::new();
    ram_bar.set_widget_name("ram_percent_bar");
    ram_box.pack_start(&ram_lbl, false, false, 0);
    ram_box.pack_start(&ram_bar, false, false, 0);
    ram_frame.add(&ram_box);
    gauges_box.pack_start(&ram_frame, false, false, 0);

    // Swap
    let swap_frame = Frame::new(Some(" Swap Usage "));
    let swap_box = Box::new(Orientation::Vertical, 5);
    swap_box.set_border_width(8);
    let swap_lbl = Label::new(Some("0.0% (0 B / 0 B)"));
    swap_lbl.set_widget_name("swap_percent_lbl");
    swap_lbl.style_context().add_class("text-cyan");
    let swap_bar = ProgressBar::new();
    swap_bar.set_widget_name("swap_percent_bar");
    swap_box.pack_start(&swap_lbl, false, false, 0);
    swap_box.pack_start(&swap_bar, false, false, 0);
    swap_frame.add(&swap_box);
    gauges_box.pack_start(&swap_frame, false, false, 0);

    container.pack_start(&gauges_box, false, false, 0);

    // Box 2: Details Grid
    let details_frame = Frame::new(Some(" Details "));
    let details_box = Box::new(Orientation::Vertical, 5);
    details_box.set_border_width(8);
    let details_lbl = Label::new(Some("Total Memory:\nUsed Memory:\nCached / Buffers:\nFree / Available:\nTemperature:"));
    details_lbl.set_widget_name("details_lbl");
    details_lbl.set_halign(gtk::Align::Start);
    details_box.pack_start(&details_lbl, false, false, 0);
    details_frame.add(&details_box);
    
    container.pack_start(&details_frame, true, true, 0);

    container.upcast::<Widget>()
}

pub fn update_tab(tab: &Widget, state: &Arc<Mutex<AppState>>) {
    let container = match tab.clone().downcast::<gtk::Container>() {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let ram_lbl = crate::gui::dashboard::find_widget_by_name(&container, "ram_percent_lbl").and_then(|w| w.downcast::<Label>().ok());
    let ram_bar = crate::gui::dashboard::find_widget_by_name(&container, "ram_percent_bar").and_then(|w| w.downcast::<ProgressBar>().ok());
    let swap_lbl = crate::gui::dashboard::find_widget_by_name(&container, "swap_percent_lbl").and_then(|w| w.downcast::<Label>().ok());
    let swap_bar = crate::gui::dashboard::find_widget_by_name(&container, "swap_percent_bar").and_then(|w| w.downcast::<ProgressBar>().ok());
    let details_lbl = crate::gui::dashboard::find_widget_by_name(&container, "details_lbl").and_then(|w| w.downcast::<Label>().ok());

    let s = state.lock();
    let usage = &s.dynamic_data.global_usage;

    let mem_percent = if usage.mem_total > 0 { (usage.mem_used as f64 / usage.mem_total as f64) * 100.0 } else { 0.0 };
    let swap_percent = if usage.swap_total > 0 { (usage.swap_used as f64 / usage.swap_total as f64) * 100.0 } else { 0.0 };

    if let Some(lbl) = ram_lbl {
        lbl.set_text(&format!("{:.1}% ({} / {})", mem_percent, format_size(usage.mem_used), format_size(usage.mem_total)));
    }
    if let Some(bar) = ram_bar {
        bar.set_fraction((mem_percent / 100.0).clamp(0.0, 1.0));
    }

    if let Some(lbl) = swap_lbl {
        lbl.set_text(&format!("{:.1}% ({} / {})", swap_percent, format_size(usage.swap_used), format_size(usage.swap_total)));
    }
    if let Some(bar) = swap_bar {
        bar.set_fraction((swap_percent / 100.0).clamp(0.0, 1.0));
    }

    if let Some(lbl) = details_lbl {
        let mem_temp_str = s.dynamic_data.sensors.iter()
            .find(|sensor| {
                let lower = sensor.label.to_lowercase();
                lower.contains("dimm") || lower.contains("dram") || lower.contains("memory") || lower.contains("sodimm")
            })
            .map(|sensor| format!("{:.1}°C", sensor.temp))
            .unwrap_or_else(|| "N/A".to_string());

        let total_mem = format_size(usage.mem_used + (usage.mem_total.saturating_sub(usage.mem_used)));
        let used_mem = format_size(usage.mem_used);
        let cached_mem = format_size(usage.mem_cached);
        let free_mem = format_size(usage.mem_total.saturating_sub(usage.mem_used));

        lbl.set_text(&format!(
            "Total Memory: {}\nUsed Memory: {}\nCached / Buffers: {}\nFree / Available: {}\nTemperature: {}",
            total_mem, used_mem, cached_mem, free_mem, mem_temp_str
        ));
    }
}
