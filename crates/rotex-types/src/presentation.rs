use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

impl Extent2D {
    pub fn clamped(self) -> Self {
        Self {
            width: self.width.max(1),
            height: self.height.max(1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SurfaceDescriptor {
    pub display_handle: RawDisplayHandle,
    pub window_handle: RawWindowHandle,
    pub extent: Extent2D,
}
