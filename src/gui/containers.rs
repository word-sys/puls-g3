use gtk::prelude::*;
use gtk::{Box, Orientation, TreeView, TreeViewColumn, CellRendererText, ListStore, Widget, ScrolledWindow};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let container = Box::new(Orientation::Vertical, 0);
    
    let scrolled = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let store = ListStore::new(&[
        glib::Type::STRING, // ID
        glib::Type::STRING, // Name
        glib::Type::STRING, // Image
        glib::Type::STRING, // Status
        glib::Type::STRING, // CPU
        glib::Type::STRING, // MEM
    ]);

    let tree = TreeView::with_model(&store);
    tree.set_widget_name("dockers_tree");
    
    let cols = [
        ("ID", 0),
        ("Name", 1),
        ("Image", 2),
        ("Status", 3),
        ("CPU", 4),
        ("MEM", 5),
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
    
    let tree = match crate::gui::dashboard::find_widget_by_name(&container, "dockers_tree").and_then(|w| w.downcast::<TreeView>().ok()) {
        Some(t) => t,
        None => return,
    };
    
    let store = match tree.model().and_then(|m| m.downcast::<ListStore>().ok()) {
        Some(s) => s,
        None => return,
    };
    
    let s = state.lock();
    store.clear();
    
    for c in &s.dynamic_data.containers {
        store.insert_with_values(None, &[
            (0, &c.id), 
            (1, &c.name), 
            (2, &c.image), 
            (3, &c.status), 
            (4, &c.cpu),
            (5, &c.mem),
        ]);
    }
}
