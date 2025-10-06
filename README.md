# tauri-splitview

Create macOS split views for your Tauri app using native NSSplitView. Build applications with multiple views where one or more use native macOS split view functionality alongside standard Tauri webviews.

## What are split views?

Split views are a native macOS UI component ([`NSSplitView`](https://developer.apple.com/documentation/appkit/nssplitview)) that allows you to divide a window into multiple resizable panes. This plugin enables you to create hybrid Tauri applications where some views use native macOS split view capabilities while others use standard webview-based rendering.

## Quick Start

### 1. Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tauri = { version = "2.8", features = ["macos-private-api"] }
tauri-splitview = { git = "https://github.com/vanalite/tauri-splitview" }
```

### 2. Initialize the Plugin

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_splitview::init())
        .setup(|app| {
            // Your setup code here
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3. Create a Split View

**Basic usage - Convert a window to a split view:**

```rust
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_splitview::{BasicSplitView, WebviewWindowExt};

// In your setup function
let window = WebviewWindowBuilder::new(
    app,
    "main",
    WebviewUrl::App("index.html".into()),
)
.title("My Split View App")
.inner_size(800.0, 600.0)
.build()?;

// Convert to split view
let split_view = window.to_split_view::<BasicSplitView>()?;

// The window now has a split view with the original webview as the first pane
split_view.show();
```

### 4. Access Split View from Commands

```rust
use tauri::Manager;
use tauri_splitview::ManagerExt;

#[tauri::command]
fn get_split_info(app: tauri::AppHandle) -> Result<String, String> {
    match app.get_split_view("main") {
        Ok(split_view) => {
            Ok(format!(
                "Panes: {}, Vertical: {}, Visible: {}",
                split_view.pane_count(),
                split_view.is_vertical(),
                split_view.is_visible()
            ))
        }
        Err(_) => Err("Split view not found".to_string()),
    }
}

#[tauri::command]
fn set_divider(app: tauri::AppHandle, position: f64) -> Result<(), String> {
    match app.get_split_view("main") {
        Ok(split_view) => {
            split_view.set_divider_position(0, position);
            Ok(())
        }
        Err(_) => Err("Split view not found".to_string()),
    }
}
```

## API Overview

### SplitView Trait Methods

```rust
// Visibility
split_view.show();
split_view.hide();
split_view.is_visible() -> bool

// Layout
split_view.is_vertical() -> bool
split_view.pane_count() -> usize

// Divider Control
split_view.set_divider_position(divider_index: usize, position: f64);
split_view.get_divider_position(divider_index: usize) -> f64;
split_view.divider_thickness() -> f64;

// Pane Access
split_view.pane_at_index(index: usize) -> Option<Retained<NSView>>;
split_view.is_pane_collapsed(index: usize) -> bool;

// Conversion
split_view.to_window() -> Option<WebviewWindow>;
split_view.label() -> &str;
```

### Manager Extensions

```rust
use tauri_splitview::ManagerExt;

// Get split view by label
let split_view = app.get_split_view("main")?;

// Remove split view
app.remove_split_view("main");
```

### Window Extensions

```rust
use tauri_splitview::WebviewWindowExt;

// Convert any Tauri window to a split view
let split_view = window.to_split_view::<BasicSplitView>()?;
```

## Features

- ✅ Create native macOS split views with NSSplitView
- ✅ Convert existing Tauri windows to split views
- ✅ Support for both vertical and horizontal splits
- ✅ Resizable panes with adjustable divider positions
- ✅ Query pane count and state
- ✅ Type-safe Rust API
- ✅ Thread-safe operations
- ✅ Proper memory management with objc2

## Platform Support

**macOS only** - This plugin uses native macOS APIs (NSSplitView from AppKit) and requires:
- macOS 10.13+
- Tauri with `macos-private-api` feature enabled

## Examples

Check out the [examples](examples/) directory:
- **basic-splitview** - Complete working example showing split view creation and usage

To run the example:
```bash
cd examples/basic-splitview
cargo tauri dev
```

## Use Cases

Split views are ideal for:
- Multi-pane editors and IDEs
- Side-by-side document viewers
- Applications with navigation/content layouts
- Tool palettes with resizable sections
- Hybrid native/web UI applications

## How It Works

The plugin:
1. Takes an existing Tauri window
2. Creates a native NSSplitView
3. Replaces the window's content view with the split view
4. Adds the original webview as the first pane
5. Provides a Rust API for controlling the split view

## Requirements

- Tauri 2.8+
- Rust 1.75+
- macOS target platform
- `macos-private-api` feature enabled in Tauri

## Documentation

- **README.md** - This file (getting started)
- **CLAUDE.md** - Architecture and development guide
- **PROGRESS.md** - Implementation details
- **examples/** - Working code examples

## Contributing

Contributions welcome! Please:
1. Read the architecture guide in CLAUDE.md
2. Check existing issues
3. Submit PRs with tests
4. Follow Rust conventions

## License

MIT or Apache-2.0 where applicable.

## Credits

Based on the architecture of [tauri-nspanel](https://github.com/ahkohd/tauri-nspanel).
