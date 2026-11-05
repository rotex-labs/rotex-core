use rotex_core::{
    ColorAttachmentLoad, PassColorTarget, PassDescriptor, ResourceBatchCreate,
    ResourceCreateDescriptor, RhiCommand,
};

fn main() {
    let pass = PassDescriptor::new("smoke")
        .with_clear_color([0.1, 0.2, 0.3, 1.0])
        .with_color_load(ColorAttachmentLoad::Clear);
    assert_eq!(pass.name, "smoke");
    assert_eq!(pass.color_target, PassColorTarget::Presentation);

    let batch = ResourceBatchCreate::new(vec![]);
    assert!(batch.resources.is_empty());

    let _descriptor: ResourceCreateDescriptor;
    let commands = [
        RhiCommand::BeginFrame { frame_index: 0 },
        RhiCommand::AcquirePresentationTarget,
        RhiCommand::BeginRenderPass { pass: pass.clone() },
        RhiCommand::EndRenderPass,
    ];
    assert_eq!(commands.len(), 4);
}
