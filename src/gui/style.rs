use gtk::prelude::*;
use gtk::CssProvider;
use gtk::gdk::Screen;

pub fn apply_styles() {
    let provider = CssProvider::new();
    let css = r#"
        /* Base Application Styles - Dark Terminal Theme */
        window {
            background-color: #0d0d0d;
            color: #e0e0e0;
            font-family: 'JetBrains Mono', 'Fira Code', 'Monospace', monospace;
            font-size: 13px;
        }

        /* Container frames */
        frame {
            padding: 0;
        }

        frame > border {
            border: 1px solid #333333;
            border-radius: 4px;
        }

        /* Frame titles */
        frame label {
            color: #cccccc;
            font-weight: bold;
            padding: 0 6px;
        }

        /* Stack Switcher (Tabs) */
        stackswitcher button {
            background-color: #111111;
            color: #999999;
            border: 1px solid #2a2a2a;
            border-radius: 4px;
            padding: 4px 10px;
            font-size: 12px;
            font-weight: normal;
            min-height: 24px;
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

        /* HeaderBar */
        headerbar {
            background-color: #080808;
            border-bottom: 1px solid #333333;
            color: #e0e0e0;
        }

        headerbar .title {
            color: #00ffff;
            font-weight: bold;
            font-size: 16px;
        }

        headerbar .subtitle {
            color: #888888;
            font-size: 12px;
        }

        /* TreeView Styling */
        treeview {
            background-color: #111111;
            color: #e0e0e0;
            font-size: 13px;
            font-family: inherit;
        }

        treeview:hover {
            background-color: #1a1a1a;
        }

        treeview:selected {
            background-color: #1a2a3a;
            color: #00ffff;
        }

        treeview header button {
            background-color: #0a0a0a;
            color: #00ff88;
            font-weight: bold;
            padding: 4px 8px;
            border: none;
            border-bottom: 2px solid #00ff88;
            font-family: inherit;
            font-size: 13px;
        }

        /* Progress Bar Styling */
        progressbar {
            min-height: 16px;
            border-radius: 3px;
        }

        progressbar trough {
            background-color: #1a1a1a;
            border-radius: 3px;
            border: 1px solid #333333;
        }

        progressbar progress {
            background-color: #00ff88;
            border-radius: 3px;
            background-image: none;
        }

        /* Color utility classes */
        .text-green { color: #00ff88; font-weight: bold; }
        .text-cyan { color: #00ffff; font-weight: bold; }
        .text-magenta { color: #ff66ff; font-weight: bold; }
        .text-orange { color: #ffaa44; font-weight: bold; }
        .text-white { color: #ffffff; font-weight: bold; }
        .text-red { color: #ff4444; font-weight: bold; }

        label {
            font-family: inherit;
            font-size: 13px;
            color: #e0e0e0;
        }

        /* Separators */
        separator {
            background-color: #333333;
            min-height: 1px;
        }

        entry {
            background-color: #111111;
            color: #00ffff;
            border: 1px solid #333333;
            border-radius: 4px;
            padding: 4px 8px;
            font-size: 13px;
        }

        entry:focus {
            border-color: #00ffff;
        }

        /* Buttons */
        button {
            background-color: #151515;
            color: #e0e0e0;
            border: 1px solid #333333;
            border-radius: 4px;
            padding: 4px 12px;
            font-size: 13px;
        }

        button:hover {
            background-color: #1a1a1a;
            color: #ffffff;
        }

        button.suggested-action {
            background-color: #0a2a1a;
            color: #00ff88;
            border-color: #00ff88;
        }

        button.destructive-action {
            background-color: #2a0a0a;
            color: #ff4444;
            border-color: #ff4444;
        }

        /* ScrolledWindow / Scrollbar */
        scrolledwindow {
            background-color: transparent;
        }

        scrollbar slider {
            background-color: #333333;
            border-radius: 4px;
            min-width: 8px;
            min-height: 8px;
        }

        scrollbar slider:hover {
            background-color: #555555;
        }
    "#;

    if let Err(err) = provider.load_from_data(css.as_bytes()) {
        eprintln!("Failed to load CSS: {}", err);
    }

    if let Some(screen) = Screen::default() {
        gtk::StyleContext::add_provider_for_screen(
            &screen,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
