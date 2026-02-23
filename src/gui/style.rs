use gtk::prelude::*;
use gtk::CssProvider;
use gtk::gdk::Screen;

pub fn apply_styles() {
    let provider = CssProvider::new();
    let css = r#"
        /* Base Application Styles */
        window {
            background-color: #0c0c0c; /* Black background matching TUI */
            color: #d0d0d0;
            font-family: 'Monospace', 'Courier New', monospace;
            font-size: 11px;
        }

        /* Container frames to look like TUI boxes */
        frame {
            padding: 0;
        }

        frame > border {
            border: 1px solid #c0c0c0;
            border-radius: 4px;
        }

        /* Labels inside frames acting as titles */
        frame label.frame-title {
            color: #c0c0c0;
            font-weight: bold;
            padding: 0 4px;
        }

        /* Notebook (Tabs) Styling */
        notebook {
            background-color: #0c0c0c;
        }
        
        notebook > header {
            background-color: #0c0c0c;
            border-bottom: 1px solid #c0c0c0;
            padding: 0;
        }
        
        notebook > header > tabs > tab {
            background-color: transparent;
            color: #c0c0c0;
            padding: 2px 8px;
            font-size: 12px;
            font-weight: normal;
            border: none;
            border-radius: 0;
        }
        
        notebook > header > tabs > tab:checked {
            color: #ffffff;
            font-weight: bold;
            background-color: #1a1a1a;
        }

        notebook > header > tabs > tab:hover {
            color: #ffffff;
            background-color: #222222;
        }

        /* TreeView Styling */
        treeview {
            background-color: #0c0c0c;
            color: #d0d0d0;
            font-size: 11px;
            font-family: inherit;
        }

        treeview:hover {
            background-color: #1a1a1a;
        }
        
        treeview:selected {
            background-color: #262626;
            color: #00ffff; /* Cyan selection */
        }

        treeview header button {
            background-color: #0c0c0c;
            color: #00ff00; /* Green headers typical for TUI columns */
            font-weight: normal;
            padding: 2px 4px;
            border: none;
            border-bottom: 1px solid #555555;
            font-family: inherit;
        }

        /* Progress Bar Styling */
        progressbar {
            min-height: 14px;
            border-radius: 0;
        }
        
        progressbar trough {
            background-color: #222222;
            border-radius: 0;
        }
        
        progressbar progress {
            background-color: #00ff00; /* Neon Green */
            border-radius: 0;
            background-image: none;
        }

        /* Specific Colors matching TUI */
        .text-green { color: #00ff00; }
        .text-cyan { color: #00ffff; }
        .text-magenta { color: #ff00ff; }
        .text-orange { color: #ffaa00; }
        .text-white { color: #ffffff; }

        label {
            font-family: inherit;
        }
        
        /* Grid Borders and Separators */
        separator {
            background-color: #555555;
        }
        
        entry {
            background-color: #111111;
            color: #00ffff;
            border: 1px solid #555555;
            border-radius: 0;
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
