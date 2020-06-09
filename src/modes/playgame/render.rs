use crate::engine::{Context, VertexBuffer};
use super::{Uniforms, Camera, tex3d::Texture3D};

pub struct BlockRenderPass {
    pub vb : VertexBuffer<f32>,
    pub camera : Camera,
    pub uniforms : Uniforms,
    pub shader_id : usize,
    pub uniform_bind_group : wgpu::BindGroup,
    pub render_pipeline : wgpu::RenderPipeline,
    pub uniform_buf : wgpu::Buffer,
    pub material_info : Texture3D,
    mat_info_bind_group : wgpu::BindGroup,
    pub terrain_textures : super::texarray::TextureArray,
    terrain_bind_group : wgpu::BindGroup
}

impl BlockRenderPass {
    pub fn new(context: &mut Context) -> Self {
        let terrain_textures = super::texarray::TextureArray::blank(context).unwrap();

        // Load the 3D texture
        let material_info = Texture3D::blank(
            context,
            None,
            crate::planet::REGION_WIDTH,
            crate::planet::REGION_HEIGHT,
            crate::planet::REGION_DEPTH
        ).unwrap();

        // Initialize the vertex buffer for cube geometry
        let mut vb = VertexBuffer::<f32>::new(&[3, 3, 2]);
        crate::utils::add_cube_geometry(&mut vb.data, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        vb.build(&context.device, wgpu::BufferUsage::VERTEX);

        // Initialize camera and uniforms
        let camera = Camera::new(context.size.width, context.size.height);
        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera, 0);

        // Shader
        let shader_id = context.register_shader(
            "resources/shaders/regionblocks.vert",
            "resources/shaders/regionblocks.frag",
        );

        // WGPU Details
        let uniform_buf = context.device.create_buffer_with_data(
            bytemuck::cast_slice(&[uniforms]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );
        let uniform_bind_group_layout = context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            }],
            label: None,
        });
        let uniform_bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &uniform_buf,
                    range: 0..std::mem::size_of::<Uniforms>() as wgpu::BufferAddress,
                },
            }],
            label: None,
        });

        // Texture buffer descriptions
        let matinfo_bind_group_layout =
            context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    bindings: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: wgpu::TextureViewDimension::D3,
                                component_type: wgpu::TextureComponentType::Uint,
                            },
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Sampler { comparison: false },
                        },
                    ],
                    label: Some("matinfo_bind_group_layout"),
                });

        let mat_info_bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &matinfo_bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &material_info.view,
                        ),
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(
                            &material_info.sampler,
                        ),
                    },
                ],
                label: Some("matinfo_bind_group"),
            }
        );

        // Terrain textures
        let terrain_bind_group_layout =
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

        let terrain_bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &terrain_bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &terrain_textures.view
                        ),
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(
                            &terrain_textures.sampler,
                        ),
                    },
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        let pipeline_layout = context.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&uniform_bind_group_layout, &matinfo_bind_group_layout, &terrain_bind_group_layout],
        });
        let render_pipeline = context.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &vec![wgpu::ColorStateDescriptor {
                format: context.swapchain_format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
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

        // Build the result
        let builder = Self{
            vb,
            camera,
            uniforms,
            shader_id,
            uniform_bind_group,
            render_pipeline,
            uniform_buf,
            material_info,
            mat_info_bind_group,
            terrain_textures,
            terrain_bind_group
        };
        builder
    }

    pub fn render(&mut self, context: &mut Context, depth_id: usize, frame: &wgpu::SwapChainOutput) {
        let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
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
            });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.uniform_bind_group, &[]);
            rpass.set_bind_group(1, &self.mat_info_bind_group, &[]);
            rpass.set_bind_group(2, &self.terrain_bind_group, &[]);
            rpass.set_vertex_buffer(0, &self.vb.buffer.as_ref().unwrap(), 0, 0);
            rpass.draw(0..self.vb.len(), 0..1);
        }
        context.queue.submit(&[encoder.finish()]);
    }
}
