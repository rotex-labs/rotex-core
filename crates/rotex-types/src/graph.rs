use crate::frame::{ComputePassDescriptor, PassDescriptor};
use crate::resource::ids::{BindGroupId, BufferId, MaterialId, MeshId};
use crate::resource::{AccessType, ShaderStageFlags};

#[derive(Debug, Clone)]
pub enum RhiCommand {
    /// Select the in-flight frame slot. User owns the ring index.
    BeginFrame {
        frame_index: u32,
    },
    /// Acquire the next swapchain image for the current frame slot.
    AcquireSwapchainImage,
    /// CPU-side buffer upload (staging copy or mapped write in HAL).
    WriteBuffer {
        buffer: BufferId,
        offset: u64,
        data: Vec<u8>,
    },
    TransitionBuffer {
        buffer: BufferId,
        from: AccessType,
        to: AccessType,
    },
    DispatchCompute(ComputePassDescriptor),
    BeginRenderPass {
        pass: PassDescriptor,
        image_index: u32,
    },
    EndRenderPass,
    BindGraphicsPipeline {
        material: MaterialId,
        mesh: MeshId,
        depth_enabled: bool,
    },
    BindDescriptorSets {
        first_set: u32,
        bind_groups: Vec<BindGroupId>,
        dynamic_offsets: Vec<u32>,
    },
    PushConstants {
        stages: ShaderStageFlags,
        offset: u32,
        data: Vec<u8>,
    },
    SetVertexBuffers {
        mesh: MeshId,
        first_binding: u32,
    },
    SetIndexBuffer {
        mesh: MeshId,
    },
    DrawIndexed {
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    },
    SubmitFrame {
        present: bool,
    },
}
