//! Platform vertical: adapter enumeration and backend creation.
//!
//! Backends implement [`GpuPlatform`]. User code selects a backend via a type alias
//! (e.g. `type AppGpuPlatform = rotex_vulkan::VulkanGpuPlatform`) and calls the
//! generic helpers here. For direct access to backend internals, hold the concrete
//! bridge type returned from `create_backend` — [`GraphicsContext`] is optional.

use crate::backend::GpuBackend;
use crate::error::{Error, ErrorKind};
use crate::frontend::GraphicsContext;
use rotex_types::{AdapterId, AdapterInfo, DeviceDescriptor, PlatformDescriptor};

#[async_trait::async_trait]
pub trait GpuPlatform: Sized {
    type Backend: GpuBackend;

    async fn new(platform_descriptor: PlatformDescriptor) -> Result<Self, Error>;

    async fn enumerate_adapters(&self) -> Result<Vec<AdapterInfo>, Error>;

    async fn create_backend(
        self,
        adapter_id: AdapterId,
        device_descriptor: DeviceDescriptor,
    ) -> Result<Self::Backend, Error>;
}

pub async fn enumerate_adapters<G: GpuPlatform>(
    platform_descriptor: PlatformDescriptor,
) -> Result<(G, Vec<AdapterInfo>), Error> {
    let platform = G::new(platform_descriptor).await?;
    let adapters = platform.enumerate_adapters().await?;
    Ok((platform, adapters))
}

pub async fn create_backend<G: GpuPlatform>(
    platform: G,
    adapter_id: AdapterId,
    device_descriptor: DeviceDescriptor,
) -> Result<G::Backend, Error> {
    platform.create_backend(adapter_id, device_descriptor).await
}

pub async fn create_backend_with_selection<G, F>(
    platform_descriptor: PlatformDescriptor,
    device_descriptor: DeviceDescriptor,
    select: F,
) -> Result<G::Backend, Error>
where
    G: GpuPlatform,
    F: FnOnce(&[AdapterInfo]) -> Option<AdapterId>,
{
    let (platform, adapters) = enumerate_adapters::<G>(platform_descriptor).await?;
    let adapter_id =
        select(&adapters).ok_or_else(|| Error::fatal(ErrorKind::NoCompatibleDevice))?;
    create_backend(platform, adapter_id, device_descriptor).await
}

pub async fn create_graphics_context<G, F>(
    platform_descriptor: PlatformDescriptor,
    device_descriptor: DeviceDescriptor,
    select: F,
) -> Result<GraphicsContext, Error>
where
    G: GpuPlatform,
    G::Backend: GpuBackend + 'static,
    F: FnOnce(&[AdapterInfo]) -> Option<AdapterId>,
{
    let backend =
        create_backend_with_selection::<G, F>(platform_descriptor, device_descriptor, select)
            .await?;
    Ok(GraphicsContext::new(Box::new(backend)))
}

/// Pure compatibility check for filtering adapters before user selection.
/// This is not selection or scoring policy.
pub fn adapter_meets_requirements(info: &AdapterInfo, desc: &DeviceDescriptor) -> bool {
    if !info
        .capabilities
        .available_features
        .covers(desc.required_features)
    {
        return false;
    }
    if !info
        .capabilities
        .extensions
        .contains(desc.required_extensions)
    {
        return false;
    }
    if !info.capabilities.limits.satisfies(desc.required_limits) {
        return false;
    }

    if desc.work_queues.needs_graphics && info.capabilities.queue_support.graphics_queue_count == 0
    {
        return false;
    }
    if desc.work_queues.needs_compute && info.capabilities.queue_support.compute_queue_count == 0 {
        return false;
    }
    if desc.work_queues.needs_copy && info.capabilities.queue_support.transfer_queue_count == 0 {
        return false;
    }

    true
}

/// Window presentation requires adapter support when attaching a surface.
pub fn adapter_supports_window_presentation(info: &AdapterInfo) -> bool {
    info.capabilities.supports_window_presentation
}
