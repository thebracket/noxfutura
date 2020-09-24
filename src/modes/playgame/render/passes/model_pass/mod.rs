use crate::{components::*, modes::playgame::GBuffer};
use crate::modes::playgame::{CameraUniform, Models, Palette};
use crate::utils::Frustrum;
use bengine::*;
use legion::*;
use std::collections::HashMap;

pub struct ModelsPass {
    pipeline: gpu::RenderPipeline,
    bind_group: gpu::BindGroup,
    palette_bind_group: gpu::BindGroup,
    models: Models,
    instance_buffer: FloatBuffer<f32>,
    instance_set: HashMap<usize, Vec<(f32, f32, f32)>>,
    instances: Vec<(usize, u32, u32)>,
    pub models_changed: bool,
}

impl ModelsPass {
    pub fn new(palette: &Palette, models: Models, uniforms: &CameraUniform) -> Self {
        let (terrain_vert, terrain_frag) = helpers::shader_from_bytes(
            bengine::gpu::include_spirv!("models.vert.spv"),
            bengine::gpu::include_spirv!("models.frag.spv"),
        );

        let mut instance_buffer = FloatBuffer::new(&[3], 1024, gpu::BufferUsage::VERTEX);
        instance_buffer.attributes[0].shader_location = 3;
        instance_buffer.add3(0.0, 0.0, 0.0);
        instance_buffer.build();

        let dl = RENDER_CONTEXT.read();
        let ctx = dl.as_ref().unwrap();

        let buffer_template = FloatBuffer::<f32>::new(
            &[3, 3, 1],
            1,
            gpu::BufferUsage::VERTEX | gpu::BufferUsage::COPY_DST,
        );

        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&gpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[gpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: gpu::ShaderStage::VERTEX,
                        ty: gpu::BindingType::UniformBuffer {
                            dynamic: false,
                            min_binding_size: gpu::BufferSize::new(64),
                        },
                        count: None,
                    }],
                });

        let bind_group = ctx.device.create_bind_group(&gpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[gpu::BindGroupEntry {
                binding: 0,
                resource: gpu::BindingResource::Buffer(uniforms.uniform_buffer.slice(..)),
            }],
        });
        let palette_bind_group = ctx.device.create_bind_group(&gpu::BindGroupDescriptor {
            label: None,
            layout: &palette.bind_group_layout,
            entries: &[gpu::BindGroupEntry {
                binding: 0,
                resource: gpu::BindingResource::Buffer(palette.palette_buf.slice(..)),
            }],
        });

        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&gpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout, &palette.bind_group_layout],
                push_constant_ranges: &[],
            });
        let pipeline = ctx
            .device
            .create_render_pipeline(&gpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex_stage: gpu::ProgrammableStageDescriptor {
                    module: SHADERS.read().get_module(terrain_vert),
                    entry_point: "main",
                },
                fragment_stage: Some(gpu::ProgrammableStageDescriptor {
                    module: SHADERS.read().get_module(terrain_frag),
                    entry_point: "main",
                }),
                rasterization_state: Some(gpu::RasterizationStateDescriptor {
                    front_face: gpu::FrontFace::Ccw,
                    cull_mode: gpu::CullMode::None,
                    ..Default::default()
                }),
                primitive_topology: gpu::PrimitiveTopology::TriangleList,
                color_states: &[
                    gpu::ColorStateDescriptor {
                        format: ctx.swapchain_format,
                        color_blend: gpu::BlendDescriptor::REPLACE,
                        alpha_blend: gpu::BlendDescriptor::REPLACE,
                        write_mask: gpu::ColorWrite::ALL,
                    },
                    gpu::ColorStateDescriptor {
                        format: gpu::TextureFormat::Rgba32Float,
                        color_blend: gpu::BlendDescriptor::REPLACE,
                        alpha_blend: gpu::BlendDescriptor::REPLACE,
                        write_mask: gpu::ColorWrite::ALL,
                    },
                ],
                depth_stencil_state: Some(gpu::DepthStencilStateDescriptor {
                    format: gpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: gpu::CompareFunction::Less,
                    stencil: gpu::StencilStateDescriptor::default(),
                }),
                vertex_state: gpu::VertexStateDescriptor {
                    index_format: gpu::IndexFormat::Uint16,
                    vertex_buffers: &[
                        buffer_template.descriptor(),
                        instance_buffer.instance_descriptor(),
                    ],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        Self {
            bind_group,
            palette_bind_group,
            pipeline,
            models,
            instance_buffer,
            instance_set: HashMap::new(),
            instances: Vec::new(),
            models_changed: true,
        }
    }

    pub fn render(&mut self, core: &Core, ecs: &mut World, frustrum: &Frustrum, gbuffer: &GBuffer) {
        let dl = RENDER_CONTEXT.read();
        let ctx = dl.as_ref().unwrap();
        let tlock = TEXTURES.read();

        let mut encoder = ctx
            .device
            .create_command_encoder(&gpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&gpu::RenderPassDescriptor {
                color_attachments: &[
                    gpu::RenderPassColorAttachmentDescriptor {
                        attachment: &gbuffer.albedo.view,
                        resolve_target: None,
                        ops: gpu::Operations {
                            load: gpu::LoadOp::Load,
                            store: true,
                        },
                    },
                    gpu::RenderPassColorAttachmentDescriptor {
                        attachment: &gbuffer.normal.view,
                        resolve_target: None,
                        ops: gpu::Operations {
                            load: gpu::LoadOp::Load,
                            store: true,
                        },
                    }
                ],
                depth_stencil_attachment: Some(gpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: tlock.get_view(0),
                    depth_ops: Some(gpu::Operations {
                        load: gpu::LoadOp::Load,
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            if self.models_changed {
                let camera_z = <(&Position, &CameraOptions)>::query()
                    .iter(ecs)
                    .map(|(pos, _)| pos.as_point3())
                    .nth(0)
                    .unwrap()
                    .z;

                self.instance_set.iter_mut().for_each(|(_k, v)| v.clear());
                <(&ObjModel, &Position)>::query()
                    .iter(ecs)
                    .for_each(|(model, pos)| {
                        if let Some(pt) = pos.as_point3_only_tile() {
                            if pt.z <= camera_z
                                && pt.z > camera_z - 50
                                && frustrum.check_sphere(&pos.as_vec3(), 2.0)
                            {
                                if let Some(i) = self.instance_set.get_mut(&model.index) {
                                    i.push((pt.x as f32, pt.z as f32, pt.y as f32));
                                } else {
                                    self.instance_set.insert(
                                        model.index,
                                        vec![(pt.x as f32, pt.z as f32, pt.y as f32)],
                                    );
                                }
                            }
                        }
                    });

                self.instances.clear();
                self.instance_buffer.clear();
                let mut start = 0;
                let mut end = 0;
                for (k, v) in self.instance_set.iter() {
                    if !v.is_empty() {
                        for (x, y, z) in v.iter() {
                            self.instance_buffer.add3(*x, *y, *z);
                            end += 1;
                        }
                        self.instances.push((*k, start as u32, end as u32));
                        start = end;
                    }
                }
                self.instance_buffer.build();
            }

            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_bind_group(1, &self.palette_bind_group, &[]);

            rpass.set_vertex_buffer(0, self.models.vertex_buffer.slice());
            rpass.set_vertex_buffer(1, self.instance_buffer.slice());
            rpass.set_index_buffer(self.models.index_buffer.slice(..));

            for render in self.instances.iter() {
                let range = self.models.model_map[render.0].start as u32
                    ..self.models.model_map[render.0].end as u32;
                rpass.draw_indexed(range, 0, render.1..render.2);
            }
        }
        ctx.queue.submit(Some(encoder.finish()));

        self.models_changed = false;
    }
}
