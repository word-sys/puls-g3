use gtk::prelude::*;
use gtk::{Box, Orientation, Label, Widget, Grid, Label as GtkLabel};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::types::AppState;

pub fn build_tab(_state: Arc<Mutex<AppState>>) -> Widget {
    let container = Box::new(Orientation::Vertical, 10);
    container.set_border_width(15);
    
    let title = Label::new(Some("System Information"));
    title.set_halign(gtk::Align::Start);
    let ctx = title.style_context();
    ctx.add_class("stat-title");
    container.pack_start(&title, false, false, 0);

    let grid = Grid::builder()
        .row_spacing(15)
        .column_spacing(25)
        .build();
    grid.set_widget_name("system_info_grid");
    
    container.pack_start(&grid, true, true, 0);
    
    container.upcast::<Widget>()
}

pub fn update_tab(tab: &Widget, state: &Arc<Mutex<AppState>>) {
    let container = match tab.clone().downcast::<gtk::Container>() {
        Ok(c) => c,
        Err(_) => return,
    };
    
    let grid = match crate::gui::dashboard::find_widget_by_name(&container, "system_info_grid").and_then(|w| w.downcast::<Grid>().ok()) {
        Some(g) => g,
        None => return,
    };
    
    for child in grid.children() {
        grid.remove(&child);
    }
    
    let s = state.lock();
    for (i, (key, val)) in s.system_info.iter().enumerate() {
        let row = i as i32 / 2;
        let col_base = (i as i32 % 2) * 2;
        
        let key_lbl = GtkLabel::builder()
            .label(key)
            .halign(gtk::Align::Start)
            .build();
        let key_ctx = key_lbl.style_context();
        key_ctx.add_class("stat-title");
        
        let val_lbl = GtkLabel::builder()
            .label(val)
            .halign(gtk::Align::Start)
            .wrap(true)
            .build();
            
        grid.attach(&key_lbl, col_base, row, 1, 1);
        grid.attach(&val_lbl, col_base + 1, row, 1, 1);
    }
    
    grid.show_all();
}
