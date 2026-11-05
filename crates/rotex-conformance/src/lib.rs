#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! Capability-driven checks for a [`GpuBackend`].
//!
//! Backends call [`run`] from their own tests. A missing device is the caller's
//! concern: this function assumes the backend was already created. Checks read
//! [`rotex_types::CoreCaps`] and submit [`rotex_types::RhiCommand`] values through
//! the [`rotex_core`] backend trait.

use rotex_core::{Error, GpuBackend};
use rotex_types::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferDescriptor, BufferId,
    BufferUsage, ColorAttachmentLoad, PassColorTarget, PassDescriptor, ResourceBatchCreate,
    ResourceBatchUpdate, ResourceCreateDescriptor, ResourceHandle, ResourceState,
    ResourceUpdateDescriptor, RhiCommand, ShaderStageFlags, Support, TextureDescriptor,
    TextureFormat,
};

/// Runs the capability checks for a backend that was already created.
pub fn run(backend: &mut impl GpuBackend) -> Result<(), Error> {
    let caps = backend.capabilities();
    duplicate_create_and_destroy(backend)?;
    barrier(backend, caps.core.barriers)?;
    if caps.core.push_constants == Support::Absent {
        let error = backend.execute(&[RhiCommand::PushConstants {
            stages: ShaderStageFlags::VERTEX,
            offset: 0,
            data: vec![0, 0, 0, 0],
        }]);
        if error.is_ok() {
            return Err(Error::unsupported("absent push constants were accepted"));
        }
    }
    if caps.core.compute == Support::Absent {
        let error = backend.execute(&[RhiCommand::Dispatch {
            workgroup_count: [1, 1, 1],
        }]);
        if error.is_ok() {
            return Err(Error::unsupported("absent compute dispatch was accepted"));
        }
    }
    if caps.core.storage_buffers == Support::Absent {
        let error = backend.create_resources(ResourceBatchCreate::new(vec![
            ResourceCreateDescriptor::BindGroupLayout {
                id: 8,
                layout: BindGroupLayoutDescriptor {
                    set: 0,
                    entries: vec![BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStageFlags::COMPUTE,
                        ty: BindingType::StorageBuffer,
                        readonly: false,
                    }],
                },
            },
        ]));
        if error.is_ok() {
            return Err(Error::unsupported(
                "absent storage buffer binding was accepted",
            ));
        }
    }
    if caps.core.render_to_texture == Support::Native
        && caps.core.readback == Support::Native
        && caps.core.copies == Support::Native
    {
        clear_and_read(backend)?;
    }
    Ok(())
}

fn duplicate_create_and_destroy(backend: &mut impl GpuBackend) -> Result<(), Error> {
    let created = backend.create_resources(ResourceBatchCreate::new(vec![buffer_create(1)]))?;
    let again = backend.create_resources(ResourceBatchCreate::new(vec![buffer_create(1)]));
    if again.is_ok() {
        return Err(Error::unsupported("duplicate resource id was accepted"));
    }
    let handle = created
        .handles
        .first()
        .cloned()
        .ok_or_else(|| Error::unsupported("create did not return a handle"))?;
    backend.update_resources(ResourceBatchUpdate::new(vec![
        ResourceUpdateDescriptor::Destroy { handle },
    ]))?;
    Ok(())
}

fn barrier(backend: &mut impl GpuBackend, support: Support) -> Result<(), Error> {
    let _ = backend.create_resources(ResourceBatchCreate::new(vec![buffer_create(2)]));
    let result = backend.execute(&[
        RhiCommand::BeginFrame { frame_index: 0 },
        RhiCommand::Barrier {
            resource: rotex_types::BarrierResource::Buffer(BufferId(2)),
            state: ResourceState::ShaderRead,
        },
        RhiCommand::SubmitFrame { present: false },
    ]);
    match support {
        Support::Native | Support::Emulated => result,
        Support::Absent => {
            if result.is_ok() {
                Err(Error::unsupported("absent barriers were accepted"))
            } else {
                Ok(())
            }
        }
    }
}

fn clear_and_read(backend: &mut impl GpuBackend) -> Result<(), Error> {
    let created = backend.create_resources(ResourceBatchCreate::new(vec![
        ResourceCreateDescriptor::Texture {
            id: 3,
            texture: TextureDescriptor {
                width: 1,
                height: 1,
                format: TextureFormat::Rgba8Unorm,
                data: vec![0, 0, 0, 0],
                render_attachment: true,
            },
        },
    ]))?;
    let texture = created.handles.iter().find_map(|handle| match handle {
        ResourceHandle::Texture(id) => Some(*id),
        _ => None,
    });
    let Some(texture) = texture else {
        return Err(Error::unsupported("texture handle was not returned"));
    };
    backend.execute(&[
        RhiCommand::BeginFrame { frame_index: 0 },
        RhiCommand::BeginRenderPass {
            pass: PassDescriptor::new("clear")
                .with_color_target(PassColorTarget::Texture(texture))
                .with_clear_color([1.0, 0.0, 0.0, 1.0])
                .with_color_load(ColorAttachmentLoad::Clear),
        },
        RhiCommand::EndRenderPass,
        RhiCommand::SubmitFrame { present: false },
    ])?;
    let readback = backend.read_texture(texture)?;
    let pixel = readback.data.first().copied().unwrap_or(0);
    if pixel < 250 {
        return Err(Error::unsupported("cleared texture did not read back red"));
    }
    Ok(())
}

fn buffer_create(id: u64) -> ResourceCreateDescriptor {
    ResourceCreateDescriptor::Buffer {
        id,
        buffer: BufferDescriptor {
            size: 256,
            usage: BufferUsage::Uniform,
            usages: rotex_types::BufferUsages::UNIFORM,
            memory_location: rotex_types::MemoryLocation::CpuToGpu,
            initial_data: None,
        },
    }
}
