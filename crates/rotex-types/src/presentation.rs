//! Window handles and pixel size used to attach a presentation surface.

use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

/// Width and height in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Extent2D {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

impl Extent2D {
    /// Returns this extent with both axes at least 1.
    pub fn clamped(self) -> Self {
        Self {
            width: self.width.max(1),
            height: self.height.max(1),
        }
    }
}

/// Window handles and size used to attach a presentation surface.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceDescriptor {
    /// Display connection the window belongs to.
    pub display_handle: RawDisplayHandle,
    /// Window the surface presents into.
    pub window_handle: RawWindowHandle,
    /// Initial pixel size of the surface.
    pub extent: Extent2D,
}
