pub mod builder;
pub mod common;
pub mod event;
pub mod splitview;

// Re-export for macro usage
#[doc(hidden)]
pub use objc2;
#[doc(hidden)]
pub use objc2_app_kit;
#[doc(hidden)]
pub use objc2_foundation;
#[doc(hidden)]
pub use pastey;

use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use objc2::runtime::ProtocolObject;
use objc2_app_kit::NSWindowDelegate;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, WebviewWindow,
};

pub use builder::{PaneConfig, SplitViewBuilder, SplitViewOrientation};
pub use splitview::BasicSplitView;

// Re-export commonly used types for convenience
pub use objc2::runtime::AnyObject;
pub use objc2_app_kit::{NSResponder, NSSplitView, NSView, NSWindow};
pub use objc2_foundation::{NSNotification, NSObject, NSPoint, NSRect, NSSize};

/// Trait for event handlers that can be used with split views
pub trait EventHandler {
    /// Get the NSWindowDelegate protocol object
    fn as_delegate(&self) -> ProtocolObject<dyn NSWindowDelegate>;
}

/// Common trait for all split view types
pub trait SplitView<R: tauri::Runtime = tauri::Wry>: Send + Sync {
    /// Show the split view
    fn show(&self);

    /// Hide the split view
    fn hide(&self);

    /// Convert split view back to a regular Tauri window
    fn to_window(&self) -> Option<tauri::WebviewWindow<R>>;

    /// Get a reference to the underlying NSSplitView
    fn as_split_view(&self) -> &objc2_app_kit::NSSplitView;

    /// Get the split view label
    fn label(&self) -> &str;

    /// Downcast to concrete type
    fn as_any(&self) -> &dyn Any;

    /// Set the event handler (window delegate)
    /// Pass `None` to remove the current delegate
    fn set_event_handler(&self, handler: Option<&ProtocolObject<dyn NSWindowDelegate>>);

    // Query methods
    /// Check if the split view is visible
    fn is_visible(&self) -> bool;

    /// Check if split view is vertical (true) or horizontal (false)
    fn is_vertical(&self) -> bool;

    /// Get number of panes
    fn pane_count(&self) -> usize;

    // Divider methods
    /// Set the position of a divider (0-indexed divider, 0.0-1.0 position)
    fn set_divider_position(&self, divider_index: usize, position: f64);

    /// Get the position of a divider (0-indexed divider)
    fn get_divider_position(&self, divider_index: usize) -> f64;

    /// Set divider thickness
    fn set_divider_thickness(&self, thickness: f64);

    /// Get divider thickness
    fn divider_thickness(&self) -> f64;

    // Pane methods
    /// Get a pane view by index
    fn pane_at_index(&self, index: usize) -> Option<objc2::rc::Retained<objc2_app_kit::NSView>>;

    /// Set whether a pane can collapse
    fn set_pane_collapsible(&self, index: usize, collapsible: bool);

    /// Check if a pane is collapsed
    fn is_pane_collapsed(&self, index: usize) -> bool;

    /// Set minimum size for a pane
    fn set_pane_min_size(&self, index: usize, size: f64);

    /// Set maximum size for a pane
    fn set_pane_max_size(&self, index: usize, size: f64);

    // Window methods
    /// Get the parent window
    fn window(&self) -> Option<objc2::rc::Retained<objc2_app_kit::NSWindow>>;
}

/// Trait for split views that can be created from a window
pub trait FromWindow<R: Runtime>: SplitView<R> + Sized {
    /// Create split view from a Tauri window
    fn from_window(window: WebviewWindow<R>, label: String) -> tauri::Result<Self>;
}

/// Type alias for shared split view references
pub type SplitViewHandle<R> = Arc<dyn SplitView<R>>;

pub struct Store<R: Runtime> {
    split_views: HashMap<String, SplitViewHandle<R>>,
}

impl<R: Runtime> Default for Store<R> {
    fn default() -> Self {
        Self {
            split_views: HashMap::new(),
        }
    }
}

pub struct SplitViewManager<R: Runtime>(pub Mutex<Store<R>>);

impl<R: Runtime> Default for SplitViewManager<R> {
    fn default() -> Self {
        Self(Mutex::new(Store::default()))
    }
}

pub trait ManagerExt<R: Runtime> {
    fn get_split_view(&self, label: &str) -> Result<SplitViewHandle<R>, Error>;
    fn remove_split_view(&self, label: &str) -> Option<SplitViewHandle<R>>;
}

#[derive(Debug)]
pub enum Error {
    SplitViewNotFound,
}

impl<R: Runtime, T: Manager<R>> ManagerExt<R> for T {
    fn get_split_view(&self, label: &str) -> Result<SplitViewHandle<R>, Error> {
        let manager = self.state::<self::SplitViewManager<R>>();
        let manager = manager.0.lock().unwrap();

        match manager.split_views.get(label) {
            Some(split_view) => Ok(split_view.clone()),
            None => Err(Error::SplitViewNotFound),
        }
    }

    fn remove_split_view(&self, label: &str) -> Option<SplitViewHandle<R>> {
        self.state::<self::SplitViewManager<R>>()
            .0
            .lock()
            .unwrap()
            .split_views
            .remove(label)
    }
}

pub trait WebviewWindowExt<R: Runtime> {
    /// Convert window to specific split view type
    fn to_split_view<S: FromWindow<R> + 'static>(&self) -> tauri::Result<SplitViewHandle<R>>;
}

impl<R: Runtime> WebviewWindowExt<R> for WebviewWindow<R> {
    fn to_split_view<S: FromWindow<R> + 'static>(&self) -> tauri::Result<SplitViewHandle<R>> {
        let label = self.label().to_string();
        let split_view = S::from_window(self.clone(), label.clone())?;
        let arc_split_view = Arc::new(split_view) as SplitViewHandle<R>;

        let manager = self.state::<SplitViewManager<R>>();
        manager
            .0
            .lock()
            .unwrap()
            .split_views
            .insert(label, arc_split_view.clone());

        Ok(arc_split_view)
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("splitview")
        .setup(|app, _api| {
            app.manage(self::SplitViewManager::<R>::default());

            Ok(())
        })
        .build()
}
