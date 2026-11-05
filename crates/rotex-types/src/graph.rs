//! Explicit GPU commands a backend executes without choosing scene policy.

use crate::caps::FeatureId;
use crate::frame::PassDescriptor;
use crate::resource::ShaderStageFlags;
use crate::resource::geometry::IndexFormat;
use crate::resource::ids::{
    BindGroupId, BufferId, ComputePipelineId, GraphicsPipelineId, MeshId, QuerySetId, TextureId,
};

/// Destination state a resource must reach before the next use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ResourceState {
    /// The resource is the source of a copy.
    CopySrc,
    /// The resource is the destination of a copy.
    CopyDst,
    /// A shader reads the resource.
    ShaderRead,
    /// A shader writes the resource.
    ShaderWrite,
    /// The resource is a color render attachment.
    ColorTarget,
    /// The resource is a depth or stencil attachment.
    DepthTarget,
    /// The resource is bound as a vertex buffer.
    VertexBuffer,
    /// The last shader write is visible to vertex fetch and to a later shader read.
    ///
    /// Use this after a compute write that the same frame draws and the next frame
    /// reads. The barrier publishes that write without waiting for earlier vertex fetches.
    VertexShaderRead,
    /// The resource is bound as an index buffer.
    IndexBuffer,
    /// The resource is bound as a uniform buffer.
    Uniform,
    /// The resource holds an indirect draw or dispatch argument.
    IndirectArg,
    /// The resource is the presentation image.
    Present,
}

/// Buffer or texture named by a [`RhiCommand::Barrier`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BarrierResource {
    /// The barrier applies to this buffer.
    Buffer(BufferId),
    /// The barrier applies to this texture.
    Texture(TextureId),
}

/// One explicit GPU operation in a frame's command list.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum RhiCommand {
    /// Selects the in-flight frame slot. The caller owns the ring index.
    BeginFrame {
        /// Index of the frame slot to wait on and record into.
        frame_index: u32,
    },
    /// Acquires the presentation target for the current frame slot.
    AcquirePresentationTarget,
    /// Uploads bytes into a buffer from the CPU.
    WriteBuffer {
        /// Buffer that receives the bytes.
        buffer: BufferId,
        /// Byte offset at which [`Self::WriteBuffer::data`] is written.
        offset: u64,
        /// Bytes copied into the buffer.
        data: Vec<u8>,
    },
    /// Opens a graphics pass using the caller's attachment state.
    BeginRenderPass {
        /// Color, depth, and buffer usage for the pass.
        pass: PassDescriptor,
    },
    /// Closes the graphics pass opened by [`Self::BeginRenderPass`].
    EndRenderPass,
    /// Binds consecutive bind groups starting at a set index.
    BindGroups {
        /// First bind-group set index to update.
        first_slot: u32,
        /// Bind groups written from [`Self::BindGroups::first_slot`] upward.
        bind_groups: Vec<BindGroupId>,
        /// Dynamic uniform offsets, in binding order.
        dynamic_offsets: Vec<u32>,
    },
    /// Sets push-constant bytes for the given shader stages.
    PushConstants {
        /// Stages that read the bytes.
        stages: ShaderStageFlags,
        /// Byte offset into the push-constant range.
        offset: u32,
        /// Bytes written at [`Self::PushConstants::offset`].
        data: Vec<u8>,
    },
    /// Binds the vertex streams stored on a mesh.
    SetVertexBuffers {
        /// Mesh whose vertex streams are bound.
        mesh: MeshId,
        /// First vertex-buffer binding those streams occupy.
        first_binding: u32,
    },
    /// Binds the index buffer stored on a mesh.
    SetIndexBuffer {
        /// Mesh whose index buffer is bound.
        mesh: MeshId,
    },
    /// Draws indexed primitives from the bound index buffer.
    DrawIndexed {
        /// Number of indices to read.
        index_count: u32,
        /// Number of instances to draw.
        instance_count: u32,
        /// First index in the bound index buffer.
        first_index: u32,
        /// Signed offset added to each vertex index.
        vertex_offset: i32,
        /// First instance id passed to the shader.
        first_instance: u32,
    },
    /// Submits the recorded frame and optionally presents.
    SubmitFrame {
        /// When true, present the acquired presentation target.
        present: bool,
    },
    /// Opens a compute pass.
    BeginComputePass {
        /// Debug name for the pass.
        name: String,
    },
    /// Closes the compute pass opened by [`Self::BeginComputePass`].
    EndComputePass,
    /// Binds the compute pipeline used by the next dispatch.
    SetComputePipeline {
        /// Pipeline to bind.
        pipeline: ComputePipelineId,
    },
    /// Dispatches a compute grid.
    Dispatch {
        /// Number of workgroups along x, y, and z.
        workgroup_count: [u32; 3],
    },
    /// Sets the viewport rectangle and depth range.
    SetViewport {
        /// Left edge of the viewport in pixels.
        x: f32,
        /// Top edge of the viewport in pixels.
        y: f32,
        /// Width of the viewport in pixels.
        width: f32,
        /// Height of the viewport in pixels.
        height: f32,
        /// Minimum depth mapped into the viewport.
        min_depth: f32,
        /// Maximum depth mapped into the viewport.
        max_depth: f32,
    },
    /// Sets the scissor rectangle in pixels.
    SetScissor {
        /// Left edge of the scissor rectangle.
        x: u32,
        /// Top edge of the scissor rectangle.
        y: u32,
        /// Width of the scissor rectangle.
        width: u32,
        /// Height of the scissor rectangle.
        height: u32,
    },
    /// Draws non-indexed primitives from the bound vertex buffers.
    Draw {
        /// Number of vertices to process.
        vertex_count: u32,
        /// Number of instances to draw.
        instance_count: u32,
        /// First vertex in the bound vertex buffers.
        first_vertex: u32,
        /// First instance id passed to the shader.
        first_instance: u32,
    },
    /// Copies a byte range from one buffer to another.
    CopyBufferToBuffer {
        /// Buffer that supplies the bytes.
        src: BufferId,
        /// Byte offset into [`Self::CopyBufferToBuffer::src`].
        src_offset: u64,
        /// Buffer that receives the bytes.
        dst: BufferId,
        /// Byte offset into [`Self::CopyBufferToBuffer::dst`].
        dst_offset: u64,
        /// Number of bytes to copy.
        size: u64,
    },
    /// Copies rows of texels from a buffer into a texture.
    CopyBufferToTexture {
        /// Buffer that supplies the rows.
        src: BufferId,
        /// Byte offset of the first row in [`Self::CopyBufferToTexture::src`].
        src_offset: u64,
        /// Texture that receives the texels.
        dst: TextureId,
        /// Stride between rows in the source buffer, in bytes.
        bytes_per_row: u32,
        /// Width of the copied region in texels.
        width: u32,
        /// Height of the copied region in texels.
        height: u32,
    },
    /// Copies a texture region into a buffer.
    CopyTextureToBuffer {
        /// Texture that supplies the texels.
        src: TextureId,
        /// Buffer that receives the rows.
        dst: BufferId,
        /// Byte offset of the first row in [`Self::CopyTextureToBuffer::dst`].
        dst_offset: u64,
        /// Stride between rows in the destination buffer, in bytes.
        bytes_per_row: u32,
        /// Width of the copied region in texels.
        width: u32,
        /// Height of the copied region in texels.
        height: u32,
    },
    /// Moves a resource to a destination state before the next use.
    Barrier {
        /// Buffer or texture whose state changes.
        resource: BarrierResource,
        /// State required by the next use.
        state: ResourceState,
    },
    /// Writes a timestamp into a query slot.
    WriteTimestamp {
        /// Query set that stores the timestamp.
        query_set: QuerySetId,
        /// Slot inside [`Self::WriteTimestamp::query_set`].
        index: u32,
    },
    /// Resolves query results into a buffer.
    ResolveQuerySet {
        /// Query set whose results are copied.
        query_set: QuerySetId,
        /// First query slot to resolve.
        first_query: u32,
        /// Number of consecutive query slots to resolve.
        query_count: u32,
        /// Buffer that receives the resolved values.
        destination: BufferId,
        /// Byte offset of the first resolved value in [`Self::ResolveQuerySet::destination`].
        offset: u64,
    },
    /// Binds the graphics pipeline used by the next draw.
    SetGraphicsPipeline {
        /// Pipeline to bind.
        pipeline: GraphicsPipelineId,
    },
    /// Binds one vertex buffer at a binding slot.
    BindVertexBuffer {
        /// Buffer that supplies vertex data.
        buffer: BufferId,
        /// Vertex-buffer binding index.
        binding: u32,
        /// Byte offset of the first vertex in [`Self::BindVertexBuffer::buffer`].
        offset: u64,
    },
    /// Binds an index buffer.
    BindIndexBuffer {
        /// Buffer that supplies indices.
        buffer: BufferId,
        /// Byte offset of the first index in [`Self::BindIndexBuffer::buffer`].
        offset: u64,
        /// Integer width of each index.
        format: IndexFormat,
    },
    /// Carries a backend-specific opcode for an optional feature.
    Extension {
        /// Feature that interprets [`Self::Extension::opcode`].
        feature: FeatureId,
        /// Feature-defined operation code.
        opcode: u32,
        /// Feature-defined payload bytes.
        payload: Vec<u8>,
    },
}
