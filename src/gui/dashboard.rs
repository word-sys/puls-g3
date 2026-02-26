use gtk::prelude::*;
use gtk::{Box, Orientation, Label, Widget, Frame, ScrolledWindow,
          TreeView, TreeViewColumn, CellRendererText, ListStore};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;
use crate::utils::{format_size, format_uptime};

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let container = Box::new(Orientation::Vertical, 4);
    container.set_border_width(6);

    let status_frame = Frame::new(Some(" System Overview "));
    let status_lbl = Label::new(Some("Loading…"));
    status_lbl.set_widget_name("dashboard_status_lbl");
    status_lbl.set_line_wrap(true);
    status_lbl.set_xalign(0.0);
    status_lbl.style_context().add_class("text-green");
    status_frame.add(&status_lbl);
    container.pack_start(&status_frame, false, false, 0);

    let proc_frame = Frame::new(Some(" Processes "));
    let proc_scroll = ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    proc_scroll.set_vexpand(true);
    proc_scroll.set_hexpand(true);
    proc_scroll.set_min_content_height(250);

    let proc_store = ListStore::new(&[
        glib::Type::STRING, // PID
        glib::Type::STRING, // Name
        glib::Type::STRING, // User
        glib::Type::STRING, // CPU %
        glib::Type::STRING, // Memory
        glib::Type::STRING, // Disk Read
        glib::Type::STRING, // Disk Write
    ]);
    let proc_tree = TreeView::with_model(&proc_store);
    proc_tree.set_widget_name("dashboard_proc_tree");
    for (title, id) in &[("PID",0),("Name",1),("User",2),("CPU %",3),("Memory",4),("Disk Read",5),("Disk Write",6)] {
        let col = TreeViewColumn::new();
        col.set_title(title);
        col.set_resizable(true);
        let r = CellRendererText::new();
        TreeViewColumnExt::pack_start(&col, &r, true);
        TreeViewColumnExt::add_attribute(&col, &r, "text", *id);
        proc_tree.append_column(&col);
    }
    proc_scroll.add(&proc_tree);
    proc_frame.add(&proc_scroll);
    container.pack_start(&proc_frame, true, true, 0);

    let dock_frame = Frame::new(Some(" Containers "));
    let dock_lbl = Label::new(Some("No containers running"));
    dock_lbl.set_widget_name("dashboard_dock_lbl");
    dock_lbl.set_halign(gtk::Align::Center);
    dock_lbl.set_margin_start(6);
    dock_lbl.set_margin_end(6);
    dock_lbl.set_margin_top(6);
    dock_lbl.set_margin_bottom(6);
    dock_lbl.style_context().add_class("text-green");
    dock_frame.add(&dock_lbl);
    container.pack_start(&dock_frame, false, false, 0);

    container.upcast::<Widget>()
}

pub fn find_widget_by_name(container: &gtk::Container, name: &str) -> Option<Widget> {
    for child in container.children() {
        if child.widget_name() == name {
            return Some(child);
        }
        if let Ok(bin) = child.clone().downcast::<gtk::Container>() {
            if let Some(w) = find_widget_by_name(&bin, name) {
                return Some(w);
            }
        }
    }
    None
}

pub fn update_tab(tab: &Widget, state: &Arc<Mutex<AppState>>) {
    let container = match tab.clone().downcast::<gtk::Container>() {
        Ok(c) => c,
        Err(_) => return,
    };

    let s = state.lock();
    let usage = &s.dynamic_data.global_usage;

    if let Some(lbl) = find_widget_by_name(&container, "dashboard_status_lbl")
        .and_then(|w| w.downcast::<Label>().ok())
    {
        let cores = s.system_info.iter()
            .find(|(k,_)| k == "Cores")
            .and_then(|(_, v)| v.split_whitespace().next()?.parse::<usize>().ok())
            .unwrap_or(1);
        let mem_pct = if usage.mem_total > 0 { usage.mem_used as f64 / usage.mem_total as f64 * 100.0 } else { 0.0 };
        let swap_pct = if usage.swap_total > 0 { usage.swap_used as f64 / usage.swap_total as f64 * 100.0 } else { 0.0 };
        let temp = s.dynamic_data.temperatures.cpu_temp.map(|t| format!(" | {:.0}°C", t)).unwrap_or_default();
        lbl.set_text(&format!(
            "Status [IDLE/HEALTHY] | CPU: {:.0}% (Eff: GOOD){} | Load: {:.2}/core | Mem: {:.0}% ({}) | Swap: {:.0}% | Up: {} | Procs: {}",
            usage.cpu, temp,
            usage.load_average.0 / cores.max(1) as f64,
            mem_pct, format_size(usage.mem_total.saturating_sub(usage.mem_used)),
            swap_pct, format_uptime(usage.uptime), s.dynamic_data.processes.len()
        ));
    }

    if let Some(tree) = find_widget_by_name(&container, "dashboard_proc_tree")
        .and_then(|w| w.downcast::<TreeView>().ok())
    {
        if let Some(store) = tree.model().and_then(|m| m.downcast::<ListStore>().ok()) {
            store.clear();
            for p in &s.dynamic_data.processes {
                store.insert_with_values(None, &[
                    (0, &p.pid), (1, &p.name), (2, &p.user),
                    (3, &p.cpu_display), (4, &p.mem_display),
                    (5, &p.disk_read), (6, &p.disk_write),
                ]);
            }
        }
    }

    if let Some(lbl) = find_widget_by_name(&container, "dashboard_dock_lbl")
        .and_then(|w| w.downcast::<Label>().ok())
    {
        let c = &s.dynamic_data.containers;
        if c.is_empty() {
            lbl.set_text("No containers running");
        } else {
            let names: Vec<_> = c.iter().map(|x| format!("{} ({})", x.name, x.status)).collect();
            lbl.set_text(&format!("{} container(s): {}", c.len(), names.join(", ")));
        }
    }
}
