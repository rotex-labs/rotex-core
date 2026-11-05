//! Recording sugar over an explicit [`RhiCommand`] list.
//!
//! `B` is the backend type so a later phase can gate methods with [`crate::Supports`].
//! Recording does not call the backend.

use std::marker::PhantomData;

use rotex_types::{
    BarrierResource, BindGroupId, BufferId, ComputePipelineId, FeatureId, GraphicsPipelineId,
    IndexFormat, MeshId, PassDescriptor, QuerySetId, ResourceState, RhiCommand, ShaderStageFlags,
    TextureId,
};

/// Builder that appends explicit [`RhiCommand`] values.
pub struct CommandEncoder<B = ()> {
    commands: Vec<RhiCommand>,
    _backend: PhantomData<fn() -> B>,
}

impl<B> CommandEncoder<B> {
    /// Creates an empty encoder.
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            _backend: PhantomData,
        }
    }

    /// Returns the recorded commands.
    pub fn finish(self) -> Vec<RhiCommand> {
        self.commands
    }

    /// Appends a begin-frame command for `frame_index`.
    pub fn begin_frame(&mut self, frame_index: u32) -> &mut Self {
        self.commands.push(RhiCommand::BeginFrame { frame_index });
        self
    }

    /// Appends a command that acquires the presentation target.
    pub fn acquire_presentation_target(&mut self) -> &mut Self {
        self.commands.push(RhiCommand::AcquirePresentationTarget);
        self
    }

    /// Appends a CPU upload into `buffer` at `offset`.
    pub fn write_buffer(&mut self, buffer: BufferId, offset: u64, data: Vec<u8>) -> &mut Self {
        self.commands.push(RhiCommand::WriteBuffer {
            buffer,
            offset,
            data,
        });
        self
    }

    /// Appends the start of a graphics pass.
    pub fn begin_render_pass(&mut self, pass: PassDescriptor) -> &mut Self {
        self.commands.push(RhiCommand::BeginRenderPass { pass });
        self
    }

    /// Appends the end of the current graphics pass.
    pub fn end_render_pass(&mut self) -> &mut Self {
        self.commands.push(RhiCommand::EndRenderPass);
        self
    }

    /// Appends a bind-group update starting at `first_slot`.
    pub fn bind_groups(
        &mut self,
        first_slot: u32,
        bind_groups: Vec<BindGroupId>,
        dynamic_offsets: Vec<u32>,
    ) -> &mut Self {
        self.commands.push(RhiCommand::BindGroups {
            first_slot,
            bind_groups,
            dynamic_offsets,
        });
        self
    }

    /// Appends push-constant bytes for `stages`.
    pub fn push_constants(
        &mut self,
        stages: ShaderStageFlags,
        offset: u32,
        data: Vec<u8>,
    ) -> &mut Self {
        self.commands.push(RhiCommand::PushConstants {
            stages,
            offset,
            data,
        });
        self
    }

    /// Appends a bind of `mesh` vertex streams at `first_binding`.
    pub fn set_vertex_buffers(&mut self, mesh: MeshId, first_binding: u32) -> &mut Self {
        self.commands.push(RhiCommand::SetVertexBuffers {
            mesh,
            first_binding,
        });
        self
    }

    /// Appends a bind of the index buffer stored on `mesh`.
    pub fn set_index_buffer(&mut self, mesh: MeshId) -> &mut Self {
        self.commands.push(RhiCommand::SetIndexBuffer { mesh });
        self
    }

    /// Appends an indexed draw.
    pub fn draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) -> &mut Self {
        self.commands.push(RhiCommand::DrawIndexed {
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        });
        self
    }

    /// Appends a submit, presenting when `present` is true.
    pub fn submit_frame(&mut self, present: bool) -> &mut Self {
        self.commands.push(RhiCommand::SubmitFrame { present });
        self
    }

    /// Appends the start of a compute pass named `name`.
    pub fn begin_compute_pass(&mut self, name: impl Into<String>) -> &mut Self {
        self.commands
            .push(RhiCommand::BeginComputePass { name: name.into() });
        self
    }

    /// Appends the end of the current compute pass.
    pub fn end_compute_pass(&mut self) -> &mut Self {
        self.commands.push(RhiCommand::EndComputePass);
        self
    }

    /// Appends a bind of a compute `pipeline`.
    pub fn set_compute_pipeline(&mut self, pipeline: ComputePipelineId) -> &mut Self {
        self.commands
            .push(RhiCommand::SetComputePipeline { pipeline });
        self
    }

    /// Appends a compute dispatch of `workgroup_count`.
    pub fn dispatch(&mut self, workgroup_count: [u32; 3]) -> &mut Self {
        self.commands.push(RhiCommand::Dispatch { workgroup_count });
        self
    }

    /// Appends a viewport rectangle and depth range.
    pub fn set_viewport(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) -> &mut Self {
        self.commands.push(RhiCommand::SetViewport {
            x,
            y,
            width,
            height,
            min_depth,
            max_depth,
        });
        self
    }

    /// Appends a scissor rectangle.
    pub fn set_scissor(&mut self, x: u32, y: u32, width: u32, height: u32) -> &mut Self {
        self.commands.push(RhiCommand::SetScissor {
            x,
            y,
            width,
            height,
        });
        self
    }

    /// Appends a non-indexed draw.
    pub fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> &mut Self {
        self.commands.push(RhiCommand::Draw {
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        });
        self
    }

    /// Appends a buffer-to-buffer copy.
    pub fn copy_buffer_to_buffer(
        &mut self,
        src: BufferId,
        src_offset: u64,
        dst: BufferId,
        dst_offset: u64,
        size: u64,
    ) -> &mut Self {
        self.commands.push(RhiCommand::CopyBufferToBuffer {
            src,
            src_offset,
            dst,
            dst_offset,
            size,
        });
        self
    }

    /// Appends a buffer-to-texture copy.
    pub fn copy_buffer_to_texture(
        &mut self,
        src: BufferId,
        src_offset: u64,
        dst: TextureId,
        bytes_per_row: u32,
        width: u32,
        height: u32,
    ) -> &mut Self {
        self.commands.push(RhiCommand::CopyBufferToTexture {
            src,
            src_offset,
            dst,
            bytes_per_row,
            width,
            height,
        });
        self
    }

    /// Appends a texture-to-buffer copy.
    pub fn copy_texture_to_buffer(
        &mut self,
        src: TextureId,
        dst: BufferId,
        dst_offset: u64,
        bytes_per_row: u32,
        width: u32,
        height: u32,
    ) -> &mut Self {
        self.commands.push(RhiCommand::CopyTextureToBuffer {
            src,
            dst,
            dst_offset,
            bytes_per_row,
            width,
            height,
        });
        self
    }

    /// Appends a transition of `resource` into `state`.
    pub fn barrier(&mut self, resource: BarrierResource, state: ResourceState) -> &mut Self {
        self.commands.push(RhiCommand::Barrier { resource, state });
        self
    }

    /// Appends a timestamp write into `query_set` at `index`.
    pub fn write_timestamp(&mut self, query_set: QuerySetId, index: u32) -> &mut Self {
        self.commands
            .push(RhiCommand::WriteTimestamp { query_set, index });
        self
    }

    /// Appends a resolve of query results into `destination`.
    pub fn resolve_query_set(
        &mut self,
        query_set: QuerySetId,
        first_query: u32,
        query_count: u32,
        destination: BufferId,
        offset: u64,
    ) -> &mut Self {
        self.commands.push(RhiCommand::ResolveQuerySet {
            query_set,
            first_query,
            query_count,
            destination,
            offset,
        });
        self
    }

    /// Appends a bind of a graphics `pipeline`.
    pub fn set_graphics_pipeline(&mut self, pipeline: GraphicsPipelineId) -> &mut Self {
        self.commands
            .push(RhiCommand::SetGraphicsPipeline { pipeline });
        self
    }

    /// Appends a bind of one vertex buffer.
    pub fn bind_vertex_buffer(&mut self, buffer: BufferId, binding: u32, offset: u64) -> &mut Self {
        self.commands.push(RhiCommand::BindVertexBuffer {
            buffer,
            binding,
            offset,
        });
        self
    }

    /// Appends a bind of an index buffer.
    pub fn bind_index_buffer(
        &mut self,
        buffer: BufferId,
        offset: u64,
        format: IndexFormat,
    ) -> &mut Self {
        self.commands.push(RhiCommand::BindIndexBuffer {
            buffer,
            offset,
            format,
        });
        self
    }

    /// Appends a feature-specific opcode and payload.
    pub fn extension(&mut self, feature: FeatureId, opcode: u32, payload: Vec<u8>) -> &mut Self {
        self.commands.push(RhiCommand::Extension {
            feature,
            opcode,
            payload,
        });
        self
    }
}

impl<B> Default for CommandEncoder<B> {
    fn default() -> Self {
        Self::new()
    }
}
