//! Settings for creating a GPU platform object.

use raw_window_handle::RawDisplayHandle;

/// Settings for creating a GPU platform object.
#[derive(Debug, Clone)]
pub struct PlatformDescriptor {
    /// Enables API validation layers when the backend has them.
    pub enable_validation: bool,
    /// Extra platform extensions the caller wants enabled.
    pub platform_extensions: Vec<String>,
    /// Display connection a presentation surface will use.
    ///
    /// Vulkan turns this into the instance extensions the window system requires.
    /// Other backends ignore it.
    pub display_handle: Option<RawDisplayHandle>,
}

impl Default for PlatformDescriptor {
    fn default() -> Self {
        Self {
            enable_validation: cfg!(debug_assertions),
            platform_extensions: Vec::new(),
            display_handle: None,
        }
    }
}

// SAFETY: `RawDisplayHandle` is a plain window-system pointer. Moving the
// descriptor between threads does not dereference it. Platform creation runs
// on the thread that owns the window.
unsafe impl Send for PlatformDescriptor {}
// SAFETY: sharing the descriptor shares the pointer bits, not a borrowed window.
unsafe impl Sync for PlatformDescriptor {}
