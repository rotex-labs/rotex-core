//! Pure rewrites of an [`RhiCommand`] list. Backends do not depend on this crate.
//! Call [`rewrite`] before `execute` when a device reports a feature as absent.

use std::collections::HashMap;

use rotex_core::Error;
use rotex_types::{
    BindGroupId, BufferId, CoreCaps, IndexFormat, PassColorTarget, RhiCommand, Support,
};

/// Caller-owned stand-ins. The rewriter never allocates a GPU resource.
#[derive(Default)]
pub struct EmulationResources {
    pub push_constant_buffer: Option<BufferId>,
    pub texture_pass_fallback: Option<PassColorTarget>,
}

/// Lower absent features, then drop repeated fixed-function state.
pub fn rewrite(
    commands: &[RhiCommand],
    caps: &CoreCaps,
    resources: &EmulationResources,
) -> Result<Vec<RhiCommand>, Error> {
    let lowered = lower(commands, caps, resources)?;
    Ok(diff_state(lowered))
}

fn lower(
    commands: &[RhiCommand],
    caps: &CoreCaps,
    resources: &EmulationResources,
) -> Result<Vec<RhiCommand>, Error> {
    let mut output = Vec::with_capacity(commands.len());
    for command in commands {
        match command {
            RhiCommand::PushConstants { offset, data, .. }
                if caps.push_constants == Support::Absent =>
            {
                let Some(buffer) = resources.push_constant_buffer else {
                    return Err(Error::unsupported(
                        "push constant emulation needs a uniform buffer",
                    ));
                };
                output.push(RhiCommand::WriteBuffer {
                    buffer,
                    offset: u64::from(*offset),
                    data: data.clone(),
                });
            }
            RhiCommand::BeginRenderPass { pass }
                if caps.render_to_texture == Support::Absent
                    && matches!(pass.color_target, PassColorTarget::Texture(_)) =>
            {
                let Some(fallback) = resources.texture_pass_fallback else {
                    return Err(Error::unsupported(
                        "texture pass emulation needs a fallback target",
                    ));
                };
                let mut pass = pass.clone();
                pass.color_target = fallback;
                output.push(RhiCommand::BeginRenderPass { pass });
            }
            other => output.push(other.clone()),
        }
    }
    Ok(output)
}

fn diff_state(commands: Vec<RhiCommand>) -> Vec<RhiCommand> {
    let mut viewport = None;
    let mut scissor = None;
    let mut pipeline = None;
    let mut groups: Option<(u32, Vec<BindGroupId>, Vec<u32>)> = None;
    let mut vertex: HashMap<u32, (BufferId, u64)> = HashMap::new();
    let mut index: Option<(BufferId, u64, IndexFormat)> = None;
    let mut output = Vec::with_capacity(commands.len());
    for command in commands {
        let drop_command = match &command {
            RhiCommand::SetViewport {
                x,
                y,
                width,
                height,
                min_depth,
                max_depth,
            } => {
                let next = (*x, *y, *width, *height, *min_depth, *max_depth);
                replace_if_same(&mut viewport, next)
            }
            RhiCommand::SetScissor {
                x,
                y,
                width,
                height,
            } => {
                let next = (*x, *y, *width, *height);
                replace_if_same(&mut scissor, next)
            }
            RhiCommand::SetGraphicsPipeline { pipeline: next } => {
                replace_if_same(&mut pipeline, *next)
            }
            RhiCommand::BindGroups {
                first_slot,
                bind_groups,
                dynamic_offsets,
            } => {
                let next = (*first_slot, bind_groups.clone(), dynamic_offsets.clone());
                replace_if_same(&mut groups, next)
            }
            RhiCommand::BindVertexBuffer {
                buffer,
                binding,
                offset,
            } => match vertex.get(binding) {
                Some(existing) if *existing == (*buffer, *offset) => true,
                _ => {
                    vertex.insert(*binding, (*buffer, *offset));
                    false
                }
            },
            RhiCommand::BindIndexBuffer {
                buffer,
                offset,
                format,
            } => {
                let next = (*buffer, *offset, *format);
                replace_if_same(&mut index, next)
            }
            _ => false,
        };
        if !drop_command {
            output.push(command);
        }
    }
    output
}

fn replace_if_same<T: PartialEq>(slot: &mut Option<T>, next: T) -> bool {
    if slot.as_ref() == Some(&next) {
        true
    } else {
        *slot = Some(next);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rotex_types::{
        BarrierResource, BufferId, GraphicsPipelineId, PassDescriptor, ResourceState,
        ShaderStageFlags, TextureId,
    };

    fn absent() -> CoreCaps {
        CoreCaps {
            push_constants: Support::Absent,
            render_to_texture: Support::Absent,
            ..CoreCaps::default()
        }
    }

    #[test]
    fn push_constants_become_uniform_writes() {
        let buffer = BufferId(4);
        let commands = rewrite(
            &[RhiCommand::PushConstants {
                stages: ShaderStageFlags::VERTEX,
                offset: 16,
                data: vec![1, 2, 3, 4],
            }],
            &absent(),
            &EmulationResources {
                push_constant_buffer: Some(buffer),
                texture_pass_fallback: None,
            },
        )
        .expect("rewrite");
        assert!(matches!(
            &commands[0],
            RhiCommand::WriteBuffer {
                buffer: id,
                offset: 16,
                data,
            } if *id == buffer && data == &vec![1, 2, 3, 4]
        ));
    }

    #[test]
    fn missing_push_constant_buffer_is_unsupported() {
        let error = rewrite(
            &[RhiCommand::PushConstants {
                stages: ShaderStageFlags::VERTEX,
                offset: 0,
                data: vec![0],
            }],
            &absent(),
            &EmulationResources::default(),
        );
        assert!(error.is_err());
    }

    #[test]
    fn repeated_state_is_dropped_and_draws_stay() {
        let pipeline = GraphicsPipelineId(1);
        let commands = rewrite(
            &[
                RhiCommand::SetGraphicsPipeline { pipeline },
                RhiCommand::SetGraphicsPipeline { pipeline },
                RhiCommand::Draw {
                    vertex_count: 3,
                    instance_count: 1,
                    first_vertex: 0,
                    first_instance: 0,
                },
                RhiCommand::Barrier {
                    resource: BarrierResource::Buffer(BufferId(1)),
                    state: ResourceState::VertexBuffer,
                },
                RhiCommand::Draw {
                    vertex_count: 3,
                    instance_count: 1,
                    first_vertex: 0,
                    first_instance: 0,
                },
            ],
            &CoreCaps::default(),
            &EmulationResources::default(),
        )
        .expect("rewrite");
        assert_eq!(commands.len(), 4);
        assert!(matches!(
            commands[0],
            RhiCommand::SetGraphicsPipeline { .. }
        ));
        assert!(matches!(commands[1], RhiCommand::Draw { .. }));
        assert!(matches!(commands[2], RhiCommand::Barrier { .. }));
        assert!(matches!(commands[3], RhiCommand::Draw { .. }));
    }

    #[test]
    fn texture_pass_uses_fallback_target() {
        let texture = TextureId(9);
        let commands = rewrite(
            &[RhiCommand::BeginRenderPass {
                pass: PassDescriptor::new("offscreen")
                    .with_color_target(PassColorTarget::Texture(texture)),
            }],
            &absent(),
            &EmulationResources {
                push_constant_buffer: None,
                texture_pass_fallback: Some(PassColorTarget::Presentation),
            },
        )
        .expect("rewrite");
        assert!(matches!(
            &commands[0],
            RhiCommand::BeginRenderPass { pass }
                if matches!(pass.color_target, PassColorTarget::Presentation)
        ));
    }

    #[test]
    fn texture_pass_without_fallback_is_unsupported() {
        let error = rewrite(
            &[RhiCommand::BeginRenderPass {
                pass: PassDescriptor::new("offscreen")
                    .with_color_target(PassColorTarget::Texture(TextureId(1))),
            }],
            &absent(),
            &EmulationResources::default(),
        );
        assert!(error.is_err());
    }
}
