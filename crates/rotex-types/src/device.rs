use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueCategory {
    Graphics,
    Compute,
    Transfer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueueRequest {
    pub category: QueueCategory,
    pub count: u32,
}

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

#[derive(Debug, Clone)]
pub struct InstanceDescriptor {
    pub enable_validation: bool,
    pub required_instance_extensions: Vec<String>,
}

impl Default for InstanceDescriptor {
    fn default() -> Self {
        Self {
            enable_validation: cfg!(debug_assertions),
            required_instance_extensions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DeviceFeatures {
    pub sampler_anisotropy: bool,
    pub fill_mode_non_solid: bool,
    pub wide_lines: bool,
}

#[derive(Debug, Clone)]
pub struct DeviceDescriptor {
    pub enable_swapchain: bool,
    pub queues: Vec<QueueRequest>,
    pub required_features: DeviceFeatures,
}

impl Default for DeviceDescriptor {
    fn default() -> Self {
        Self {
            enable_swapchain: true,
            queues: vec![
                QueueRequest {
                    category: QueueCategory::Graphics,
                    count: 1,
                },
                QueueRequest {
                    category: QueueCategory::Transfer,
                    count: 1,
                },
            ],
            required_features: DeviceFeatures::default(),
        }
    }
}
