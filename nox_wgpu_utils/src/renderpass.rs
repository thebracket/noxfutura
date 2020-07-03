pub fn get_frame(swap_chain: &mut wgpu::SwapChain) -> wgpu::SwapChainOutput {
    swap_chain
        .get_next_texture()
        .expect("Timeout when acquiring next swap chain texture")
}

pub fn get_encoder(context: &super::Context) -> wgpu::CommandEncoder {
    context
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None })
}

pub fn get_render_pass_with_depth<'a>(
    context: &'a super::Context,
    encoder: &'a mut wgpu::CommandEncoder,
    frame: &'a wgpu::SwapChainOutput,
    depth_id: usize,
    load_op: wgpu::LoadOp,
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &frame.view,
            resolve_target: None,
            load_op: load_op,
            store_op: wgpu::StoreOp::Store,
            clear_color: wgpu::Color::BLACK,
        }],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
            attachment: &context.textures[depth_id].view,
            depth_load_op: wgpu::LoadOp::Clear,
            depth_store_op: wgpu::StoreOp::Store,
            clear_depth: 1.0,
            stencil_load_op: wgpu::LoadOp::Clear,
            stencil_store_op: wgpu::StoreOp::Store,
            clear_stencil: 0,
        }),
    })
}

pub fn get_render_pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    frame: &'a wgpu::SwapChainOutput,
    load_op: wgpu::LoadOp,
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &frame.view,
            resolve_target: None,
            load_op: load_op,
            store_op: wgpu::StoreOp::Store,
            clear_color: wgpu::Color::BLACK,
        }],
        depth_stencil_attachment: None,
    })
}
