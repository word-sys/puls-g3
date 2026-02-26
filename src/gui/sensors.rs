use gtk::prelude::*;
use gtk::{Box, Orientation, TreeView, TreeViewColumn, CellRendererText, ListStore, Widget, ScrolledWindow};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let container = Box::new(Orientation::Vertical, 10);
    container.set_border_width(10);
    
    let scrolled_window = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);

    let store = ListStore::new(&[
        glib::Type::STRING, // Type
        glib::Type::STRING, // Label
        glib::Type::STRING, // Value
        glib::Type::STRING, // State
    ]);

    let tree = TreeView::with_model(&store);
    tree.set_widget_name("sensors_tree");

    let cols = [
        ("Type", 0),
        ("Label", 1),
        ("Value", 2),
        ("State / Details", 3),
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
    
    container.upcast::<Widget>()
}

pub fn update_tab(tab: &Widget, state: &Arc<Mutex<AppState>>) {
    let container = match tab.clone().downcast::<gtk::Container>() {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let tree = match crate::gui::dashboard::find_widget_by_name(&container, "sensors_tree").and_then(|w| w.downcast::<TreeView>().ok()) {
        Some(t) => t,
        None => return,
    };
    
    let store = match tree.model().and_then(|m| m.downcast::<ListStore>().ok()) {
        Some(s) => s,
        None => return,
    };
    
    let s = state.lock();
    store.clear();
    
    for sensor in &s.dynamic_data.sensors {
        let value_str = match sensor.sensor_type.as_str() {
            "temp" => format!("{:.1}°C", sensor.value),
            "fan" => format!("{:.0} RPM", sensor.value),
            "in" => format!("{:.3} V", sensor.value),
            "power" => format!("{:.2} W", sensor.value),
            "curr" => format!("{:.2} A", sensor.value),
            _ => format!("{:.2}", sensor.value),
        };
        
        let type_display = match sensor.sensor_type.as_str() {
            "temp" => "Temperature",
            "fan" => "Fan Speed",
            "in" => "Voltage",
            "power" => "Power",
            "curr" => "Current",
            _ => "Other",
        };

        store.insert_with_values(None, &[
            (0, &type_display), 
            (1, &sensor.label), 
            (2, &value_str), 
            (3, &"ACTIVE"),
        ]);
    }
}
