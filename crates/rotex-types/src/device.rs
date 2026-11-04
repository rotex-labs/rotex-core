use crate::caps::{CoreCaps, FeatureSet};
use crate::format::Limits;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DeviceFeatures {
    pub sampler_anisotropy: bool,
    pub fill_mode_non_solid: bool,
    pub wide_lines: bool,
}

impl DeviceFeatures {
    pub const fn covers(self, required: Self) -> bool {
        (!required.sampler_anisotropy || self.sampler_anisotropy)
            && (!required.fill_mode_non_solid || self.fill_mode_non_solid)
            && (!required.wide_lines || self.wide_lines)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorkQueueHints {
    pub needs_graphics: bool,
    pub needs_compute: bool,
    pub needs_copy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceDescriptor {
    pub required_features: DeviceFeatures,
    pub work_queues: WorkQueueHints,
    pub required_extensions: FeatureSet,
    pub required_limits: Limits,
}

impl DeviceDescriptor {
    pub fn default_graphics() -> Self {
        Self {
            required_features: DeviceFeatures::default(),
            work_queues: WorkQueueHints {
                needs_graphics: true,
                needs_compute: false,
                needs_copy: true,
            },
            required_extensions: FeatureSet::empty(),
            required_limits: Limits::CORE_MIN,
        }
    }
}

impl Default for DeviceDescriptor {
    fn default() -> Self {
        Self::default_graphics()
    }
}

/// Stable index assigned at enumeration time (same order as the returned adapter list).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdapterId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareFamily {
    DiscreteGpu,
    IntegratedGpu,
    VirtualGpu,
    Cpu,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum GpuApiBackend {
    Vulkan,
    Metal,
    Dx12,
    OpenGl,
    WebGl,
    WebGpu,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AdapterQueueSupport {
    pub graphics_queue_count: u32,
    pub compute_queue_count: u32,
    pub transfer_queue_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AdapterCapabilities {
    pub supports_window_presentation: bool,
    pub queue_support: AdapterQueueSupport,
    pub available_features: DeviceFeatures,
    pub limits: Limits,
    pub extensions: FeatureSet,
    pub core: CoreCaps,
}

#[derive(Debug, Clone)]
pub struct AdapterInfo {
    pub id: AdapterId,
    pub name: String,
    pub hardware_family: HardwareFamily,
    pub vendor_id: u32,
    pub device_id: u32,
    pub driver_name: String,
    pub driver_info: String,
    pub api_backend: GpuApiBackend,
    pub capabilities: AdapterCapabilities,
}
