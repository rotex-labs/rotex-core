//! Frame and pass descriptors for multi-pass rendering and compute.
//!
//! Pass descriptors are **user-composed** inputs to [`crate::RhiCommand::BeginRenderPass`].
//! The HAL does not interpret scene data or choose draw strategies — the user
//! builds explicit command lists ([`crate::RhiCommand::BindGroups`], [`crate::RhiCommand::DrawIndexed`], etc.)
//! between [`crate::RhiCommand::BeginRenderPass`] and [`crate::RhiCommand::EndRenderPass`].
//!
//! Bind group layouts and set indices are user-defined via [`crate::BindGroupLayoutDescriptor`].
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

/// What happens to the previous color attachment contents at pass start.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorAttachmentLoad {
    /// Replace the attachment with the clear color.
    Clear,
    /// Keep the attachment contents already stored.
    Load,
}

/// What happens to the previous depth attachment contents at pass start.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DepthAttachmentLoad {
    /// Replace the attachment with the clear depth.
    Clear,
    /// Keep the attachment contents already stored.
    Load,
    /// This pass has no depth attachment.
    None,
}

/// What happens to an attachment when the pass ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StoreOp {
    /// Write the attachment back to memory.
    #[default]
    Store,
    /// Discard the attachment contents.
    Discard,
}

/// Where a graphics pass writes its color output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassColorTarget {
    /// The acquired presentation image.
    Presentation,
    /// An offscreen texture.
    Texture(TextureId),
}

/// Load, store, and clear state for one color attachment.
#[derive(Debug, Clone)]
pub struct ColorAttachment {
    /// Image the pass writes.
    pub target: PassColorTarget,
    /// How existing color contents are treated at pass start.
    pub load: ColorAttachmentLoad,
    /// How color contents are treated at pass end.
    pub store: StoreOp,
    /// RGBA clear color used when [`Self::load`] is [`ColorAttachmentLoad::Clear`].
    pub clear_color: [f32; 4],
}

/// Load, store, and clear state for a depth attachment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DepthStencilAttachment {
    /// How existing depth contents are treated at pass start.
    pub load: DepthAttachmentLoad,
    /// How depth contents are treated at pass end.
    pub store: StoreOp,
    /// Clear depth used when [`Self::load`] is [`DepthAttachmentLoad::Clear`].
    pub clear_depth: f32,
}

/// User-composed description of one graphics pass.
#[derive(Debug, Clone)]
pub struct PassDescriptor {
    /// Debug name for the pass.
    pub name: String,
    /// Image the pass writes color into.
    pub color_target: PassColorTarget,
    /// How existing color contents are treated at pass start.
    pub color_load: ColorAttachmentLoad,
    /// RGBA clear color used when [`Self::color_load`] clears.
    pub clear_color: [f32; 4],
    /// How existing depth contents are treated at pass start.
    pub depth_load: DepthAttachmentLoad,
    /// Clear depth used when [`Self::depth_load`] clears.
    pub clear_depth: f32,
    /// Indices into the caller's instance list for this pass.
    pub instance_indices: Vec<usize>,
    /// Buffers this pass reads or writes, and the slots they occupy.
    pub buffer_intents: Vec<BufferUsageIntent>,
    /// How color contents are treated at pass end.
    pub color_store: StoreOp,
    /// How depth contents are treated at pass end.
    pub depth_store: StoreOp,
}

impl PassDescriptor {
    /// Builds a pass that clears the presentation target and has no depth attachment.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            color_target: PassColorTarget::Presentation,
            color_load: ColorAttachmentLoad::Clear,
            clear_color: [0.04, 0.05, 0.08, 1.0],
            depth_load: DepthAttachmentLoad::None,
            clear_depth: 1.0,
            instance_indices: Vec::new(),
            buffer_intents: Vec::new(),
            color_store: StoreOp::Store,
            depth_store: StoreOp::Store,
        }
    }

    /// Builds the color attachment described by this pass.
    pub fn color_attachment(&self) -> ColorAttachment {
        ColorAttachment {
            target: self.color_target,
            load: self.color_load,
            store: self.color_store,
            clear_color: self.clear_color,
        }
    }

    /// Returns the depth attachment when this pass uses one.
    pub fn depth_stencil_attachment(&self) -> Option<DepthStencilAttachment> {
        if self.uses_depth_attachment() {
            Some(DepthStencilAttachment {
                load: self.depth_load,
                store: self.depth_store,
                clear_depth: self.clear_depth,
            })
        } else {
            None
        }
    }

    /// Returns whether the pass loads or clears a depth attachment.
    pub fn uses_depth_attachment(&self) -> bool {
        !matches!(self.depth_load, DepthAttachmentLoad::None)
    }

    /// Sets the clear color and selects a color clear load.
    pub fn with_clear_color(mut self, clear_color: [f32; 4]) -> Self {
        self.clear_color = clear_color;
        self.color_load = ColorAttachmentLoad::Clear;
        self
    }

    /// Sets a depth clear, or drops the depth attachment when `clear_depth` is `None`.
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

    /// Sets how existing color contents are treated at pass start.
    pub fn with_color_load(mut self, color_load: ColorAttachmentLoad) -> Self {
        self.color_load = color_load;
        self
    }

    /// Sets how existing depth contents are treated at pass start.
    pub fn with_depth_load(mut self, depth_load: DepthAttachmentLoad) -> Self {
        self.depth_load = depth_load;
        self
    }

    /// Sets the image the pass writes color into.
    pub fn with_color_target(mut self, color_target: PassColorTarget) -> Self {
        self.color_target = color_target;
        self
    }

    /// Sets indices into the caller's instance list for this pass.
    pub fn with_instance_indices(mut self, instance_indices: Vec<usize>) -> Self {
        self.instance_indices = instance_indices;
        self
    }

    /// Sets the buffers this pass reads or writes.
    pub fn with_buffer_intents(mut self, buffer_intents: Vec<BufferUsageIntent>) -> Self {
        self.buffer_intents = buffer_intents;
        self
    }

    /// Wraps this pass as a graphics [`FramePass`].
    pub fn into_frame_pass(self) -> FramePass {
        FramePass::Graphics(self)
    }
}

/// User-composed description of one compute pass.
#[derive(Debug, Clone)]
pub struct ComputePassDescriptor {
    /// Debug name for the pass.
    pub name: String,
    /// Compute pipeline the pass dispatches.
    pub pipeline: ComputePipelineId,
    /// Number of workgroups along x, y, and z.
    pub workgroup_count: [u32; 3],
    /// Buffers this pass reads or writes, and the slots they occupy.
    pub buffer_intents: Vec<BufferUsageIntent>,
}

impl ComputePassDescriptor {
    /// Builds a compute pass with no buffer intents.
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

    /// Sets the buffers this pass reads or writes.
    pub fn with_buffer_intents(mut self, buffer_intents: Vec<BufferUsageIntent>) -> Self {
        self.buffer_intents = buffer_intents;
        self
    }

    /// Wraps this pass as a compute [`FramePass`].
    pub fn into_frame_pass(self) -> FramePass {
        FramePass::Compute(self)
    }
}

/// One graphics or compute pass inside a [`FrameDescriptor`].
#[derive(Debug, Clone)]
pub enum FramePass {
    /// A graphics pass.
    Graphics(PassDescriptor),
    /// A compute pass.
    Compute(ComputePassDescriptor),
}

/// Ordered passes the caller will record for one frame.
#[derive(Debug, Clone)]
pub struct FrameDescriptor {
    /// Passes in submission order.
    pub passes: Vec<FramePass>,
}

impl FrameDescriptor {
    /// Builds a frame from graphics and compute passes.
    pub fn new(passes: Vec<FramePass>) -> Self {
        Self { passes }
    }

    /// Builds a frame from graphics passes only.
    pub fn new_graphics(passes: Vec<PassDescriptor>) -> Self {
        Self {
            passes: passes.into_iter().map(FramePass::Graphics).collect(),
        }
    }

    /// Builds a one-pass frame that clears color and optional depth.
    pub fn single_pass(clear_color: [f32; 4], clear_depth: Option<f32>) -> Self {
        Self::new_graphics(vec![
            PassDescriptor::new("main")
                .with_clear_color(clear_color)
                .with_clear_depth(clear_depth),
        ])
    }
}
