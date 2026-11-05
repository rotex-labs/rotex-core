//! Adapter enumeration records and the requirements used to filter them.

use crate::caps::{CoreCaps, FeatureSet};
use crate::format::Limits;

/// Optional fixed-function features a device may expose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DeviceFeatures {
    /// Anisotropic filtering is available.
    pub sampler_anisotropy: bool,
    /// Non-solid fill modes are available.
    pub fill_mode_non_solid: bool,
    /// Line widths other than 1.0 are available.
    pub wide_lines: bool,
}

impl DeviceFeatures {
    /// Returns whether `self` has every feature set on `required`.
    pub const fn covers(self, required: Self) -> bool {
        (!required.sampler_anisotropy || self.sampler_anisotropy)
            && (!required.fill_mode_non_solid || self.fill_mode_non_solid)
            && (!required.wide_lines || self.wide_lines)
    }
}

/// Queues the caller needs when opening a device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorkQueueHints {
    /// The device must expose a graphics queue.
    pub needs_graphics: bool,
    /// The device must expose a compute queue.
    pub needs_compute: bool,
    /// The device must expose a transfer queue.
    pub needs_copy: bool,
}

/// Requirements checked before a caller accepts an adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceDescriptor {
    /// Fixed-function features the device must support.
    pub required_features: DeviceFeatures,
    /// Queue families the device must expose.
    pub work_queues: WorkQueueHints,
    /// Optional feature bits the device must support.
    pub required_extensions: FeatureSet,
    /// Limits the device must meet.
    pub required_limits: Limits,
}

impl DeviceDescriptor {
    /// Requirements for a graphics device with a copy queue.
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
pub struct AdapterId(
    /// Position of this adapter in the enumerated list.
    pub u32,
);

/// Coarse class of GPU the adapter reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareFamily {
    /// A discrete GPU with its own memory.
    DiscreteGpu,
    /// A GPU integrated with the CPU.
    IntegratedGpu,
    /// A virtualized GPU.
    VirtualGpu,
    /// A software implementation of the graphics API.
    Cpu,
    /// A device that does not match the other classes.
    Other,
}

/// Native API the adapter is exposed through.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum GpuApiBackend {
    /// Adapter accessed through Vulkan.
    Vulkan,
    /// Adapter accessed through Metal.
    Metal,
    /// Adapter accessed through Direct3D 12.
    Dx12,
    /// Adapter accessed through OpenGL.
    OpenGl,
    /// Adapter accessed through WebGL.
    WebGl,
    /// Adapter accessed through WebGPU.
    WebGpu,
    /// API that does not match the other backends.
    Unknown,
}

/// How many queues of each kind the adapter exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AdapterQueueSupport {
    /// Number of graphics queues.
    pub graphics_queue_count: u32,
    /// Number of compute queues.
    pub compute_queue_count: u32,
    /// Number of transfer queues.
    pub transfer_queue_count: u32,
}

/// What an adapter can do before a device is opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AdapterCapabilities {
    /// The adapter can present to a window surface.
    pub supports_window_presentation: bool,
    /// Queue counts by family.
    pub queue_support: AdapterQueueSupport,
    /// Fixed-function features the adapter supports.
    pub available_features: DeviceFeatures,
    /// Numeric limits the adapter supports.
    pub limits: Limits,
    /// Optional feature bits the adapter supports.
    pub extensions: FeatureSet,
    /// Core HAL features and whether each is native, emulated, or absent.
    pub core: CoreCaps,
}

/// Identity and capabilities of one enumerated adapter.
#[derive(Debug, Clone)]
pub struct AdapterInfo {
    /// Index of this adapter in the enumerated list.
    pub id: AdapterId,
    /// Human-readable device name.
    pub name: String,
    /// Discrete, integrated, virtual, CPU, or other class.
    pub hardware_family: HardwareFamily,
    /// Vendor identifier reported by the graphics API.
    pub vendor_id: u32,
    /// Device identifier reported by the graphics API.
    pub device_id: u32,
    /// Name of the driver serving this adapter.
    pub driver_name: String,
    /// Driver version or other driver-supplied detail.
    pub driver_info: String,
    /// Graphics API this adapter speaks.
    pub api_backend: GpuApiBackend,
    /// Features, limits, and queues the adapter exposes.
    pub capabilities: AdapterCapabilities,
}
