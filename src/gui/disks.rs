use gtk::prelude::*;
use gtk::{Box, Orientation, Label, Widget, ScrolledWindow, Grid};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;
use crate::utils::{format_size, format_rate};

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let container = Box::new(Orientation::Vertical, 5);
    container.set_border_width(10);
    
    let scrolled = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);
    
    let list_box = Box::new(Orientation::Vertical, 5);
    list_box.set_widget_name("disks_container");
    
    scrolled.add(&list_box);
    container.pack_start(&scrolled, true, true, 0);

    container.upcast::<Widget>()
}

pub fn update_tab(tab: &Widget, state: &Arc<Mutex<AppState>>) {
    let container = match tab.clone().downcast::<gtk::Container>() {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let disks_container = match crate::gui::dashboard::find_widget_by_name(&container, "disks_container").and_then(|w| w.downcast::<Box>().ok()) {
        Some(w) => w,
        None => return,
    };
    
    // Clear old children
    for child in disks_container.children() {
        disks_container.remove(&child);
    }
    
    let s = state.lock();
    for disk in &s.dynamic_data.disks {
        let grid = Grid::builder()
            .row_spacing(5)
            .column_spacing(15)
            .margin(5)
            .build();
            
        let name_lbl = Label::new(Some(&format!("Disk: {} ({})", disk.name, disk.device)));
        name_lbl.set_halign(gtk::Align::Start);
        grid.attach(&name_lbl, 0, 0, 2, 1);
        
        let fs_lbl = Label::new(Some(&format!("FS: {}", disk.fs)));
        fs_lbl.set_halign(gtk::Align::Start);
        grid.attach(&fs_lbl, 0, 1, 1, 1);
        
        let usage_lbl = Label::new(Some(&format!("Usage: {} / {} ({} free)", format_size(disk.used), format_size(disk.total), format_size(disk.free))));
        usage_lbl.set_halign(gtk::Align::Start);
        grid.attach(&usage_lbl, 1, 1, 1, 1);
        
        let io_lbl = Label::new(Some(&format!("Read: {} | Write: {}", format_rate(disk.read_rate), format_rate(disk.write_rate))));
        io_lbl.set_halign(gtk::Align::Start);
        grid.attach(&io_lbl, 0, 2, 2, 1);
        
        disks_container.pack_start(&grid, false, false, 0);
        let sep = gtk::Separator::new(Orientation::Horizontal);
        disks_container.pack_start(&sep, false, false, 5);
    }
    
    disks_container.show_all();
}
