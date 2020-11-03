use bengine::*;
use legion::*;

use crate::modes::playgame::GBuffer;
mod light_uniforms;
use light_uniforms::LightUniformManager;
mod terrain_lights;
pub use terrain_lights::TerrainLights;

pub struct LightingPass {
    vb: FloatBuffer<f32>,
    pipeline: gpu::RenderPipeline,
    bind_group_layout: gpu::BindGroupLayout,
    bind_group: gpu::BindGroup,
    light_uniforms: LightUniformManager,
    uniform_bind_group: gpu::BindGroup,
    terrain_lights: TerrainLights,
    pub lighting_changed: bool,
}

impl LightingPass {
    pub fn new(gbuffer: &GBuffer) -> Self {
        // Shader
        let (light_vert, light_frag) = helpers::shader_from_bytes(
            bengine::gpu::include_spirv!("lighting.vert.spv"),
            bengine::gpu::include_spirv!("lighting.frag.spv"),
        );

        // Lighting buffer setup
        let terrain_lights = TerrainLights::new();

        // Uniform setup
        let light_uniforms = LightUniformManager::new();

        // Simple quad VB for output
        let mut vb = FloatBuffer::<f32>::new(&[2, 2], 24, gpu::BufferUsage::VERTEX);
        vb.add_slice(&[
            -1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
        ]);
        vb.build();

        // Pipeline setup
        let dl = RENDER_CONTEXT.read();
        let ctx = dl.as_ref().unwrap();

        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&gpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        gpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: gpu::TextureViewDimension::D2,
                                component_type: gpu::TextureComponentType::Uint,
                            },
                            count: None,
                        },
                        gpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::Sampler { comparison: true },
                            count: None,
                        },
                        gpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: gpu::TextureViewDimension::D2,
                                component_type: gpu::TextureComponentType::Uint,
                            },
                            count: None,
                        },
                        gpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::Sampler { comparison: true },
                            count: None,
                        },
                        gpu::BindGroupLayoutEntry {
                            binding: 4,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: gpu::TextureViewDimension::D2,
                                component_type: gpu::TextureComponentType::Uint,
                            },
                            count: None,
                        },
                        gpu::BindGroupLayoutEntry {
                            binding: 5,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::Sampler { comparison: true },
                            count: None,
                        },
                    ],
                });

        let bind_group = ctx.device.create_bind_group(&gpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                gpu::BindGroupEntry {
                    binding: 0,
                    resource: gpu::BindingResource::TextureView(&gbuffer.albedo.view),
                },
                gpu::BindGroupEntry {
                    binding: 1,
                    resource: gpu::BindingResource::Sampler(&gbuffer.albedo.sampler),
                },
                gpu::BindGroupEntry {
                    binding: 2,
                    resource: gpu::BindingResource::TextureView(&gbuffer.normal.view),
                },
                gpu::BindGroupEntry {
                    binding: 3,
                    resource: gpu::BindingResource::Sampler(&gbuffer.normal.sampler),
                },
                gpu::BindGroupEntry {
                    binding: 4,
                    resource: gpu::BindingResource::TextureView(&gbuffer.coords.view),
                },
                gpu::BindGroupEntry {
                    binding: 5,
                    resource: gpu::BindingResource::Sampler(&gbuffer.coords.sampler),
                },
            ],
        });

        let uniform_bind_group_layout =
            ctx.device
                .create_bind_group_layout(&gpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[gpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: gpu::ShaderStage::VERTEX | gpu::ShaderStage::FRAGMENT,
                        ty: gpu::BindingType::UniformBuffer {
                            dynamic: false,
                            min_binding_size: gpu::BufferSize::new(64),
                        },
                        count: None,
                    }],
                });

        let uniform_bind_group = ctx.device.create_bind_group(&gpu::BindGroupDescriptor {
            label: None,
            layout: &uniform_bind_group_layout,
            entries: &[gpu::BindGroupEntry {
                binding: 0,
                resource: gpu::BindingResource::Buffer(light_uniforms.uniform_buffer.slice(..)),
            }],
        });

        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&gpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &bind_group_layout,
                    &uniform_bind_group_layout,
                    &terrain_lights.bind_group_layout,
                    &gbuffer.bind_group_layout,
                ],
                push_constant_ranges: &[],
            });
        let pipeline = ctx
            .device
            .create_render_pipeline(&gpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex_stage: gpu::ProgrammableStageDescriptor {
                    module: SHADERS.read().get_module(light_vert),
                    entry_point: "main",
                },
                fragment_stage: Some(gpu::ProgrammableStageDescriptor {
                    module: SHADERS.read().get_module(light_frag),
                    entry_point: "main",
                }),
                rasterization_state: Some(gpu::RasterizationStateDescriptor {
                    front_face: gpu::FrontFace::Ccw,
                    cull_mode: gpu::CullMode::None,
                    ..Default::default()
                }),
                primitive_topology: gpu::PrimitiveTopology::TriangleList,
                color_states: &[gpu::ColorStateDescriptor {
                    format: ctx.swapchain_format,
                    color_blend: gpu::BlendDescriptor::REPLACE,
                    alpha_blend: gpu::BlendDescriptor::REPLACE,
                    write_mask: gpu::ColorWrite::ALL,
                }],
                depth_stencil_state: None,
                vertex_state: gpu::VertexStateDescriptor {
                    index_format: gpu::IndexFormat::Uint16,
                    vertex_buffers: &[vb.descriptor()],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        Self {
            vb,
            bind_group_layout,
            bind_group,
            pipeline,
            light_uniforms,
            uniform_bind_group,
            terrain_lights,
            lighting_changed: true,
        }
    }

    pub fn on_resize(&mut self, gbuffer: &GBuffer) {
        let dl = RENDER_CONTEXT.read();
        let ctx = dl.as_ref().unwrap();
        let bind_group = ctx.device.create_bind_group(&gpu::BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_layout,
            entries: &[
                gpu::BindGroupEntry {
                    binding: 0,
                    resource: gpu::BindingResource::TextureView(&gbuffer.albedo.view),
                },
                gpu::BindGroupEntry {
                    binding: 1,
                    resource: gpu::BindingResource::Sampler(&gbuffer.albedo.sampler),
                },
                gpu::BindGroupEntry {
                    binding: 2,
                    resource: gpu::BindingResource::TextureView(&gbuffer.normal.view),
                },
                gpu::BindGroupEntry {
                    binding: 3,
                    resource: gpu::BindingResource::Sampler(&gbuffer.normal.sampler),
                },
                gpu::BindGroupEntry {
                    binding: 4,
                    resource: gpu::BindingResource::TextureView(&gbuffer.coords.view),
                },
                gpu::BindGroupEntry {
                    binding: 5,
                    resource: gpu::BindingResource::Sampler(&gbuffer.coords.sampler),
                },
            ],
        });
        self.bind_group = bind_group;
    }

    pub fn render(&mut self, core: &Core, ecs: &mut World, gbuffer: &GBuffer) {
        let mouse_pos = core.imgui.io().mouse_pos;
        if self.lighting_changed {
            self.light_uniforms
                .uniforms
                .update(ecs, &mut self.terrain_lights.flags, &mouse_pos);
            self.light_uniforms.send_buffer_to_gpu();
            self.terrain_lights.update_buffer();
            self.lighting_changed = false;
        } else {
            self.light_uniforms.uniforms.update_partial(ecs, &mouse_pos);
            self.light_uniforms.send_buffer_to_gpu();
            self.lighting_changed = false;
        }

        let dl = RENDER_CONTEXT.read();
        let ctx = dl.as_ref().unwrap();

        let mut encoder = ctx
            .device
            .create_command_encoder(&gpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&gpu::RenderPassDescriptor {
                color_attachments: &[gpu::RenderPassColorAttachmentDescriptor {
                    attachment: &core.frame.output.view,
                    resolve_target: None,
                    ops: gpu::Operations {
                        load: gpu::LoadOp::Clear(gpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_bind_group(1, &self.uniform_bind_group, &[]);
            rpass.set_bind_group(2, &self.terrain_lights.bind_group, &[]);
            rpass.set_bind_group(3, &gbuffer.bind_group, &[]);

            rpass.set_vertex_buffer(0, self.vb.slice());
            rpass.draw(0..self.vb.len(), 0..1);
        }
        ctx.queue.submit(Some(encoder.finish()));
    }
}
