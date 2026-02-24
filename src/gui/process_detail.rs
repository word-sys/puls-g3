use gtk::prelude::*;
use gtk::{Box, Orientation, Label, Widget, Frame};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;
use crate::utils::format_size;

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let container = Box::new(Orientation::Horizontal, 10);
    container.set_border_width(10);
    container.set_vexpand(true);
    container.set_hexpand(true);

    let info_frame = Frame::new(Some(" Process Information "));
    let info_box = Box::new(Orientation::Vertical, 0);
    info_box.set_border_width(8);
    let info_lbl = Label::new(Some("Double-click a process in Dashboard or Processes tab to view details."));
    info_lbl.set_widget_name("proc_detail_info_lbl");
    info_lbl.set_halign(gtk::Align::Start);
    info_lbl.set_valign(gtk::Align::Start);
    info_lbl.set_line_wrap(true);
    info_lbl.set_vexpand(true);
    info_lbl.set_hexpand(true);
    info_lbl.set_use_markup(true);
    info_box.pack_start(&info_lbl, true, true, 0);
    info_frame.add(&info_box);
    container.pack_start(&info_frame, true, true, 0);

    let cmd_frame = Frame::new(Some(" Command & Environment "));
    let cmd_scroll = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    cmd_scroll.set_vexpand(true);
    let cmd_lbl = Label::new(Some("No process selected"));
    cmd_lbl.set_widget_name("proc_detail_cmd_lbl");
    cmd_lbl.set_halign(gtk::Align::Start);
    cmd_lbl.set_valign(gtk::Align::Start);
    cmd_lbl.set_line_wrap(true);
    cmd_lbl.set_selectable(true);
    cmd_lbl.set_use_markup(true);
    cmd_lbl.set_margin_start(8);
    cmd_lbl.set_margin_end(8);
    cmd_lbl.set_margin_top(8);
    cmd_lbl.set_margin_bottom(8);
    cmd_scroll.add(&cmd_lbl);
    cmd_frame.add(&cmd_scroll);
    container.pack_start(&cmd_frame, true, true, 0);

    container.upcast::<Widget>()
}

pub fn update_tab(tab: &Widget, state: &Arc<Mutex<AppState>>) {
    let container = match tab.clone().downcast::<gtk::Container>() {
        Ok(c) => c,
        Err(_) => return,
    };

    let info_lbl = crate::gui::dashboard::find_widget_by_name(&container, "proc_detail_info_lbl")
        .and_then(|w| w.downcast::<Label>().ok());
    let cmd_lbl = crate::gui::dashboard::find_widget_by_name(&container, "proc_detail_cmd_lbl")
        .and_then(|w| w.downcast::<Label>().ok());

    let s = state.lock();

    if let Some(ref process) = s.dynamic_data.detailed_process {
        if let Some(lbl) = info_lbl {
            let cwd_str = process.cwd.as_deref().unwrap_or("N/A");
            let parent_str = process.parent.as_deref().unwrap_or("N/A");
            let fd_str = process.file_descriptors.map(|f| f.to_string()).unwrap_or_else(|| "N/A".to_string());
            lbl.set_markup(&format!(
                "<span foreground='#00ffff' weight='bold'>PID:</span> {}\n\
                 <span foreground='#00ffff' weight='bold'>Name:</span> {}\n\
                 <span foreground='#00ffff' weight='bold'>User:</span> {}\n\
                 <span foreground='#00ffff' weight='bold'>Status:</span> {}\n\
                 <span foreground='#00ffff' weight='bold'>Parent PID:</span> {}\n\
                 <span foreground='#00ffff' weight='bold'>Started:</span> {}\n\
                 <span foreground='#00ffff' weight='bold'>CPU Usage:</span> {:.2}%\n\
                 <span foreground='#00ffff' weight='bold'>Memory (RSS):</span> {}\n\
                 <span foreground='#00ffff' weight='bold'>Memory (VMS):</span> {}\n\
                 <span foreground='#00ffff' weight='bold'>Threads:</span> {}\n\
                 <span foreground='#00ffff' weight='bold'>File Descriptors:</span> {}\n\
                 <span foreground='#00ffff' weight='bold'>Working Dir:</span> {}",
                glib::markup_escape_text(&process.pid),
                glib::markup_escape_text(&process.name),
                glib::markup_escape_text(&process.user),
                glib::markup_escape_text(&process.status),
                glib::markup_escape_text(parent_str),
                glib::markup_escape_text(&process.start_time),
                process.cpu_usage,
                format_size(process.memory_rss),
                format_size(process.memory_vms),
                process.threads,
                fd_str,
                glib::markup_escape_text(cwd_str),
            ));
        }

        if let Some(lbl) = cmd_lbl {
            let mut text = format!(
                "<span foreground='#00ff00' weight='bold'>Command:</span>\n\n{}\n\n\
                 <span foreground='#00ff00' weight='bold'>Environment Variables ({}):</span>\n",
                glib::markup_escape_text(&process.command),
                process.environ.len()
            );
            for (i, env) in process.environ.iter().enumerate() {
                if i >= 50 {
                    text.push_str("\n<i>... (truncated, showing 50 of ");
                    text.push_str(&process.environ.len().to_string());
                    text.push_str(")</i>");
                    break;
                }
                text.push('\n');
                text.push_str(&glib::markup_escape_text(env));
            }
            lbl.set_markup(&text);
        }
    } else {
        if let Some(lbl) = info_lbl {
            lbl.set_text("Double-click a process in Dashboard or Processes tab to view details.");
        }
        if let Some(lbl) = cmd_lbl {
            lbl.set_text("No process selected");
        }
    }
}
