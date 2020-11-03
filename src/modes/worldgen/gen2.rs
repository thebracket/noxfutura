use super::worldgen_uniforms::*;
use crate::{GameMode, NoxMode, SharedResources};
use bengine::gui::*;
use bengine::*;
use nox_planet::WORLDGEN_RENDER;

pub struct WorldGen2 {
    pipeline: gpu::RenderPipeline,
    bind_group: gpu::BindGroup,
    uniforms: Uniforms,
    camera: Camera,
}

impl WorldGen2 {
    pub fn new() -> Self {
        let (planet_shader_vert, planet_shader_frag) = helpers::shader_from_bytes(
            bengine::gpu::include_spirv!("../../../resources/shaders/planetgen.vert.spv"),
            bengine::gpu::include_spirv!("../../../resources/shaders/planetgen.frag.spv"),
        );

        let mut renderlock = WORLDGEN_RENDER.lock();
        renderlock.vertex_buffer.build();

        let dl = RENDER_CONTEXT.read();
        let ctx = dl.as_ref().unwrap();
        let size = ctx.size;
        let mut camera = Camera::new(size.width, size.height);

        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&mut camera);

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
        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&gpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        let pipeline = ctx
            .device
            .create_render_pipeline(&gpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex_stage: gpu::ProgrammableStageDescriptor {
                    module: SHADERS.read().get_module(planet_shader_vert),
                    entry_point: "main",
                },
                fragment_stage: Some(gpu::ProgrammableStageDescriptor {
                    module: SHADERS.read().get_module(planet_shader_frag),
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
                depth_stencil_state: Some(gpu::DepthStencilStateDescriptor {
                    format: gpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: gpu::CompareFunction::Less,
                    stencil: gpu::StencilStateDescriptor::default(),
                }),
                vertex_state: gpu::VertexStateDescriptor {
                    index_format: gpu::IndexFormat::Uint16,
                    vertex_buffers: &[renderlock.vertex_buffer.descriptor()],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        Self {
            camera,
            uniforms,
            bind_group,
            pipeline,
        }
    }
}

impl NoxMode for WorldGen2 {
    fn tick(&mut self, core: &mut Core, shared: &SharedResources) -> GameMode {
        let mut result = GameMode::WorldGen2;
        shared.quad_render.render(shared.background_image, core);

        gui::Window::new(im_str!("Status"))
            .position([10.0, 10.0], Condition::Always)
            .always_auto_resize(true)
            .collapsible(false)
            .build(core.imgui, || {
                core.imgui
                    .text(ImString::new(nox_planet::get_worldgen_status()));
            });

        let mut renderlock = WORLDGEN_RENDER.lock();
        if renderlock.needs_update {
            renderlock.vertex_buffer.update_buffer();
            renderlock.needs_update = false;
        }

        if renderlock.vertex_buffer.len() > 0 {
            let tlock = TEXTURES.read();
            self.uniforms.update_view_proj(&self.camera);

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
                            load: gpu::LoadOp::Load,
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: Some(
                        gpu::RenderPassDepthStencilAttachmentDescriptor {
                            attachment: tlock.get_view(0),
                            depth_ops: Some(gpu::Operations {
                                load: gpu::LoadOp::Clear(1.0),
                                store: true,
                            }),
                            stencil_ops: None,
                        },
                    ),
                });
                rpass.set_pipeline(&self.pipeline);
                rpass.set_bind_group(0, &self.bind_group, &[]);
                rpass.set_vertex_buffer(0, renderlock.vertex_buffer.slice());
                rpass.draw(0..renderlock.vertex_buffer.len(), 0..1);
            }

            ctx.queue.submit(Some(encoder.finish()));
        }

        if nox_planet::is_worldgen_done() {
            result = GameMode::MainMenu;
        }

        result
    }
}
