//! Settings for creating a GPU platform object.

use raw_window_handle::RawDisplayHandle;

/// API validation and diagnostics requested at platform creation.
///
/// Backends that have a validation layer honor this. Backends without one,
/// including GL, ignore it. The heavy GPU-based validation modes are not
/// represented here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DebugMode {
    /// No validation layer and no debug messenger.
    Off,
    /// The backend's standard validation layer and diagnostics.
    Validation,
}

impl DebugMode {
    /// Validation when debug assertions are enabled, otherwise [`Self::Off`].
    pub const fn from_build() -> Self {
        if cfg!(debug_assertions) {
            Self::Validation
        } else {
            Self::Off
        }
    }
}

impl Default for DebugMode {
    fn default() -> Self {
        Self::from_build()
    }
}

/// Settings for creating a GPU platform object.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlatformDescriptor {
    /// Validation and diagnostics the backend should enable.
    pub debug: DebugMode,
    /// Display connection a presentation surface will use.
    ///
    /// Vulkan turns this into the instance extensions the window system requires.
    /// Other backends ignore it.
    pub display_handle: Option<RawDisplayHandle>,
}

impl PlatformDescriptor {
    /// A descriptor that presents on `display_handle`.
    ///
    /// Debug mode follows [`DebugMode::from_build`].
    pub const fn with_display(display_handle: RawDisplayHandle) -> Self {
        Self {
            debug: DebugMode::from_build(),
            display_handle: Some(display_handle),
        }
    }
}

// SAFETY: `RawDisplayHandle` is a plain window-system pointer. Moving the
// descriptor between threads does not dereference it. Platform creation runs
// on the thread that owns the window.
unsafe impl Send for PlatformDescriptor {}
// SAFETY: sharing the descriptor shares the pointer bits, not a borrowed window.
unsafe impl Sync for PlatformDescriptor {}
