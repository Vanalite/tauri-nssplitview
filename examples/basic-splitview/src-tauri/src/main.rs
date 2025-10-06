// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_splitview::{BasicSplitView, ManagerExt, WebviewWindowExt};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_splitview::init())
        .setup(|app| {
            // Get the main window that Tauri creates by default
            let window = app.get_webview_window("main").expect("Main window not found");

            // Convert the window to a split view
            let split_view = window.to_split_view::<BasicSplitView>()?;

            println!("✅ Split view created successfully!");
            println!("  - Label: {}", split_view.label());
            println!("  - Is vertical: {}", split_view.is_vertical());
            println!("  - Pane count: {}", split_view.pane_count());

            // Add a native macOS view as the second pane
            #[cfg(target_os = "macos")]
            add_native_view(split_view.as_ref());

            println!("  - Final pane count: {}", split_view.pane_count());
            println!("\n🎉 Split view now has:");
            println!("  • Left pane: Tauri webview (index.html)");
            println!("  • Right pane: Native macOS NSView");

            // Show the split view
            split_view.show();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_split_view_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(target_os = "macos")]
fn add_native_view(split_view: &dyn tauri_splitview::SplitView) {
    // Use the re-exported types from tauri_splitview
    use tauri_splitview::objc2;
    use tauri_splitview::objc2::rc::Retained;
    use tauri_splitview::objc2::runtime::AnyObject;
    use tauri_splitview::objc2_app_kit::NSView;
    use tauri_splitview::objc2_foundation::{NSPoint, NSRect, NSSize};

    unsafe {
        // Get the NSSplitView
        let ns_split_view = split_view.as_split_view();

        // Create a simple native NSView with purple background
        let native_view: Retained<NSView> = {
            let alloc: *mut AnyObject = objc2::msg_send![objc2::class!(NSView), alloc];
            let frame = NSRect {
                origin: NSPoint { x: 0.0, y: 0.0 },
                size: NSSize {
                    width: 400.0,
                    height: 600.0,
                },
            };
            let view: *mut AnyObject = objc2::msg_send![alloc, initWithFrame: frame];
            Retained::retain(view as *mut NSView).unwrap()
        };

        // Set purple background using layer
        let _: () = objc2::msg_send![&*native_view, setWantsLayer: true];
        let layer: *mut AnyObject = objc2::msg_send![&*native_view, layer];

        // Create purple color (RGB: 0.4, 0.3, 0.6)
        let color_space: *mut AnyObject = objc2::msg_send![
            objc2::class!(NSColorSpace),
            deviceRGBColorSpace
        ];
        let components: [f64; 4] = [0.4, 0.3, 0.6, 1.0];
        let color: *mut AnyObject = objc2::msg_send![
            objc2::class!(NSColor),
            colorWithColorSpace: color_space,
            components: components.as_ptr(),
            count: 4usize
        ];
        let cg_color: *mut AnyObject = objc2::msg_send![color, CGColor];
        let _: () = objc2::msg_send![layer, setBackgroundColor: cg_color];

        // Add it as a subview to the split view
        let _: () = objc2::msg_send![ns_split_view, addSubview: &*native_view];

        println!("  ✓ Added native macOS NSView with purple background");
    }
}

#[cfg(not(target_os = "macos"))]
fn add_native_view(_split_view: &dyn tauri_splitview::SplitView) {
    println!("  ⚠ Native views only supported on macOS");
}

#[tauri::command]
fn get_split_view_info(app: tauri::AppHandle) -> Result<String, String> {
    match app.get_split_view("main") {
        Ok(split_view) => {
            let info = format!(
                "Split View Info:\n\
                 - Label: {}\n\
                 - Vertical: {}\n\
                 - Pane Count: {}\n\
                 - Visible: {}\n\
                 - Divider Thickness: {}",
                split_view.label(),
                split_view.is_vertical(),
                split_view.pane_count(),
                split_view.is_visible(),
                split_view.divider_thickness(),
            );
            Ok(info)
        }
        Err(_) => Err("Split view not found".to_string()),
    }
}
