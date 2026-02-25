use gtk::prelude::*;
use gtk::CssProvider;
use gtk::gdk::Screen;
use std::cell::Cell;
use std::rc::Rc;

thread_local! {
    static PROVIDER: std::cell::RefCell<Option<CssProvider>> = std::cell::RefCell::new(None);
}

pub fn is_system_dark() -> bool {
    if let Some(settings) = gtk::Settings::default() {
        let theme_name = settings.gtk_theme_name()
            .map(|s| s.to_string())
            .unwrap_or_default()
            .to_lowercase();
        theme_name.contains("dark") || theme_name.contains("noir")
    } else {
        false
    }
}

pub fn dark_mode_flag() -> Rc<Cell<bool>> {
    thread_local! {
        static FLAG: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    }
    FLAG.with(|f| f.clone())
}

pub fn apply_styles() {
    let is_dark = is_system_dark();
    dark_mode_flag().set(is_dark);
    apply_theme(is_dark);
}

pub fn apply_theme(dark: bool) {
    let provider = CssProvider::new();

    let css = if dark {
        dark_theme()
    } else {
        light_theme()
    };

    if let Err(err) = provider.load_from_data(css.as_bytes()) {
        eprintln!("Failed to load CSS: {}", err);
        return;
    }

    if let Some(screen) = Screen::default() {
        PROVIDER.with(|p| {
            if let Some(old) = p.borrow().as_ref() {
                gtk::StyleContext::remove_provider_for_screen(&screen, old);
            }
        });

        gtk::StyleContext::add_provider_for_screen(
            &screen,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        PROVIDER.with(|p| {
            *p.borrow_mut() = Some(provider);
        });
    }
}

fn dark_theme() -> String {
    r#"
        .puls-content {
            background-color: #0d0d0d;
        }

        .puls-content frame > border {
            border: 1px solid #333333;
            border-radius: 4px;
        }

        .puls-content frame label {
            color: #cccccc;
            font-weight: bold;
            padding: 0 6px;
        }

        stackswitcher button {
            background-color: #111111;
            color: #999999;
            border: 1px solid #2a2a2a;
            padding: 4px 10px;
            font-size: 12px;
            font-weight: normal;
        }

        stackswitcher button:checked {
            background-color: #1a1a1a;
            color: #00ffff;
            font-weight: bold;
            border-color: #00ffff;
        }

        stackswitcher button:hover {
            background-color: #1a1a1a;
            color: #ffffff;
        }

        .puls-content treeview {
            background-color: #111111;
            color: #e0e0e0;
            font-size: 13px;
        }

        .puls-content treeview:hover {
            background-color: #1a1a1a;
        }

        .puls-content treeview:selected {
            background-color: #1a2a3a;
            color: #00ffff;
        }

        .puls-content treeview header button {
            background-color: #0a0a0a;
            color: #00ff88;
            font-weight: bold;
            padding: 4px 8px;
            border: none;
            border-bottom: 2px solid #00ff88;
            font-size: 13px;
        }

        .puls-content progressbar {
            min-height: 16px;
        }

        .puls-content progressbar trough {
            background-color: #1a1a1a;
            border: 1px solid #333333;
        }

        .puls-content progressbar progress {
            background-color: #00ff88;
            background-image: none;
        }

        .puls-content label {
            font-size: 13px;
            color: #e0e0e0;
        }

        .puls-content separator {
            background-color: #333333;
        }

        .puls-content entry {
            background-color: #111111;
            color: #00ffff;
            border: 1px solid #333333;
            padding: 4px 8px;
            font-size: 13px;
        }

        .puls-content entry:focus {
            border-color: #00ffff;
        }

        .puls-content button {
            background-color: #151515;
            color: #e0e0e0;
            border: 1px solid #333333;
            padding: 4px 12px;
            font-size: 13px;
        }

        .puls-content button:hover {
            background-color: #1a1a1a;
            color: #ffffff;
        }

        .puls-content button.suggested-action {
            background-color: #0a2a1a;
            color: #00ff88;
            border-color: #00ff88;
        }

        .puls-content button.destructive-action {
            background-color: #2a0a0a;
            color: #ff4444;
            border-color: #ff4444;
        }

        .puls-content scrollbar slider {
            background-color: #333333;
            min-width: 8px;
            min-height: 8px;
        }

        .puls-content scrollbar slider:hover {
            background-color: #555555;
        }

        .text-green { color: #00ff88; font-weight: bold; }
        .text-cyan { color: #00ffff; font-weight: bold; }
        .text-magenta { color: #ff66ff; font-weight: bold; }
        .text-orange { color: #ffaa44; font-weight: bold; }
        .text-white { color: #ffffff; font-weight: bold; }
        .text-red { color: #ff4444; font-weight: bold; }
    "#.to_string()
}

fn light_theme() -> String {
    r#"
        .puls-content {
            background-color: #f5f5f5;
        }

        .puls-content frame > border {
            border: 1px solid #cccccc;
            border-radius: 4px;
        }

        .puls-content frame label {
            color: #333333;
            font-weight: bold;
            padding: 0 6px;
        }

        stackswitcher button {
            background-color: #e8e8e8;
            color: #555555;
            border: 1px solid #cccccc;
            padding: 4px 10px;
            font-size: 12px;
            font-weight: normal;
        }

        stackswitcher button:checked {
            background-color: #ffffff;
            color: #0066cc;
            font-weight: bold;
            border-color: #0066cc;
        }

        stackswitcher button:hover {
            background-color: #f0f0f0;
            color: #333333;
        }

        .puls-content treeview {
            background-color: #ffffff;
            color: #333333;
            font-size: 13px;
        }

        .puls-content treeview:hover {
            background-color: #f0f0f0;
        }

        .puls-content treeview:selected {
            background-color: #cce5ff;
            color: #003366;
        }

        .puls-content treeview header button {
            background-color: #f0f0f0;
            color: #006644;
            font-weight: bold;
            padding: 4px 8px;
            border: none;
            border-bottom: 2px solid #006644;
            font-size: 13px;
        }

        .puls-content progressbar {
            min-height: 16px;
        }

        .puls-content progressbar trough {
            background-color: #e0e0e0;
            border: 1px solid #cccccc;
        }

        .puls-content progressbar progress {
            background-color: #00aa55;
            background-image: none;
        }

        .puls-content label {
            font-size: 13px;
            color: #333333;
        }

        .puls-content separator {
            background-color: #cccccc;
        }

        .puls-content entry {
            background-color: #ffffff;
            color: #006688;
            border: 1px solid #cccccc;
            padding: 4px 8px;
            font-size: 13px;
        }

        .puls-content entry:focus {
            border-color: #0066cc;
        }

        .puls-content button {
            background-color: #e8e8e8;
            color: #333333;
            border: 1px solid #cccccc;
            padding: 4px 12px;
            font-size: 13px;
        }

        .puls-content button:hover {
            background-color: #d8d8d8;
            color: #000000;
        }

        .puls-content button.suggested-action {
            background-color: #d4edda;
            color: #155724;
            border-color: #28a745;
        }

        .puls-content button.destructive-action {
            background-color: #f8d7da;
            color: #721c24;
            border-color: #dc3545;
        }

        .puls-content scrollbar slider {
            background-color: #cccccc;
            min-width: 8px;
            min-height: 8px;
        }

        .puls-content scrollbar slider:hover {
            background-color: #aaaaaa;
        }

        .text-green { color: #006622; font-weight: bold; }
        .text-cyan { color: #006688; font-weight: bold; }
        .text-magenta { color: #880088; font-weight: bold; }
        .text-orange { color: #cc6600; font-weight: bold; }
        .text-white { color: #333333; font-weight: bold; }
        .text-red { color: #cc0000; font-weight: bold; }
    "#.to_string()
}
