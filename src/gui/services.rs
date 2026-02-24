use gtk::prelude::*;
use gtk::{Box, Orientation, TreeView, TreeViewColumn, CellRendererText, ListStore, Widget, ScrolledWindow, Button, Frame};
use std::sync::Arc;
use parking_lot::Mutex;
use glib::clone;
use crate::types::AppState;

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let container = Box::new(Orientation::Vertical, 5);
    container.set_border_width(10);

    let frame = Frame::new(Some(" Services "));

    let inner = Box::new(Orientation::Vertical, 5);
    inner.set_border_width(5);

    let action_box = Box::new(Orientation::Horizontal, 5);

    let start_btn = Button::with_label("▶ Start");
    let stop_btn = Button::with_label("■ Stop");
    let restart_btn = Button::with_label("⟳ Restart");
    let enable_btn = Button::with_label("✓ Enable");
    let disable_btn = Button::with_label("✗ Disable");

    start_btn.style_context().add_class("suggested-action");
    stop_btn.style_context().add_class("destructive-action");

    action_box.pack_start(&start_btn, false, false, 0);
    action_box.pack_start(&stop_btn, false, false, 0);
    action_box.pack_start(&restart_btn, false, false, 0);
    action_box.pack_start(&enable_btn, false, false, 0);
    action_box.pack_start(&disable_btn, false, false, 0);
    inner.pack_start(&action_box, false, false, 0);

    let status_lbl = gtk::Label::new(Some("Select a service and use the buttons above."));
    status_lbl.set_widget_name("service_status_lbl");
    status_lbl.set_halign(gtk::Align::Start);
    status_lbl.style_context().add_class("text-cyan");
    inner.pack_start(&status_lbl, false, false, 0);

    let scrolled = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let store = ListStore::new(&[
        glib::Type::STRING, // Name
        glib::Type::STRING, // Status
        glib::Type::STRING, // Enabled
        glib::Type::STRING, // Description
    ]);

    let tree = TreeView::with_model(&store);
    tree.set_widget_name("services_tree");

    for (title, id) in &[("Name", 0), ("Status", 1), ("Enabled", 2), ("Description", 3)] {
        let col = TreeViewColumn::new();
        col.set_title(title);
        col.set_resizable(true);
        let r = CellRendererText::new();
        TreeViewColumnExt::pack_start(&col, &r, true);
        TreeViewColumnExt::add_attribute(&col, &r, "text", *id);
        tree.append_column(&col);
    }

    scrolled.add(&tree);
    inner.pack_start(&scrolled, true, true, 0);
    frame.add(&inner);
    container.pack_start(&frame, true, true, 0);

    fn get_selected_service(tree: &TreeView) -> Option<String> {
        tree.selection().selected().and_then(|(model, iter)| {
            model.value(&iter, 0).get::<String>().ok()
        })
    }

    let tree_ref = tree.clone();
    let status_ref = status_lbl.clone();
    start_btn.connect_clicked(clone!(@strong tree_ref, @strong status_ref => move |_| {
        if let Some(name) = get_selected_service(&tree_ref) {
            let mgr = crate::system_service::SystemManager::new();
            match mgr.start_service(&name) {
                Ok(()) => status_ref.set_text(&format!("✓ Started {}", name)),
                Err(e) => status_ref.set_text(&format!("✗ Failed to start {}: {}", name, e)),
            }
        } else {
            status_ref.set_text("No service selected");
        }
    }));

    let tree_ref = tree.clone();
    let status_ref = status_lbl.clone();
    stop_btn.connect_clicked(clone!(@strong tree_ref, @strong status_ref => move |_| {
        if let Some(name) = get_selected_service(&tree_ref) {
            let mgr = crate::system_service::SystemManager::new();
            match mgr.stop_service(&name) {
                Ok(()) => status_ref.set_text(&format!("✓ Stopped {}", name)),
                Err(e) => status_ref.set_text(&format!("✗ Failed to stop {}: {}", name, e)),
            }
        } else {
            status_ref.set_text("No service selected");
        }
    }));

    let tree_ref = tree.clone();
    let status_ref = status_lbl.clone();
    restart_btn.connect_clicked(clone!(@strong tree_ref, @strong status_ref => move |_| {
        if let Some(name) = get_selected_service(&tree_ref) {
            let mgr = crate::system_service::SystemManager::new();
            match mgr.restart_service(&name) {
                Ok(()) => status_ref.set_text(&format!("✓ Restarted {}", name)),
                Err(e) => status_ref.set_text(&format!("✗ Failed to restart {}: {}", name, e)),
            }
        } else {
            status_ref.set_text("No service selected");
        }
    }));

    let tree_ref = tree.clone();
    let status_ref = status_lbl.clone();
    enable_btn.connect_clicked(clone!(@strong tree_ref, @strong status_ref => move |_| {
        if let Some(name) = get_selected_service(&tree_ref) {
            let mgr = crate::system_service::SystemManager::new();
            match mgr.enable_service(&name) {
                Ok(()) => status_ref.set_text(&format!("✓ Enabled {}", name)),
                Err(e) => status_ref.set_text(&format!("✗ Failed to enable {}: {}", name, e)),
            }
        } else {
            status_ref.set_text("No service selected");
        }
    }));

    let tree_ref = tree.clone();
    let status_ref = status_lbl.clone();
    disable_btn.connect_clicked(clone!(@strong tree_ref, @strong status_ref => move |_| {
        if let Some(name) = get_selected_service(&tree_ref) {
            let mgr = crate::system_service::SystemManager::new();
            match mgr.disable_service(&name) {
                Ok(()) => status_ref.set_text(&format!("✓ Disabled {}", name)),
                Err(e) => status_ref.set_text(&format!("✗ Failed to disable {}: {}", name, e)),
            }
        } else {
            status_ref.set_text("No service selected");
        }
    }));

    container.upcast::<Widget>()
}

pub fn update_tab(tab: &Widget, state: &Arc<Mutex<AppState>>) {
    let container = match tab.clone().downcast::<gtk::Container>() {
        Ok(c) => c,
        Err(_) => return,
    };

    let tree = match crate::gui::dashboard::find_widget_by_name(&container, "services_tree")
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

    for srv in &s.services {
        store.insert_with_values(None, &[
            (0, &srv.name),
            (1, &srv.status),
            (2, &(if srv.enabled { "Yes" } else { "No" }).to_string()),
            (3, &srv.description),
        ]);
    }
}
