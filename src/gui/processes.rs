use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, TreeView, TreeViewColumn, CellRendererText, ListStore, Widget, ScrolledWindow, Button, SearchEntry};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let container = GtkBox::new(Orientation::Vertical, 5);
    container.set_border_width(10);
    
    let header_box = GtkBox::new(Orientation::Horizontal, 10);
    let search = SearchEntry::new();
    search.set_hexpand(true);
    search.set_placeholder_text(Some("Filter..."));
    
    let kill_btn = Button::with_label("Kill Selected");
    let ctx = kill_btn.style_context();
    ctx.add_class("destructive-action");
    
    header_box.pack_start(&search, true, true, 0);
    header_box.pack_start(&kill_btn, false, false, 0);
    container.pack_start(&header_box, false, false, 0);
    
    let scrolled_window = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);

    let store = ListStore::new(&[
        glib::Type::STRING, // PID
        glib::Type::STRING, // Name
        glib::Type::STRING, // CPU
        glib::Type::STRING, // RAM
        glib::Type::STRING, // User
        glib::Type::STRING, // Disk Read
        glib::Type::STRING, // Disk Write
    ]);

    let tree = TreeView::with_model(&store);
    tree.set_widget_name("process_tree");
    
    let cols = [
        ("PID", 0),
        ("Name", 1),
        ("CPU", 2),
        ("RAM", 3),
        ("User", 4),
        ("Disk Read", 5),
        ("Disk Write", 6),
    ];
    
    for (title, col_id) in cols.iter() {
        let col = TreeViewColumn::new();
        col.set_title(title);
        let renderer = CellRendererText::new();
        gtk::prelude::TreeViewColumnExt::pack_start(&col, &renderer, true);
        gtk::prelude::TreeViewColumnExt::add_attribute(&col, &renderer, "text", *col_id);
        tree.append_column(&col);
    }
    
    scrolled_window.add(&tree);
    container.pack_start(&scrolled_window, true, true, 0);
    
    let tree_clone = tree.clone();
    kill_btn.connect_clicked(move |_| {
        let selection = tree_clone.selection();
        if let Some((model, iter)) = selection.selected() {
            if let Ok(pid_val) = model.value(&iter, 0).get::<String>() {
                let _ = std::process::Command::new("kill").args(["-9", &pid_val]).spawn();
            }
        }
    });

    container.upcast::<Widget>()
}

pub fn update_tab(tab: &Widget, state: &Arc<Mutex<AppState>>) {
    let container = match tab.clone().downcast::<gtk::Container>() {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let tree = match crate::gui::dashboard::find_widget_by_name(&container, "process_tree").and_then(|w| w.downcast::<TreeView>().ok()) {
        Some(t) => t,
        None => return,
    };
    
    let store = match tree.model().and_then(|m| m.downcast::<ListStore>().ok()) {
        Some(s) => s,
        None => return,
    };
    
    let s = state.lock();
    store.clear();
    
    for proc in &s.dynamic_data.processes {
        store.insert_with_values(None, &[(0, &proc.pid), (1, &proc.name), (2, &proc.cpu_display), (3, &proc.mem_display), (4, &proc.user), (5, &proc.disk_read), (6, &proc.disk_write)]);
    }
}
