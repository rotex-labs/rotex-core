use crate::caps::FeatureId;
use crate::frame::{ComputePassDescriptor, PassDescriptor};
use crate::resource::geometry::IndexFormat;
use crate::resource::ids::{
    BindGroupId, BufferId, ComputePipelineId, GraphicsPipelineId, MaterialId, MeshId, QuerySetId,
    TextureId,
};
use crate::resource::{AccessType, ShaderStageFlags};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ResourceState {
    CopySrc,
    CopyDst,
    ShaderRead,
    ShaderWrite,
    ColorTarget,
    DepthTarget,
    VertexBuffer,
    IndexBuffer,
    Uniform,
    IndirectArg,
    Present,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BarrierResource {
    Buffer(BufferId),
    Texture(TextureId),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum RhiCommand {
    /// Select the in-flight frame slot. User owns the ring index.
    BeginFrame {
        frame_index: u32,
    },
    /// Acquire the presentation target for the current frame slot.
    AcquirePresentationTarget,
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
    },
    EndRenderPass,
    BindGraphicsPipeline {
        material: MaterialId,
        mesh: MeshId,
        depth_enabled: bool,
    },
    BindGroups {
        first_slot: u32,
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
    BeginComputePass {
        name: String,
    },
    EndComputePass,
    SetComputePipeline {
        pipeline: ComputePipelineId,
    },
    Dispatch {
        workgroup_count: [u32; 3],
    },
    SetViewport {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    },
    SetScissor {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    Draw {
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    },
    CopyBufferToBuffer {
        src: BufferId,
        src_offset: u64,
        dst: BufferId,
        dst_offset: u64,
        size: u64,
    },
    CopyBufferToTexture {
        src: BufferId,
        src_offset: u64,
        dst: TextureId,
        bytes_per_row: u32,
        width: u32,
        height: u32,
    },
    CopyTextureToBuffer {
        src: TextureId,
        dst: BufferId,
        dst_offset: u64,
        bytes_per_row: u32,
        width: u32,
        height: u32,
    },
    Barrier {
        resource: BarrierResource,
        state: ResourceState,
    },
    WriteTimestamp {
        query_set: QuerySetId,
        index: u32,
    },
    ResolveQuerySet {
        query_set: QuerySetId,
        first_query: u32,
        query_count: u32,
        destination: BufferId,
        offset: u64,
    },
    SetGraphicsPipeline {
        pipeline: GraphicsPipelineId,
    },
    BindVertexBuffer {
        buffer: BufferId,
        binding: u32,
        offset: u64,
    },
    BindIndexBuffer {
        buffer: BufferId,
        offset: u64,
        format: IndexFormat,
    },
    Extension {
        feature: FeatureId,
        opcode: u32,
        payload: Vec<u8>,
    },
}
