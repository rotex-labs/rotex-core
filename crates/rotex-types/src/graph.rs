use crate::frame::{ComputePassDescriptor, PassDescriptor};
use crate::resource::{AccessType, BufferId};

#[derive(Debug, Clone)]
pub enum RenderCommand {
    TransitionBuffer {
        buffer: BufferId,
        from: AccessType,
        to: AccessType,
    },
    DispatchCompute(ComputePassDescriptor),
    DrawGraphics(PassDescriptor),
}
