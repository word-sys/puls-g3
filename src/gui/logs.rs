use gtk::prelude::*;
use gtk::{Box, Orientation, TreeView, TreeViewColumn, CellRendererText, ListStore, Widget, ScrolledWindow, SearchEntry};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let container = Box::new(Orientation::Vertical, 5);
    
    let search = SearchEntry::new();
    search.set_margin_start(10);
    search.set_margin_end(10);
    search.set_margin_top(10);
    search.set_placeholder_text(Some("Filter logs..."));
    container.pack_start(&search, false, false, 0);

    let scrolled = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let store = ListStore::new(&[
        glib::Type::STRING, // Timestamp
        glib::Type::STRING, // Level
        glib::Type::STRING, // Service
        glib::Type::STRING, // Message
    ]);

    let tree = TreeView::with_model(&store);
    tree.set_widget_name("logs_tree");
    
    let cols = [
        ("Time", 0),
        ("Level", 1),
        ("Service", 2),
        ("Message", 3),
    ];
    
    for (title, col_id) in cols.iter() {
        let col = TreeViewColumn::new();
        col.set_title(title);
        let renderer = CellRendererText::new();
        gtk::prelude::TreeViewColumnExt::pack_start(&col, &renderer, true);
        gtk::prelude::TreeViewColumnExt::add_attribute(&col, &renderer, "text", *col_id);
        tree.append_column(&col);
    }
    
    scrolled.add(&tree);
    container.pack_start(&scrolled, true, true, 0);
    
    container.upcast::<Widget>()
}

pub fn update_tab(tab: &Widget, state: &Arc<Mutex<AppState>>) {
    let container = match tab.clone().downcast::<gtk::Container>() {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let tree = match crate::gui::dashboard::find_widget_by_name(&container, "logs_tree").and_then(|w| w.downcast::<TreeView>().ok()) {
        Some(t) => t,
        None => return,
    };
    
    let store = match tree.model().and_then(|m| m.downcast::<ListStore>().ok()) {
        Some(s) => s,
        None => return,
    };
    
    let s = state.lock();
    store.clear();
    
    for log in &s.logs {
        store.insert_with_values(None, &[
            (0, &log.timestamp), 
            (1, &log.level),
            (2, &log.service),
            (3, &log.message),
        ]);
    }
}
