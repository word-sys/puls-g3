use gtk::prelude::*;
use gtk::{Box, Orientation, Widget, ScrolledWindow, Frame,
          TreeView, TreeViewColumn, CellRendererText, ListStore};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;
use crate::utils::format_size;

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let container = Box::new(Orientation::Vertical, 5);
    container.set_border_width(10);

    let frame = Frame::new(Some(" Disk Usage "));

    let scrolled = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let store = ListStore::new(&[
        glib::Type::STRING, // Mount
        glib::Type::STRING, // Device
        glib::Type::STRING, // FS
        glib::Type::STRING, // Total
        glib::Type::STRING, // Used
        glib::Type::STRING, // Free
        glib::Type::STRING, // Use%
        glib::Type::STRING, // Temp
        glib::Type::STRING, // Health
        glib::Type::STRING, // Cycles
        glib::Type::STRING, // Type
    ]);

    let tree = TreeView::with_model(&store);
    tree.set_widget_name("disks_tree");

    for (title, id) in &[
        ("Mount", 0), ("Device", 1), ("FS", 2), ("Total", 3),
        ("Used", 4), ("Free", 5), ("Use%", 6), ("Temp", 7),
        ("Health", 8), ("Cycles", 9), ("Type", 10),
    ] {
        let col = TreeViewColumn::new();
        col.set_title(title);
        col.set_resizable(true);
        let r = CellRendererText::new();
        TreeViewColumnExt::pack_start(&col, &r, true);
        TreeViewColumnExt::add_attribute(&col, &r, "text", *id);
        tree.append_column(&col);
    }

    scrolled.add(&tree);
    frame.add(&scrolled);
    container.pack_start(&frame, true, true, 0);

    container.upcast::<Widget>()
}

pub fn update_tab(tab: &Widget, state: &Arc<Mutex<AppState>>) {
    let container = match tab.clone().downcast::<gtk::Container>() {
        Ok(c) => c,
        Err(_) => return,
    };

    let tree = match crate::gui::dashboard::find_widget_by_name(&container, "disks_tree")
        .and_then(|w| w.downcast::<TreeView>().ok())
    {
        Some(t) => t,
        None => return,
    };

    let store = match tree.model().and_then(|m| m.downcast::<ListStore>().ok()) {
        Some(s) => s,
        None => return,
    };

    let s = state.lock();
    store.clear();

    for disk in &s.dynamic_data.disks {
        let use_pct = if disk.total > 0 {
            format!("{:.1}%", disk.used as f64 / disk.total as f64 * 100.0)
        } else {
            "-".to_string()
        };
        let temp_str = disk.temp.map(|t| format!("{:.0}°C", t)).unwrap_or_else(|| "-".to_string());
        let health_str = disk.health_pct.map(|h| format!("{}%", h)).unwrap_or_else(|| "-".to_string());
        let cycles_str = disk.power_cycles.map(|c| c.to_string()).unwrap_or_else(|| "-".to_string());

        store.insert_with_values(None, &[
            (0, &disk.name),
            (1, &disk.device),
            (2, &disk.fs),
            (3, &format_size(disk.total)),
            (4, &format_size(disk.used)),
            (5, &format_size(disk.free)),
            (6, &use_pct),
            (7, &temp_str),
            (8, &health_str),
            (9, &cycles_str),
            (10, &disk.is_ssd.map(|ssd| if ssd { "SSD".to_string() } else { "HDD".to_string() }).unwrap_or_else(|| "-".to_string())),
        ]);
    }
}
