mod cubes;
use crate::modes::playgame::{CameraUniform, DesignMode, RunState};
use bengine::*;
use cubes::add_cube_geometry;
use legion::*;
use nox_components::*;
use nox_spatial::*;

pub struct CursorPass {
    pub render_pipeline: gpu::RenderPipeline,
    pub vb: FloatBuffer<f32>,
    pub texture_id: usize,
    texture_bind_group: gpu::BindGroup,
}

impl CursorPass {
    pub fn new(uniforms: &CameraUniform) -> Self {
        // Vertex Buffer
        let mut vb = FloatBuffer::<f32>::new(&[3, 3], 100, gpu::BufferUsage::VERTEX);
        vb.add3(1.0, 1.0, 1.0);
        vb.add(1.0);
        vb.build();

        // Shader
        let (cursor_vert, cursor_frag) = helpers::shader_from_bytes(
            bengine::gpu::include_spirv!("cursors.vert.spv"),
            bengine::gpu::include_spirv!("cursors.frag.spv"),
        );

        // Internal pipeline

        let texture_id = TEXTURES.write().load_texture_from_bytes(
            include_bytes!("../../../../../../resources/cursors/chop_cursor.png"),
            "Choppa",
        );

        let mut ctx = RENDER_CONTEXT.write();
        let context = ctx.as_mut().unwrap();

        let texture_bind_group_layout =
            context
                .device
                .create_bind_group_layout(&gpu::BindGroupLayoutDescriptor {
                    entries: &[
                        gpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: gpu::ShaderStage::VERTEX,
                            ty: gpu::BindingType::UniformBuffer {
                                dynamic: false,
                                min_binding_size: gpu::BufferSize::new(64),
                            },
                            count: None,
                        },
                        gpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: gpu::TextureViewDimension::D2,
                                component_type: gpu::TextureComponentType::Uint,
                            },
                            count: None,
                        },
                        gpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::Sampler { comparison: false },
                            count: None,
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                });

        let texture_bind_group = context.device.create_bind_group(&gpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                gpu::BindGroupEntry {
                    binding: 0,
                    resource: gpu::BindingResource::Buffer(uniforms.uniform_buffer.slice(..)),
                },
                gpu::BindGroupEntry {
                    binding: 1,
                    resource: gpu::BindingResource::TextureView(
                        TEXTURES.read().get_view(texture_id),
                    ),
                },
                gpu::BindGroupEntry {
                    binding: 2,
                    resource: gpu::BindingResource::Sampler(
                        TEXTURES.read().get_sampler(texture_id),
                    ),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let pipeline_layout =
            context
                .device
                .create_pipeline_layout(&gpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&texture_bind_group_layout],
                    label: None,
                    push_constant_ranges: &[],
                });
        let render_pipeline =
            context
                .device
                .create_render_pipeline(&gpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&pipeline_layout),
                    vertex_stage: gpu::ProgrammableStageDescriptor {
                        module: SHADERS.read().get_module(cursor_vert),
                        entry_point: "main",
                    },
                    fragment_stage: Some(gpu::ProgrammableStageDescriptor {
                        module: SHADERS.read().get_module(cursor_frag),
                        entry_point: "main",
                    }),
                    rasterization_state: Some(gpu::RasterizationStateDescriptor {
                        front_face: gpu::FrontFace::Ccw,
                        cull_mode: gpu::CullMode::Back,
                        depth_bias: 0,
                        depth_bias_slope_scale: 0.0,
                        depth_bias_clamp: 0.0,
                        clamp_depth: false,
                    }),
                    primitive_topology: gpu::PrimitiveTopology::TriangleList,
                    color_states: &[gpu::ColorStateDescriptor {
                        format: context.swapchain_format,
                        color_blend: gpu::BlendDescriptor {
                            src_factor: gpu::BlendFactor::SrcAlpha,
                            dst_factor: gpu::BlendFactor::OneMinusSrcAlpha,
                            operation: gpu::BlendOperation::Add,
                        },
                        alpha_blend: gpu::BlendDescriptor {
                            src_factor: gpu::BlendFactor::One,
                            dst_factor: gpu::BlendFactor::One,
                            operation: gpu::BlendOperation::Add,
                        },
                        write_mask: gpu::ColorWrite::ALL,
                    }],
                    depth_stencil_state: Some(gpu::DepthStencilStateDescriptor {
                        format: gpu::TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: gpu::CompareFunction::Less,
                        stencil: gpu::StencilStateDescriptor::default(),
                    }),
                    vertex_state: gpu::VertexStateDescriptor {
                        index_format: gpu::IndexFormat::Uint16,
                        vertex_buffers: &[vb.descriptor()],
                    },
                    sample_count: 1,
                    sample_mask: !0,
                    alpha_to_coverage_enabled: false,
                });

        Self {
            render_pipeline,
            vb,
            texture_id,
            texture_bind_group,
        }
    }

    fn lumberjack(&mut self, ecs: &World) {
        self.vb.clear();

        let rlock = crate::modes::playgame::systems::REGION.read();

        <(&Position, &IdentityTag)>::query()
            .filter(component::<Tree>())
            .iter(ecs)
            .filter(|(_, id)| rlock.jobs_board.get_trees().contains(&id.0))
            .for_each(|(pos, _)| {
                let pt = pos.as_point3();
                add_cube_geometry(
                    &mut self.vb.data,
                    pt.x as f32,
                    pt.y as f32,
                    pt.z as f32,
                    1.0,
                    1.0,
                    2.0,
                    1.0,
                );
            });
        if self.vb.len() == 0 {
            return;
        }
        self.vb.update_buffer();
    }

    fn mining(&mut self) {
        self.vb.clear();

        let rlock = crate::modes::playgame::systems::REGION.read();
        rlock
            .jobs_board
            .mining_designations
            .iter()
            .for_each(|(idx, _t)| {
                let (x, y, z) = idxmap(*idx);
                add_cube_geometry(
                    &mut self.vb.data,
                    x as f32,
                    y as f32,
                    z as f32,
                    1.0,
                    1.0,
                    1.0,
                    1.0,
                );
            });

        if self.vb.len() == 0 {
            return;
        }
        self.vb.update_buffer();
    }

    pub fn render(&mut self, core: &Core, ecs: &World, run_state: &RunState) {
        self.vb.clear();
        if let RunState::Design { mode } = run_state {
            match mode {
                DesignMode::Lumberjack => self.lumberjack(ecs),
                DesignMode::Mining { .. } => self.mining(),
                _ => {}
            }
        }

        if self.vb.len() == 0 {
            return;
        }

        let tlock = TEXTURES.read();
        let mut ctx_lock = RENDER_CONTEXT.write();
        let context = ctx_lock.as_mut().unwrap();
        let mut encoder = context
            .device
            .create_command_encoder(&gpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&gpu::RenderPassDescriptor {
                color_attachments: &[gpu::RenderPassColorAttachmentDescriptor {
                    attachment: &core.frame.output.view,
                    resolve_target: None,
                    ops: gpu::Operations {
                        load: gpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(gpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: tlock.get_view(0),
                    depth_ops: Some(gpu::Operations {
                        load: gpu::LoadOp::Load,
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.texture_bind_group, &[]);

            if self.vb.len() > 0 {
                rpass.set_vertex_buffer(0, self.vb.buffer.as_ref().unwrap().slice(..));
                rpass.draw(0..self.vb.len(), 0..1);
            }
        }
        context.queue.submit(Some(encoder.finish()));
    }
}
