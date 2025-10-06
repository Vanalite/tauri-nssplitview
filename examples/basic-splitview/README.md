# Basic Split View Example - Two Panes Side-by-Side

This example demonstrates **the key feature** of tauri-splitview: **native macOS views alongside Tauri webviews**.

## What You'll See

When you run this example, you'll see a window split into **two panes**:

| Left Pane (Webview) | Right Pane (Native) |
|---------------------|---------------------|
| 🌐 HTML + CSS + JS | 🍎 Native NSTextField |
| Tauri webview | Pure AppKit NSView |
| Gradient background | Purple background |
| Interactive button | Static text labels |

**You can drag the divider in the middle to resize the panes!**

## How to Run

### Prerequisites
- macOS (required for native views)
- Rust and Cargo installed
- Tauri CLI: `cargo install tauri-cli --version "^2.0.0"`

### Running the Example

```bash
cd examples/basic-splitview
cargo tauri dev
```

## Expected Output

### Console Output
```
✅ Split view created successfully!
  - Label: main
  - Is vertical: true
  - Pane count: 1
  ✓ Added native macOS view to split view
  - Final pane count: 2

🎉 Split view now has:
  • Left pane: Tauri webview (index.html)
  • Right pane: Native macOS NSTextField
```

### Visual Result
- **Left side**: Beautiful gradient webview with split view info
- **Right side**: Purple native view with white text
- **Divider**: Draggable separator between the panes

## What This Demonstrates

✅ **Hybrid UI**: Web + Native side-by-side
✅ **Native NSView**: Pure AppKit, no HTML
✅ **Tauri Webview**: Standard web technologies
✅ **Resizable**: Drag the divider to adjust
✅ **Plugin Usage**: How to add native views

## Code Highlights

### Creating the Split View (main.rs)
```rust
// Convert window to split view
let split_view = window.to_split_view::<BasicSplitView>()?;

// Add native macOS view as second pane
add_native_view(&split_view);

// Result: 2 panes side-by-side!
```

### Native View Creation
The right pane is created using pure AppKit APIs:
- `NSView` for the container
- `NSTextField` for text labels
- `NSColor` for background
- No HTML, CSS, or JavaScript!

## Try It Yourself

1. **Resize the panes**: Drag the divider
2. **Click "Refresh Info"**: See split view details
3. **Check the console**: See the creation process
4. **Modify the code**: Change colors, text, layout

## Troubleshooting

**No second pane visible?**
- Check console output for "Added native macOS view"
- Pane count should be 2
- Try dragging from the center

**Build errors?**
- Make sure you're on macOS
- Install Tauri CLI: `cargo install tauri-cli`
- Check you're in the right directory

**Window won't open?**
- Icon files are placeholders - this is OK
- The app will still run fine

## Next Steps

- Modify `create_native_label_view()` to customize the native pane
- Change the webview HTML in `index.html`
- Add more panes by calling `addSubview` multiple times
- Experiment with horizontal orientation

## More Information

- [Main README](../../README.md)
- [Usage Guide](../../USAGE_GUIDE.md)
- [API Documentation](../../STATUS.md)
