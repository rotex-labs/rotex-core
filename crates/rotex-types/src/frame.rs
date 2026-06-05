//! Frame and pass descriptors for multi-pass rendering and compute.
//!
//! # Shader descriptor set convention
//!
//! | Set | Binding | Content | Bind frequency |
//! |-----|---------|---------|----------------|
//! | 0 | 0 | `view` + `projection` (128-byte UBO) | once per pass |
//! | 1 | 0 | albedo `sampler2D` / combined image sampler | per material change |
//! | 2 | 0 | `model` matrix (dynamic UBO, 64-byte stride) | set bound once per pass; dynamic byte offset per draw |
//!
//! # Multi-pass example
//!
//! ```rust
//! use rotex_types::{ColorAttachmentLoad, FrameDescriptor, FramePass, PassDescriptor};
//!
//! let frame = FrameDescriptor::new(vec![
//!     FramePass::Graphics(
//!         PassDescriptor::new("scene")
//!             .with_clear_color([0.02, 0.02, 0.04, 1.0])
//!             .with_clear_depth(Some(1.0)),
//!     ),
//!     FramePass::Graphics(
//!         PassDescriptor::new("ui").with_color_load(ColorAttachmentLoad::Load),
//!     ),
//! ]);
//! ```

use crate::resource::{BufferUsageIntent, ComputePipelineId, TextureId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorAttachmentLoad {
    Clear,
    Load,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DepthAttachmentLoad {
    Clear,
    Load,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassColorTarget {
    Swapchain,
    Texture(TextureId),
}

#[derive(Debug, Clone)]
pub struct PassDescriptor {
    pub name: String,
    pub color_target: PassColorTarget,
    pub color_load: ColorAttachmentLoad,
    pub clear_color: [f32; 4],
    pub depth_load: DepthAttachmentLoad,
    pub clear_depth: f32,
    pub instance_indices: Vec<usize>,
    pub buffer_intents: Vec<BufferUsageIntent>,
}

impl PassDescriptor {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            color_target: PassColorTarget::Swapchain,
            color_load: ColorAttachmentLoad::Clear,
            clear_color: [0.04, 0.05, 0.08, 1.0],
            depth_load: DepthAttachmentLoad::None,
            clear_depth: 1.0,
            instance_indices: Vec::new(),
            buffer_intents: Vec::new(),
        }
    }

    pub fn uses_depth_attachment(&self) -> bool {
        !matches!(self.depth_load, DepthAttachmentLoad::None)
    }

    pub fn with_clear_color(mut self, clear_color: [f32; 4]) -> Self {
        self.clear_color = clear_color;
        self.color_load = ColorAttachmentLoad::Clear;
        self
    }

    pub fn with_clear_depth(mut self, clear_depth: Option<f32>) -> Self {
        match clear_depth {
            Some(depth) => {
                self.depth_load = DepthAttachmentLoad::Clear;
                self.clear_depth = depth;
            }
            None => {
                self.depth_load = DepthAttachmentLoad::None;
            }
        }
        self
    }

    pub fn with_color_load(mut self, color_load: ColorAttachmentLoad) -> Self {
        self.color_load = color_load;
        self
    }

    pub fn with_depth_load(mut self, depth_load: DepthAttachmentLoad) -> Self {
        self.depth_load = depth_load;
        self
    }

    pub fn with_color_target(mut self, color_target: PassColorTarget) -> Self {
        self.color_target = color_target;
        self
    }

    pub fn with_instance_indices(mut self, instance_indices: Vec<usize>) -> Self {
        self.instance_indices = instance_indices;
        self
    }

    pub fn with_buffer_intents(mut self, buffer_intents: Vec<BufferUsageIntent>) -> Self {
        self.buffer_intents = buffer_intents;
        self
    }

    pub fn into_frame_pass(self) -> FramePass {
        FramePass::Graphics(self)
    }
}

#[derive(Debug, Clone)]
pub struct ComputePassDescriptor {
    pub name: String,
    pub pipeline: ComputePipelineId,
    pub workgroup_count: [u32; 3],
    pub buffer_intents: Vec<BufferUsageIntent>,
}

impl ComputePassDescriptor {
    pub fn new(
        name: impl Into<String>,
        pipeline: ComputePipelineId,
        workgroup_count: [u32; 3],
    ) -> Self {
        Self {
            name: name.into(),
            pipeline,
            workgroup_count,
            buffer_intents: Vec::new(),
        }
    }

    pub fn with_buffer_intents(mut self, buffer_intents: Vec<BufferUsageIntent>) -> Self {
        self.buffer_intents = buffer_intents;
        self
    }

    pub fn into_frame_pass(self) -> FramePass {
        FramePass::Compute(self)
    }
}

#[derive(Debug, Clone)]
pub enum FramePass {
    Graphics(PassDescriptor),
    Compute(ComputePassDescriptor),
}

#[derive(Debug, Clone)]
pub struct FrameDescriptor {
    pub passes: Vec<FramePass>,
}

impl FrameDescriptor {
    pub fn new(passes: Vec<FramePass>) -> Self {
        Self { passes }
    }

    pub fn new_graphics(passes: Vec<PassDescriptor>) -> Self {
        Self {
            passes: passes.into_iter().map(FramePass::Graphics).collect(),
        }
    }

    pub fn single_pass(clear_color: [f32; 4], clear_depth: Option<f32>) -> Self {
        Self::new_graphics(vec![
            PassDescriptor::new("main")
                .with_clear_color(clear_color)
                .with_clear_depth(clear_depth),
        ])
    }
}
