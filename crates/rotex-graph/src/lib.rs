#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! Turn declared pass usage into destination-only [`RhiCommand::Barrier`] commands.
//!
//! The graph does not create resources or read GPU objects. A resource with no
//! initial state emits nothing on its first declared use. A later use in a
//! different state emits one barrier carrying only the destination. Pass
//! descriptors and commands come from [`rotex_types`].
//!
//! ```
//! use rotex_graph::{GraphPass, record};
//! use rotex_types::{
//!     BarrierResource, FramePass, PassColorTarget, PassDescriptor, ResourceState, RhiCommand,
//!     TextureId,
//! };
//!
//! let texture = TextureId(7);
//! let draw = RhiCommand::Draw {
//!     vertex_count: 3,
//!     instance_count: 1,
//!     first_vertex: 0,
//!     first_instance: 0,
//! };
//! let write = GraphPass::new(
//!     FramePass::Graphics(
//!         PassDescriptor::new("write").with_color_target(PassColorTarget::Texture(texture)),
//!     ),
//!     vec![draw],
//! );
//! let commands = record(
//!     &[(BarrierResource::Texture(texture), ResourceState::ShaderRead)],
//!     &[write],
//! );
//! assert!(matches!(
//!     &commands[0],
//!     RhiCommand::Barrier {
//!         state: ResourceState::ColorTarget,
//!         ..
//!     }
//! ));
//! ```

use std::collections::HashMap;

use rotex_types::{
    AccessType, BarrierResource, BufferUsageIntent, FramePass, PassColorTarget, ResourceState,
    RhiCommand, TextureId,
};

/// One pass plus the draw or dispatch commands the caller already recorded.
///
/// `texture_access` declares sampled or storage use of a texture. A graphics
/// color target wins over a read of the same texture in that pass.
pub struct GraphPass {
    /// Graphics or compute pass whose targets and buffer intents drive barriers.
    pub pass: FramePass,
    /// Draw or dispatch commands spliced inside the pass markers.
    pub commands: Vec<RhiCommand>,
    /// Sampled or storage uses of textures that are not color targets.
    pub texture_access: Vec<(TextureId, AccessType)>,
}

impl GraphPass {
    /// Builds a pass with no extra texture accesses.
    pub fn new(pass: FramePass, commands: Vec<RhiCommand>) -> Self {
        Self {
            pass,
            commands,
            texture_access: Vec::new(),
        }
    }

    /// Sets textures this pass samples or writes outside the color target.
    pub fn with_texture_access(mut self, texture_access: Vec<(TextureId, AccessType)>) -> Self {
        self.texture_access = texture_access;
        self
    }
}

/// Splices destination barriers and pass markers around the caller's commands.
pub fn record(
    initial: &[(BarrierResource, ResourceState)],
    passes: &[GraphPass],
) -> Vec<RhiCommand> {
    let mut current: HashMap<BarrierResource, ResourceState> = initial.iter().copied().collect();
    let mut commands = Vec::new();
    for pass in passes {
        for (resource, state) in desired_states(pass) {
            match current.get(&resource).copied() {
                Some(existing) if existing == state => {}
                Some(_) => {
                    commands.push(RhiCommand::Barrier { resource, state });
                    current.insert(resource, state);
                }
                None => {
                    current.insert(resource, state);
                }
            }
        }
        match &pass.pass {
            FramePass::Graphics(descriptor) => {
                commands.push(RhiCommand::BeginRenderPass {
                    pass: descriptor.clone(),
                });
                commands.extend(pass.commands.iter().cloned());
                commands.push(RhiCommand::EndRenderPass);
            }
            FramePass::Compute(descriptor) => {
                commands.push(RhiCommand::BeginComputePass {
                    name: descriptor.name.clone(),
                });
                commands.extend(pass.commands.iter().cloned());
                commands.push(RhiCommand::EndComputePass);
            }
        }
    }
    commands
}

fn desired_states(pass: &GraphPass) -> Vec<(BarrierResource, ResourceState)> {
    let mut states = Vec::new();
    match &pass.pass {
        FramePass::Graphics(descriptor) => {
            if let PassColorTarget::Texture(texture) = descriptor.color_target {
                states.push((
                    BarrierResource::Texture(texture),
                    ResourceState::ColorTarget,
                ));
            }
            push_buffer_states(&mut states, &descriptor.buffer_intents);
        }
        FramePass::Compute(descriptor) => {
            push_buffer_states(&mut states, &descriptor.buffer_intents);
        }
    }
    for (texture, access) in &pass.texture_access {
        let resource = BarrierResource::Texture(*texture);
        if states.iter().any(|(existing, _)| *existing == resource) {
            continue;
        }
        if let Some(state) = state_for_access(*access) {
            states.push((resource, state));
        }
    }
    states
}

fn push_buffer_states(
    states: &mut Vec<(BarrierResource, ResourceState)>,
    intents: &[BufferUsageIntent],
) {
    for intent in intents {
        if let Some(state) = state_for_access(intent.access) {
            states.push((BarrierResource::Buffer(intent.buffer), state));
        }
    }
}

fn state_for_access(access: AccessType) -> Option<ResourceState> {
    match access {
        AccessType::None => None,
        AccessType::ComputeRead | AccessType::FragmentRead => Some(ResourceState::ShaderRead),
        AccessType::ComputeWrite | AccessType::ComputeReadWrite => Some(ResourceState::ShaderWrite),
        AccessType::VertexRead | AccessType::VertexShaderRead => Some(ResourceState::VertexBuffer),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rotex_types::{PassDescriptor, TextureId};

    #[test]
    fn texture_write_then_fragment_read_emits_two_barriers() {
        let texture = TextureId(7);
        let draw = RhiCommand::Draw {
            vertex_count: 3,
            instance_count: 1,
            first_vertex: 0,
            first_instance: 0,
        };
        let write = GraphPass::new(
            FramePass::Graphics(
                PassDescriptor::new("write")
                    .with_color_target(PassColorTarget::Texture(texture))
                    .with_clear_color([0.0, 0.0, 0.0, 1.0]),
            ),
            vec![draw.clone()],
        );
        let read = GraphPass::new(FramePass::Graphics(PassDescriptor::new("read")), vec![draw])
            .with_texture_access(vec![(texture, AccessType::FragmentRead)]);

        let commands = record(
            &[(BarrierResource::Texture(texture), ResourceState::ShaderRead)],
            &[write, read],
        );

        assert!(matches!(
            &commands[0],
            RhiCommand::Barrier {
                resource: BarrierResource::Texture(id),
                state: ResourceState::ColorTarget,
            } if *id == texture
        ));
        assert!(matches!(
            &commands[1],
            RhiCommand::BeginRenderPass { pass } if pass.name == "write"
        ));
        assert!(matches!(
            &commands[2],
            RhiCommand::Draw {
                vertex_count: 3,
                ..
            }
        ));
        assert!(matches!(&commands[3], RhiCommand::EndRenderPass));
        assert!(matches!(
            &commands[4],
            RhiCommand::Barrier {
                resource: BarrierResource::Texture(id),
                state: ResourceState::ShaderRead,
            } if *id == texture
        ));
        assert!(matches!(
            &commands[5],
            RhiCommand::BeginRenderPass { pass } if pass.name == "read"
        ));
        assert!(matches!(&commands[6], RhiCommand::Draw { .. }));
        assert!(matches!(&commands[7], RhiCommand::EndRenderPass));
        assert_eq!(commands.len(), 8);
    }
}
