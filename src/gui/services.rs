use gtk::prelude::*;
use gtk::{Box, Orientation, TreeView, TreeViewColumn, CellRendererText, ListStore, Widget, ScrolledWindow, Button};
use std::sync::Arc;
use parking_lot::Mutex;
use glib::clone;
use crate::types::AppState;

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let container = Box::new(Orientation::Vertical, 5);
    container.set_border_width(10);
    
    let action_box = Box::new(Orientation::Horizontal, 5);
    let start_btn = Button::with_label("Start");
    let stop_btn = Button::with_label("Stop");
    let restart_btn = Button::with_label("Restart");
    
    start_btn.style_context().add_class("suggested-action");
    stop_btn.style_context().add_class("destructive-action");
    
    action_box.pack_start(&start_btn, false, false, 0);
    action_box.pack_start(&stop_btn, false, false, 0);
    action_box.pack_start(&restart_btn, false, false, 0);
    container.pack_start(&action_box, false, false, 0);
    
    let scrolled = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let store = ListStore::new(&[
        glib::Type::STRING, // Name
        glib::Type::STRING, // Status
        glib::Type::BOOL,   // Enabled
        glib::Type::STRING, // Description
    ]);

    let tree = TreeView::with_model(&store);
    tree.set_widget_name("services_tree");
    
    let cols = [
        ("Name", 0),
        ("Status", 1),
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
    
    let tree_clone = tree.clone();
    
    start_btn.connect_clicked(clone!(@strong tree_clone => move |_| {
        if let Some((model, iter)) = tree_clone.selection().selected() {
            if let Ok(name) = model.value(&iter, 0).get::<String>() {
                let sys_mgr = crate::system_service::SystemManager::new();
                let _ = sys_mgr.start_service(&name);
            }
        }
    }));
    
    let tree_clone2 = tree.clone();
    stop_btn.connect_clicked(clone!(@strong tree_clone2 => move |_| {
        if let Some((model, iter)) = tree_clone2.selection().selected() {
            if let Ok(name) = model.value(&iter, 0).get::<String>() {
                let sys_mgr = crate::system_service::SystemManager::new();
                let _ = sys_mgr.stop_service(&name);
            }
        }
    }));
    
    let tree_clone3 = tree.clone();
    restart_btn.connect_clicked(clone!(@strong tree_clone3 => move |_| {
        if let Some((model, iter)) = tree_clone3.selection().selected() {
            if let Ok(name) = model.value(&iter, 0).get::<String>() {
                let sys_mgr = crate::system_service::SystemManager::new();
                let _ = sys_mgr.restart_service(&name);
            }
        }
    }));
    
    container.upcast::<Widget>()
}

pub fn update_tab(tab: &Widget, state: &Arc<Mutex<AppState>>) {
    let container = match tab.clone().downcast::<gtk::Container>() {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let tree = match crate::gui::dashboard::find_widget_by_name(&container, "services_tree").and_then(|w| w.downcast::<TreeView>().ok()) {
        Some(t) => t,
        None => return,
    };
    
    let store = match tree.model().and_then(|m| m.downcast::<ListStore>().ok()) {
        Some(s) => s,
        None => return,
    };
    
    let s = state.lock();
    
    // Simplistic diff checking (only rebuild if counts map or something major changes)
    // To ease CPU burn we recreate. If it jumps around, we refine that.
    store.clear();
    
    for srv in &s.services {
        store.insert_with_values(None, &[
            (0, &srv.name), 
            (1, &srv.status),
            (2, &srv.enabled),
            (3, &srv.description),
        ]);
    }
}
