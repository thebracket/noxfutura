use crate::modes::playgame::{CameraUniform, GBuffer, Palette};
use super::super::VoxBuffer;
use bengine::*;
use crate::utils::Frustrum;
use legion::*;
use crate::components::*;

pub struct VoxPass {
    pub vox_models: VoxBuffer,
    pub instance_buffer: FloatBuffer<f32>,
    pipeline: gpu::RenderPipeline,
    bind_group: gpu::BindGroup,
    palette_bind_group: gpu::BindGroup,
    pub models_changed: bool,
    vox_instances: Vec<(u32, u32, u32)>
}

impl VoxPass {
    pub fn new(uniforms: &CameraUniform, palette: &Palette) -> Self {
        // Shader
        let (vox_vert, vox_frag) = helpers::shader_from_bytes(
            bengine::gpu::include_spirv!("vox.vert.spv"),
            bengine::gpu::include_spirv!("vox.frag.spv"),
        );

        // Build the base voxel buffer
        let mut vox_models = VoxBuffer::new();
        vox_models.load(palette);

        // Instance buffer
        let mut instance_buffer = FloatBuffer::<f32>::new(&[3, 1, 1, 1], 100, gpu::BufferUsage::VERTEX);
        instance_buffer.attributes[0].shader_location = 3;
        instance_buffer.attributes[1].shader_location = 4;
        instance_buffer.attributes[2].shader_location = 5;
        instance_buffer.attributes[3].shader_location = 6;
        instance_buffer.add3(128., 256., 128.);
        instance_buffer.add(0.0);
        instance_buffer.add(0.0);
        instance_buffer.add(0.0);
        instance_buffer.build();

        // Pipeline setup
        let dl = RENDER_CONTEXT.read();
        let ctx = dl.as_ref().unwrap();

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
            }
        );

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
            }
        );

        let pipeline = ctx
            .device
            .create_render_pipeline(&gpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex_stage: gpu::ProgrammableStageDescriptor {
                    module: SHADERS.read().get_module(vox_vert),
                    entry_point: "main",
                },
                fragment_stage: Some(gpu::ProgrammableStageDescriptor {
                    module: SHADERS.read().get_module(vox_frag),
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
                        format: ctx.swapchain_format,
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
                        vox_models.vertices.descriptor(),
                        instance_buffer.instance_descriptor(),
                    ],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            }
        );

        Self{
            vox_models,
            instance_buffer,
            pipeline,
            palette_bind_group,
            bind_group,
            models_changed: true,
            vox_instances: Vec::new()
        }
    }

    pub fn render(&mut self, core: &Core, ecs: &mut World, frustrum: &Frustrum, palette: &Palette, gbuffer: &GBuffer) {
        if self.models_changed {
            let camera_z = <(&Position, &CameraOptions)>::query()
                .iter(ecs)
                .map(|(pos, _)| pos.as_point3())
                .nth(0)
                .unwrap()
                .z as usize;

            super::super::voxels::build_vox_instances2(
                ecs,
                camera_z,
                &self.vox_models,
                &mut self.instance_buffer,
                &mut self.vox_instances,
                frustrum,
                &(0, 0, 0),
                &None,
                palette
            );

            self.models_changed = false;
        }

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

            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_bind_group(1, &self.palette_bind_group, &[]);

            rpass.set_vertex_buffer(0, self.vox_models.vertices.slice());
            rpass.set_vertex_buffer(1, self.instance_buffer.slice());

            if !self.vox_instances.is_empty() {
                let mut count = 0;
                for i in self.vox_instances.iter() {
                    rpass.draw(i.0..i.1, count as u32..count as u32 + i.2 as u32);
                    count += i.2;
                }
            }
        }
        ctx.queue.submit(Some(encoder.finish()));
    }
}