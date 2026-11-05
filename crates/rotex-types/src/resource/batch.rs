//! Create and update batches submitted to a backend.

use crate::resource::descriptors::{
    BindGroupDescriptor, BindGroupLayoutDescriptor, BufferDescriptor, ComputePipelineDescriptor,
    GraphicsPipelineDescriptor, MaterialDescriptor, MeshDescriptor, QuerySetDescriptor,
    SamplerDescriptor, TextureDescriptor,
};
use crate::resource::geometry::IndexFormat;
use crate::resource::ids::{
    BindGroupId, BindGroupLayoutId, BufferId, ComputePipelineId, GraphicsPipelineId, MaterialId,
    MeshId, QuerySetId, SamplerId, TextureId,
};

/// One resource to create, tagged with the raw id stored on its handle.
#[derive(Debug, Clone)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
pub enum ResourceCreateDescriptor {
    /// Creates a mesh.
    Mesh {
        /// Raw id copied into the created mesh handle.
        id: u64,
        /// Vertex streams and index data to upload.
        mesh: MeshDescriptor,
    },
    /// Creates a texture.
    Texture {
        /// Raw id copied into the created texture handle.
        id: u64,
        /// Size, format, and initial texels.
        texture: TextureDescriptor,
    },
    /// Creates a material.
    Material {
        /// Raw id copied into the created material handle.
        id: u64,
        /// Shaders and fixed state.
        material: MaterialDescriptor,
    },
    /// Creates a buffer.
    Buffer {
        /// Raw id copied into the created buffer handle.
        id: u64,
        /// Size, usage, and optional initial bytes.
        buffer: BufferDescriptor,
    },
    /// Creates a compute pipeline.
    ComputePipeline {
        /// Raw id copied into the created pipeline handle.
        id: u64,
        /// Shader and storage bindings.
        pipeline: ComputePipelineDescriptor,
    },
    /// Creates a bind-group layout.
    BindGroupLayout {
        /// Raw id copied into the created layout handle.
        id: u64,
        /// Set index and binding slots.
        layout: BindGroupLayoutDescriptor,
    },
    /// Creates a bind group.
    BindGroup {
        /// Raw id copied into the created bind-group handle.
        id: u64,
        /// Layout and resources to write.
        group: BindGroupDescriptor,
    },
    /// Creates a sampler.
    Sampler {
        /// Raw id copied into the created sampler handle.
        id: u64,
        /// Filter and wrap mode.
        sampler: SamplerDescriptor,
    },
    /// Creates a graphics pipeline.
    GraphicsPipeline {
        /// Raw id copied into the created pipeline handle.
        id: u64,
        /// Shaders and fixed state.
        pipeline: GraphicsPipelineDescriptor,
    },
    /// Creates a query set.
    QuerySet {
        /// Raw id copied into the created query-set handle.
        id: u64,
        /// Slot count and query kind.
        query_set: QuerySetDescriptor,
    },
}

/// A group of resources to create in one backend call.
#[derive(Debug, Clone)]
pub struct ResourceBatchCreate {
    /// Create requests applied together.
    pub resources: Vec<ResourceCreateDescriptor>,
}

impl ResourceBatchCreate {
    /// Builds a batch from create requests.
    pub fn new(resources: Vec<ResourceCreateDescriptor>) -> Self {
        Self { resources }
    }
}

/// Handle returned after a resource is created.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ResourceHandle {
    /// Created mesh.
    Mesh(MeshId),
    /// Created texture.
    Texture(TextureId),
    /// Created material.
    Material(MaterialId),
    /// Created buffer.
    Buffer(BufferId),
    /// Created compute pipeline.
    ComputePipeline(ComputePipelineId),
    /// Created bind-group layout.
    BindGroupLayout(BindGroupLayoutId),
    /// Created bind group.
    BindGroup(BindGroupId),
    /// Created sampler.
    Sampler(SamplerId),
    /// Created graphics pipeline.
    GraphicsPipeline(GraphicsPipelineId),
    /// Created query set.
    QuerySet(QuerySetId),
}

/// Handles returned for one [`ResourceBatchCreate`], in request order.
#[derive(Debug, Clone)]
pub struct CreatedResources {
    /// One handle per create request, in the same order.
    pub handles: Vec<ResourceHandle>,
}

/// One change to apply to an existing resource.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ResourceUpdateDescriptor {
    /// Replaces vertex streams and indices on an existing mesh.
    Mesh {
        /// Mesh to replace.
        id: MeshId,
        /// New vertex streams.
        vertex_streams: Vec<crate::resource::descriptors::VertexStream>,
        /// New index bytes.
        index_data: Vec<u8>,
        /// Width of each index in [`Self::Mesh::index_data`].
        index_format: IndexFormat,
        /// Number of indices the mesh draws.
        index_count: u32,
    },
    /// Supplies replacement vertex bytes for an existing mesh.
    MeshVertices {
        /// Mesh whose vertices are replaced.
        id: MeshId,
        /// Replacement vertex bytes.
        vertex_data: Vec<u8>,
    },
    /// Replaces the texel bytes of an existing texture.
    Texture {
        /// Texture whose texels are replaced.
        id: TextureId,
        /// Replacement texel bytes.
        data: Vec<u8>,
    },
    /// Replaces selected fields on an existing material.
    Material {
        /// Material to update.
        id: MaterialId,
        /// New depth-test flag; `None` leaves the current value.
        enable_depth: Option<bool>,
        /// New texture binding; `None` leaves it, and `Some(None)` clears it.
        texture: Option<Option<TextureId>>,
    },
    /// Supplies replacement bytes for an existing buffer.
    Buffer {
        /// Buffer whose bytes are replaced.
        id: BufferId,
        /// Replacement bytes.
        data: Vec<u8>,
    },
    /// Destroys the GPU object named by this handle.
    Destroy {
        /// Object to destroy.
        handle: ResourceHandle,
    },
}

/// A group of resource updates to apply in one backend call.
#[derive(Debug, Clone)]
pub struct ResourceBatchUpdate {
    /// Updates applied together.
    pub updates: Vec<ResourceUpdateDescriptor>,
}

impl ResourceBatchUpdate {
    /// Builds a batch from update requests.
    pub fn new(updates: Vec<ResourceUpdateDescriptor>) -> Self {
        Self { updates }
    }
}
