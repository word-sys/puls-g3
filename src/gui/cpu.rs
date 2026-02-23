use gtk::prelude::*;
use gtk::{Box, Orientation, Label, ProgressBar, Frame, Widget, Grid, ScrolledWindow};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;
use crate::utils::{format_frequency};

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let scrolled = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let container = Box::new(Orientation::Vertical, 10);
    container.set_border_width(10);

    // Box 1: CPU Information Frame
    let info_frame = Frame::new(Some(" CPU Information "));
    let info_box = Box::new(Orientation::Vertical, 5);
    info_box.set_border_width(8);
    let info_lbl = Label::new(Some("Model: \nCores: \nTemperature: \nLoad Average: "));
    info_lbl.set_widget_name("cpu_info_lbl");
    info_lbl.set_halign(gtk::Align::Start);
    info_lbl.style_context().add_class("text-magenta");
    info_box.pack_start(&info_lbl, false, false, 0);
    info_frame.add(&info_box);
    container.pack_start(&info_frame, false, false, 0);

    // Box 2: Detailed Core Usage Grid
    let grid_frame = Frame::new(Some(" Detailed Core Usage "));
    let grid_box = Box::new(Orientation::Vertical, 5);
    grid_box.set_border_width(8);
    
    let grid = Grid::builder()
        .row_spacing(5)
        .column_spacing(5)
        .column_homogeneous(true)
        .build();
    grid.set_widget_name("cores_grid");
    
    grid_box.pack_start(&grid, true, true, 0);
    grid_frame.add(&grid_box);
    container.pack_start(&grid_frame, true, true, 0);

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

    let info_lbl = crate::gui::dashboard::find_widget_by_name(&container.clone().upcast::<gtk::Container>(), "cpu_info_lbl")
        .and_then(|w| w.downcast::<Label>().ok());

    let cores_grid = crate::gui::dashboard::find_widget_by_name(&container.clone().upcast::<gtk::Container>(), "cores_grid")
        .and_then(|w| w.downcast::<Grid>().ok());

    let s = state.lock();
    let usage = &s.dynamic_data.global_usage;
    let cores = &s.dynamic_data.cores;
    let cpu_model = s.system_info.iter().find(|(k, _)| k == "CPU").map(|(_, v)| v.as_str()).unwrap_or("Unknown CPU");

    if let Some(lbl) = info_lbl {
        let temp_str = s.dynamic_data.temperatures.cpu_temp
            .map(|t| format!("{:.1}°C", t))
            .unwrap_or_else(|| "N/A".to_string());
            
        lbl.set_text(&format!(
            "Model: {}\nCores: {} Logical | Usage: {:.1}%\nTemperature: {}\nLoad Average: {:.2} {:.2} {:.2}",
            cpu_model,
            cores.len(),
            usage.cpu,
            temp_str,
            usage.load_average.0, usage.load_average.1, usage.load_average.2
        ));
    }

    if let Some(grid) = cores_grid {
        grid.forall(|child| {
            grid.remove(child);
        });

        let columns = 4;
        for (i, core) in cores.iter().enumerate() {
            let row = i as i32 / columns;
            let col = i as i32 % columns;

            let core_box = Box::new(Orientation::Vertical, 2);
            let core_frame = Frame::new(None);
            
            let temp_display = core.temp.map(|t| format!("{:.0}°C", t)).unwrap_or_default();
            let label_text = format!("C{} {} {:.1}% {}", i, format_frequency(core.freq), core.usage, temp_display);

            let lbl = Label::new(Some(&label_text));
            lbl.style_context().add_class("text-green");
            lbl.set_halign(gtk::Align::Center);

            let bar = ProgressBar::new();
            bar.set_fraction((core.usage / 100.0).clamp(0.0, 1.0) as f64);

            core_box.pack_start(&lbl, false, false, 0);
            core_box.pack_start(&bar, false, false, 0);
            core_frame.add(&core_box);
            
            grid.attach(&core_frame, col, row, 1, 1);
        }
        grid.show_all();
    }
}
