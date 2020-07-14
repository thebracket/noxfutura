mod cubes;
use super::super::RunState;
use crate::engine::{VertexBuffer, DEVICE_CONTEXT};
use cubes::add_cube_geometry;
use legion::prelude::*;
use nox_components::*;

pub struct CursorPass {
    pub shader_id: usize,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vb: VertexBuffer<f32>,
    pub texture_id: usize,
    texture_bind_group: wgpu::BindGroup,
}

impl CursorPass {
    pub fn new(uniform_bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        // Vertex Buffer
        let mut vb = VertexBuffer::<f32>::new(&[3, 3]);
        vb.add3(1.0, 1.0, 1.0);
        vb.add(1.0);
        vb.build(wgpu::BufferUsage::VERTEX);

        // Shader
        let shader_id = crate::engine::register_shader(
            "resources/shaders/cursors.vert",
            "resources/shaders/cursors.frag",
        );

        // Internal pipeline
        let mut ctx = DEVICE_CONTEXT.write();
        let context = ctx.as_mut().unwrap();

        let texture_id = context.register_texture(
            include_bytes!("../../../../../resources/cursors/chop_cursor.png"),
            "Choppa",
        );

        let texture_bind_group_layout =
            context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    bindings: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: wgpu::TextureViewDimension::D2,
                                component_type: wgpu::TextureComponentType::Uint,
                            },
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Sampler { comparison: false },
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                });

        let texture_bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &context.textures[texture_id].view,
                        ),
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(
                            &context.textures[texture_id].sampler,
                        ),
                    },
                ],
                label: Some("diffuse_bind_group"),
            });

        let pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
                });
        let render_pipeline =
            context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    layout: &pipeline_layout,
                    vertex_stage: wgpu::ProgrammableStageDescriptor {
                        module: &context.shaders[shader_id].vs_module,
                        entry_point: "main",
                    },
                    fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                        module: &context.shaders[shader_id].fs_module,
                        entry_point: "main",
                    }),
                    rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: wgpu::CullMode::Back,
                        depth_bias: 0,
                        depth_bias_slope_scale: 0.0,
                        depth_bias_clamp: 0.0,
                    }),
                    primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                    color_states: &[wgpu::ColorStateDescriptor {
                        format: context.swapchain_format,
                        color_blend: wgpu::BlendDescriptor {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha_blend: wgpu::BlendDescriptor {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                    depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                        format: crate::engine::texture::Texture::DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
                        stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
                        stencil_read_mask: 0,
                        stencil_write_mask: 0,
                    }),
                    vertex_state: wgpu::VertexStateDescriptor {
                        index_format: wgpu::IndexFormat::Uint16,
                        vertex_buffers: &[vb.descriptor()],
                    },
                    sample_count: 1,
                    sample_mask: !0,
                    alpha_to_coverage_enabled: false,
                });

        Self {
            shader_id,
            render_pipeline,
            vb,
            texture_id,
            texture_bind_group,
        }
    }

    pub fn render(
        &mut self,
        depth_id: usize,
        frame: &wgpu::SwapChainOutput,
        uniform_bg: &wgpu::BindGroup,
        run_state: &RunState,
        ecs: &World,
    ) {
        if let RunState::Design { .. } = run_state {
            self.vb.clear();
            let rlock = crate::systems::REGION.read();
            <(Read<Position>, Read<Identity>)>::query()
                .filter(tag::<Tree>())
                .iter(ecs)
                .filter(|(_, id)| rlock.jobs_board.get_trees().contains(&id.id))
                .for_each(|(pos, _)| {
                    let pt = pos.as_point3();
                    add_cube_geometry(
                        &mut self.vb.data,
                        pt.x as f32,
                        pt.y as f32,
                        pt.z as f32,
                        3.0,
                        3.0,
                        3.0,
                        1.0,
                    );
                });
            std::mem::drop(rlock);
            if self.vb.len() == 0 {
                return;
            }
            self.vb.update_buffer();

            let mut ctx_lock = DEVICE_CONTEXT.write();
            let context = ctx_lock.as_mut().unwrap();
            let mut encoder = context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Load,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::BLUE,
                    }],
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachmentDescriptor {
                            attachment: &context.textures[depth_id].view,
                            depth_load_op: wgpu::LoadOp::Clear,
                            depth_store_op: wgpu::StoreOp::Store,
                            clear_depth: 1.0,
                            stencil_load_op: wgpu::LoadOp::Clear,
                            stencil_store_op: wgpu::StoreOp::Store,
                            clear_stencil: 0,
                        },
                    ),
                });

                rpass.set_pipeline(&self.render_pipeline);
                rpass.set_bind_group(0, &uniform_bg, &[]);
                rpass.set_bind_group(1, &self.texture_bind_group, &[]);

                if self.vb.len() > 0 {
                    rpass.set_vertex_buffer(0, &self.vb.buffer.as_ref().unwrap(), 0, 0);
                    rpass.draw(0..self.vb.len(), 0..1);
                }
            }
            context.queue.submit(&[encoder.finish()]);
        }
    }
}
