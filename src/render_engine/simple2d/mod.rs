use super::{ASSETS, OUTPUT_FORMAT, RENDER_CONTEXT};
use wgpu::RenderPipeline;

pub fn pipeline2d() -> RenderPipeline {
    let mut gpu_lock = RENDER_CONTEXT.write();
    let gpu = gpu_lock.as_mut().unwrap();

    let assets = ASSETS.read();
    let shader_handle = assets.shader_handle("simple2d");
    let buffer_handle = assets.buffer_handle("background_quad");
    let tex_handle = assets.texture_handle("background_logo");

    let render_pipeline_layout =
        gpu.device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&assets.textures[tex_handle].bind_group_layout],
                push_constant_ranges: &[],
            });

    let pipeline = gpu
        .device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &assets.shaders[shader_handle].vs_module,
                entry_point: "main",
                buffers: &[assets.vertex_buffers[buffer_handle].descriptor()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &assets.shaders[shader_handle].fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: OUTPUT_FORMAT,
                    write_mask: wgpu::ColorWrite::ALL,
                    blend: None,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

    pipeline
}

pub fn render_nf_background(
    pipeline: &Option<RenderPipeline>,
    swap_chain_texture: &wgpu::SwapChainTexture,
) {
    let assets = ASSETS.read();
    let buffer_handle = assets.buffer_handle("background_quad");
    let tex_handle = assets.texture_handle("background_logo");

    let mut gpu_lock = RENDER_CONTEXT.write();
    let gpu = gpu_lock.as_mut().unwrap();

    let mut encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

    {
        let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("LoaderRenderPass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &swap_chain_texture.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        rp.set_pipeline(pipeline.as_ref().unwrap());
        rp.set_vertex_buffer(0, assets.vertex_buffers[buffer_handle].buffer.slice(..));
        rp.set_bind_group(0, &assets.textures[tex_handle].bind_group, &[]);
        rp.draw(0..6, 0..1);
    }
    gpu.queue.submit(Some(encoder.finish()));
}

pub fn render_nebula_background(
    pipeline: &Option<RenderPipeline>,
    swap_chain_texture: &wgpu::SwapChainTexture,
) {
    let assets = ASSETS.read();
    let buffer_handle = assets.buffer_handle("background_quad");
    let tex_handle = assets.texture_handle("nebula");

    let mut gpu_lock = RENDER_CONTEXT.write();
    let gpu = gpu_lock.as_mut().unwrap();

    let mut encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

    {
        let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("LoaderRenderPass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &swap_chain_texture.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        rp.set_pipeline(pipeline.as_ref().unwrap());
        rp.set_vertex_buffer(0, assets.vertex_buffers[buffer_handle].buffer.slice(..));
        rp.set_bind_group(0, &assets.textures[tex_handle].bind_group, &[]);
        rp.draw(0..6, 0..1);
    }
    gpu.queue.submit(Some(encoder.finish()));
}