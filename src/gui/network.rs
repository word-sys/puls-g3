use gtk::prelude::*;
use gtk::{Box, Orientation, Widget, ScrolledWindow, Frame,
          TreeView, TreeViewColumn, CellRendererText, ListStore};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;
use crate::utils::{format_rate, format_size};

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let container = Box::new(Orientation::Vertical, 5);
    container.set_border_width(10);

    let frame = Frame::new(Some(" Network Interfaces "));

    let scrolled = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let store = ListStore::new(&[
        glib::Type::STRING, // Interface
        glib::Type::STRING, // Status
        glib::Type::STRING, // Download/s
        glib::Type::STRING, // Upload/s
        glib::Type::STRING, // Total Down
        glib::Type::STRING, // Total Up
        glib::Type::STRING, // Packets Rx/Tx
    ]);

    let tree = TreeView::with_model(&store);
    tree.set_widget_name("network_tree");

    for (title, id) in &[
        ("Interface", 0), ("Status", 1), ("Download/s", 2), ("Upload/s", 3),
        ("Total Down", 4), ("Total Up", 5), ("Packets Rx/Tx", 6),
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

    let tree = match crate::gui::dashboard::find_widget_by_name(&container, "network_tree")
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

    for net in &s.dynamic_data.networks {
        store.insert_with_values(None, &[
            (0, &net.name),
            (1, &(if net.is_up { "UP" } else { "DOWN" }).to_string()),
            (2, &format_rate(net.down_rate)),
            (3, &format_rate(net.up_rate)),
            (4, &format_size(net.total_down)),
            (5, &format_size(net.total_up)),
            (6, &format!("{}/{}", net.packets_rx, net.packets_tx)),
        ]);
    }
}
