use std::sync::Arc;

use tauri::{AppHandle, Position, Runtime, Size, WebviewUrl, WebviewWindowBuilder};

use crate::{FromWindow, SplitView, WebviewWindowExt};

/// Type alias for window configuration function
type WindowConfigFn<'a, R> = Box<
    dyn FnOnce(
        WebviewWindowBuilder<'a, R, AppHandle<R>>,
    ) -> WebviewWindowBuilder<'a, R, AppHandle<R>>,
>;

/// Orientation for split views
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SplitViewOrientation {
    /// Vertical split (side-by-side panes)
    Vertical,
    /// Horizontal split (stacked panes)
    Horizontal,
}

impl SplitViewOrientation {
    /// Returns true if orientation is vertical
    pub fn is_vertical(&self) -> bool {
        matches!(self, SplitViewOrientation::Vertical)
    }
}

impl Default for SplitViewOrientation {
    fn default() -> Self {
        SplitViewOrientation::Vertical
    }
}

/// Configuration for a pane in the split view
#[derive(Debug, Clone)]
pub enum PaneConfig {
    /// A webview pane with a URL
    Webview { url: WebviewUrl },
    /// A native NSView pane (placeholder for now)
    Native { identifier: String },
}

/// Configuration for the split view
#[derive(Default)]
pub(crate) struct SplitViewConfig {
    pub orientation: Option<SplitViewOrientation>,
    pub divider_thickness: Option<f64>,
    pub panes: Vec<PaneConfig>,
}

/// Builder for creating split views with Tauri-like API
///
/// SplitViewBuilder provides a fluent interface that creates a Tauri window,
/// configures it with an NSSplitView, and applies split-view-specific configurations.
///
/// # Type Parameters
/// - `R`: The Tauri runtime type
/// - `T`: The split view type (must implement `FromWindow<R>`)
///
/// # Example
/// ```rust
/// use tauri_nssplitview::{SplitViewBuilder, SplitViewOrientation, PaneConfig};
/// use tauri::WebviewUrl;
///
/// let split_view = SplitViewBuilder::new(&app, "my-splitview")
///     .orientation(SplitViewOrientation::Vertical)
///     .add_pane(PaneConfig::Webview {
///         url: WebviewUrl::App("left.html".into())
///     })
///     .add_pane(PaneConfig::Webview {
///         url: WebviewUrl::App("right.html".into())
///     })
///     .build()?;
/// ```
pub struct SplitViewBuilder<'a, R: Runtime, T: FromWindow<R> + 'static> {
    handle: &'a AppHandle<R>,
    label: String,
    title: Option<String>,
    position: Option<Position>,
    size: Option<Size>,
    pub(crate) split_view_config: SplitViewConfig,
    window_fn: Option<WindowConfigFn<'a, R>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, R: Runtime + 'a, T: FromWindow<R> + 'static> SplitViewBuilder<'a, R, T> {
    /// Create a new SplitViewBuilder
    pub fn new(handle: &'a AppHandle<R>, label: impl Into<String>) -> Self {
        Self {
            handle,
            label: label.into(),
            title: None,
            position: None,
            size: None,
            split_view_config: SplitViewConfig::default(),
            window_fn: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set the window title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the window position
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    /// Set the window size
    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the split view orientation (vertical or horizontal)
    pub fn orientation(mut self, orientation: SplitViewOrientation) -> Self {
        self.split_view_config.orientation = Some(orientation);
        self
    }

    /// Add a pane to the split view
    pub fn add_pane(mut self, pane: PaneConfig) -> Self {
        self.split_view_config.panes.push(pane);
        self
    }

    /// Set the divider thickness
    pub fn divider_thickness(mut self, thickness: f64) -> Self {
        self.split_view_config.divider_thickness = Some(thickness);
        self
    }

    /// Apply a custom configuration function to the WebviewWindowBuilder
    ///
    /// This allows access to any Tauri window configuration not exposed by the split view builder.
    /// The closure receives the WebviewWindowBuilder and should return it after applying
    /// any desired configurations.
    ///
    /// # Example
    /// ```rust
    /// use tauri_nssplitview::SplitViewBuilder;
    ///
    /// SplitViewBuilder::new(&app, "my-splitview")
    ///     .with_window(|window| {
    ///         window
    ///             .min_inner_size(300.0, 200.0)
    ///             .max_inner_size(800.0, 600.0)
    ///             .resizable(true)
    ///     })
    ///     .build()
    /// ```
    pub fn with_window<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
                WebviewWindowBuilder<'a, R, AppHandle<R>>,
            ) -> WebviewWindowBuilder<'a, R, AppHandle<R>>
            + 'static,
    {
        self.window_fn = Some(Box::new(f) as WindowConfigFn<'a, R>);
        self
    }

    /// Build the split view
    ///
    /// Creates a Tauri window using the configured properties, converts it to
    /// a split view, and applies all split-view-specific settings.
    pub fn build(self) -> tauri::Result<Arc<dyn SplitView<R>>> {
        // For now, create a basic window
        // TODO: Implement actual NSSplitView creation

        // Use the first pane's URL if available, otherwise use default
        let url = self
            .split_view_config
            .panes
            .first()
            .and_then(|pane| match pane {
                PaneConfig::Webview { url } => Some(url.clone()),
                _ => None,
            })
            .unwrap_or(WebviewUrl::App("index.html".into()));

        let mut window_builder = WebviewWindowBuilder::new(self.handle, &self.label, url);

        if let Some(title) = self.title {
            window_builder = window_builder.title(title);
        }

        if let Some(position) = self.position {
            match position {
                Position::Physical(pos) => {
                    window_builder = window_builder.position(pos.x as f64, pos.y as f64);
                }
                Position::Logical(pos) => {
                    window_builder = window_builder.position(pos.x, pos.y);
                }
            }
        }

        if let Some(size) = self.size {
            match size {
                Size::Physical(s) => {
                    window_builder = window_builder.inner_size(s.width as f64, s.height as f64);
                }
                Size::Logical(s) => {
                    window_builder = window_builder.inner_size(s.width, s.height);
                }
            }
        }

        // Apply custom configuration if provided
        if let Some(window_fn) = self.window_fn {
            window_builder = window_fn(window_builder);
        }

        // Build the window
        let window = window_builder.build()?;

        // Convert to split view
        let split_view = window.to_split_view::<T>()?;

        // TODO: Apply split view configuration
        // - Set orientation
        // - Add panes
        // - Configure dividers

        Ok(split_view)
    }
}
