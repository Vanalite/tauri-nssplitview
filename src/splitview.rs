use std::any::Any;
use std::cell::{OnceCell, RefCell};

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::ClassType;
use objc2_app_kit::{NSSplitView, NSView, NSWindow, NSWindowDelegate};
use objc2_foundation::NSRect;
use tauri::{AppHandle, Manager, Runtime, WebviewWindow};

use crate::{FromWindow, SplitView};

/// A basic split view implementation
///
/// This wraps a Tauri window and replaces its content view with an NSSplitView
/// containing multiple panes.
pub struct BasicSplitView<R: Runtime = tauri::Wry> {
    split_view: Retained<NSSplitView>,
    label: String,
    app_handle: AppHandle<R>,
    original_delegate: OnceCell<Retained<ProtocolObject<dyn NSWindowDelegate>>>,
    event_handler: RefCell<Option<Retained<ProtocolObject<dyn NSWindowDelegate>>>>,
}

// SAFETY: While NSSplitView must only be used on the main thread, we implement Send + Sync
// to allow passing references through Tauri's command system. Users must ensure
// actual split view operations happen on the main thread.
unsafe impl<R: Runtime> Send for BasicSplitView<R> {}
unsafe impl<R: Runtime> Sync for BasicSplitView<R> {}

impl<R: Runtime> BasicSplitView<R> {
    /// Create a new BasicSplitView from a window
    pub fn new(
        split_view: Retained<NSSplitView>,
        label: String,
        app_handle: AppHandle<R>,
    ) -> Self {
        Self {
            split_view,
            label,
            app_handle,
            original_delegate: OnceCell::new(),
            event_handler: RefCell::new(None),
        }
    }
}

impl<R: Runtime> SplitView<R> for BasicSplitView<R> {
    fn show(&self) {
        if let Some(window) = self.window() {
            unsafe {
                let _: () = objc2::msg_send![&*window, orderFrontRegardless];
            }
        }
    }

    fn hide(&self) {
        if let Some(window) = self.window() {
            unsafe {
                let _: () = objc2::msg_send![&*window, orderOut: objc2::ffi::nil];
            }
        }
    }

    fn to_window(&self) -> Option<WebviewWindow<R>> {
        use tauri::Manager;
        self.app_handle.get_webview_window(&self.label)
    }

    fn as_split_view(&self) -> &NSSplitView {
        &self.split_view
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_event_handler(
        &self,
        handler: Option<&ProtocolObject<dyn NSWindowDelegate>>,
    ) {
        if let Some(window) = self.window() {
            unsafe {
                match handler {
                    Some(h) => {
                        // Store original delegate if this is the first time
                        if self.event_handler.borrow().is_none() && self.original_delegate.get().is_none() {
                            if let Some(current_delegate) = window.delegate() {
                                let _ = self.original_delegate.set(current_delegate);
                            }
                        }

                        // Create a retained copy by calling retain on the raw pointer
                        let ptr = h as *const ProtocolObject<dyn NSWindowDelegate>;
                        let retained_handler = Retained::retain(ptr as *mut ProtocolObject<dyn NSWindowDelegate>);
                        if let Some(handler) = retained_handler {
                            *self.event_handler.borrow_mut() = Some(handler);
                        }

                        // Set as window delegate
                        let _: () = objc2::msg_send![&*window, setDelegate: h];
                    }
                    None => {
                        if self.original_delegate.get().is_none() {
                            return;
                        }

                        // Clear stored handler
                        *self.event_handler.borrow_mut() = None;

                        // Restore original delegate
                        if let Some(orig_delegate) = self.original_delegate.get() {
                            let _: () = objc2::msg_send![&*window, setDelegate: &**orig_delegate];
                        }
                    }
                }
            }
        }
    }

    fn is_visible(&self) -> bool {
        if let Some(window) = self.window() {
            unsafe { objc2::msg_send![&*window, isVisible] }
        } else {
            false
        }
    }

    fn is_vertical(&self) -> bool {
        unsafe { objc2::msg_send![&*self.split_view, isVertical] }
    }

    fn pane_count(&self) -> usize {
        unsafe {
            let subviews: Retained<objc2_foundation::NSArray<NSView>> =
                objc2::msg_send![&*self.split_view, subviews];
            objc2::msg_send![&*subviews, count]
        }
    }

    fn set_divider_position(&self, divider_index: usize, position: f64) {
        unsafe {
            let _: () = objc2::msg_send![
                &*self.split_view,
                setPosition: position,
                ofDividerAtIndex: divider_index as isize
            ];
        }
    }

    fn get_divider_position(&self, divider_index: usize) -> f64 {
        // NSSplitView doesn't have a direct method to get divider position
        // We need to calculate it from subview frames
        unsafe {
            let subviews: Retained<objc2_foundation::NSArray<NSView>> =
                objc2::msg_send![&*self.split_view, subviews];
            let count: usize = objc2::msg_send![&*subviews, count];

            if divider_index >= count - 1 {
                return 0.0;
            }

            let view: Retained<NSView> = objc2::msg_send![&*subviews, objectAtIndex: divider_index];
            let frame: objc2_foundation::NSRect = objc2::msg_send![&*view, frame];

            if self.is_vertical() {
                frame.origin.x + frame.size.width
            } else {
                frame.origin.y + frame.size.height
            }
        }
    }

    fn set_divider_thickness(&self, thickness: f64) {
        // NSSplitView divider thickness is typically controlled by the dividerThickness property
        // but it's read-only. We'd need to subclass to customize this.
        // For now, this is a no-op placeholder
        let _ = thickness;
    }

    fn divider_thickness(&self) -> f64 {
        unsafe { objc2::msg_send![&*self.split_view, dividerThickness] }
    }

    fn pane_at_index(&self, index: usize) -> Option<Retained<NSView>> {
        unsafe {
            let subviews: Retained<objc2_foundation::NSArray<NSView>> =
                objc2::msg_send![&*self.split_view, subviews];
            let count: usize = objc2::msg_send![&*subviews, count];

            if index < count {
                Some(objc2::msg_send![&*subviews, objectAtIndex: index])
            } else {
                None
            }
        }
    }

    fn set_pane_collapsible(&self, index: usize, _collapsible: bool) {
        // This would typically be handled by NSSplitViewDelegate
        // For now, this is a placeholder
        let _ = index;
    }

    fn is_pane_collapsed(&self, index: usize) -> bool {
        if let Some(view) = self.pane_at_index(index) {
            unsafe {
                let result: bool = objc2::msg_send![
                    &*self.split_view,
                    isSubviewCollapsed: &*view
                ];
                result
            }
        } else {
            false
        }
    }

    fn set_pane_min_size(&self, _index: usize, _size: f64) {
        // This would be handled by NSSplitViewDelegate's constrainMinCoordinate method
        // For now, this is a placeholder
    }

    fn set_pane_max_size(&self, _index: usize, _size: f64) {
        // This would be handled by NSSplitViewDelegate's constrainMaxCoordinate method
        // For now, this is a placeholder
    }

    fn window(&self) -> Option<Retained<NSWindow>> {
        unsafe { objc2::msg_send![&*self.split_view, window] }
    }
}

impl<R: Runtime> FromWindow<R> for BasicSplitView<R> {
    fn from_window(window: WebviewWindow<R>, label: String) -> tauri::Result<Self> {
        unsafe {
            // Get the NSWindow as a raw pointer
            let ns_window_ptr = window.ns_window().map_err(|e| {
                tauri::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to get NSWindow: {:?}", e),
                ))
            })?;
            let ns_window = ns_window_ptr as *mut AnyObject;

            // Get the current content view before we replace it
            let original_content_view: *mut AnyObject = objc2::msg_send![ns_window, contentView];

            // Create an NSSplitView
            let content_frame: NSRect = objc2::msg_send![original_content_view, frame];

            // Allocate and initialize the split view
            let alloc: *mut AnyObject = objc2::msg_send![NSSplitView::class(), alloc];
            let init: *mut AnyObject = objc2::msg_send![alloc, initWithFrame: content_frame];
            let split_view = Retained::retain(init as *mut NSSplitView).unwrap();

            // Set vertical orientation by default
            let _: () = objc2::msg_send![&*split_view, setVertical: true];

            // Set autoresizing mask
            let resize_mask = objc2_app_kit::NSAutoresizingMaskOptions::ViewWidthSizable
                | objc2_app_kit::NSAutoresizingMaskOptions::ViewHeightSizable;
            let _: () = objc2::msg_send![&*split_view, setAutoresizingMask: resize_mask];

            // Set the split view as the window's content view
            let _: () = objc2::msg_send![ns_window, setContentView: &*split_view];

            // Add the original content view as the first pane
            let _: () = objc2::msg_send![&*split_view, addSubview: original_content_view];

            Ok(BasicSplitView::new(
                split_view,
                label,
                window.app_handle().clone(),
            ))
        }
    }
}
