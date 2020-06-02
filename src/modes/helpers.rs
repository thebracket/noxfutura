use crate::engine::{renderpass, Context};

pub fn render_menu_background(
    context: &mut Context,
    frame: &wgpu::SwapChainOutput,
    resources: &super::SharedResources,
) {
    let mut encoder = renderpass::get_encoder(&context);
    {
        let mut rpass = renderpass::get_render_pass(&mut encoder, &frame, wgpu::LoadOp::Clear);
        rpass.set_pipeline(resources.quad_pipeline.as_ref().unwrap());
        rpass.set_bind_group(0, resources.quad_bind_group.as_ref().unwrap(), &[]);
        rpass.set_vertex_buffer(0, &resources.quad_vb.buffer.as_ref().unwrap(), 0, 0);
        rpass.draw(0..resources.quad_vb.len(), 0..1);
    }

    context.queue.submit(&[encoder.finish()]);
}
