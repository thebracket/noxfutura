use crate::engine::DEVICE_CONTEXT;
use crate::engine::VertexBuffer;
use ultraviolet::Mat4;
use ultraviolet::Vec3;
use crate::engine::uniforms::UniformBlock;

pub struct SunDepthVoxPass {
    pub shader_id: usize,
    pub render_pipeline: wgpu::RenderPipeline,

    pub camera : SunCamera,
    pub uniforms: SunUniforms,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_buf: wgpu::Buffer,
}

impl SunDepthVoxPass {
    pub fn new(vb: &VertexBuffer<f32>, instance_buffer: &VertexBuffer<f32>) -> Self {
        let size = crate::engine::get_window_size();
        let camera = SunCamera::new(size.width, size.height);
        let mut uniforms = SunUniforms::new();
        uniforms.update_view_proj(&camera);

        // Shader
        let shader_id = crate::engine::register_shader(
            "resources/shaders/sun_vox_depth.vert",
            "resources/shaders/sun_vox_depth.frag",
        );
        // WGPU Details
        let mut ctx = DEVICE_CONTEXT.write();
        let context = ctx.as_mut().unwrap();

        let uniform_buf = context.device.create_buffer_with_data(
            bytemuck::cast_slice(&[uniforms]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );
        let uniform_bind_group_layout =
            context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    bindings: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                    }],
                    label: None,
                });
        let uniform_bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &uniform_bind_group_layout,
                bindings: &[wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf,
                        range: 0..std::mem::size_of::<SunUniforms>() as wgpu::BufferAddress,
                    },
                }],
                label: None,
            });

        let pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&uniform_bind_group_layout],
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
                    color_states: &vec![],
                    depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                        format: crate::engine::texture::Texture::DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::LessEqual,
                        stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
                        stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
                        stencil_read_mask: 0,
                        stencil_write_mask: 0,
                    }),
                    vertex_state: wgpu::VertexStateDescriptor {
                        index_format: wgpu::IndexFormat::Uint16,
                        vertex_buffers: &[
                            vb.descriptor(),
                            instance_buffer.instance_descriptor(),
                        ],
                    },
                    sample_count: 1,
                    sample_mask: !0,
                    alpha_to_coverage_enabled: false,
                }
            );
        std::mem::drop(ctx);

        // Build the result
        let builder = Self {
            shader_id,
            render_pipeline,
            camera,
            uniforms,
            uniform_bind_group_layout,
            uniform_bind_group,
            uniform_buf
        };
        builder
    }

    pub fn update_uniforms(&mut self, sun_pos: (f32, f32, f32)) {
        self.camera.update(sun_pos);
        self.uniforms.update_view_proj(&self.camera);
        self.uniforms.update_buffer(&self.uniform_buf);
    }

    pub fn render(
        &mut self,
        depth_view: &wgpu::TextureView,
        vertices: &VertexBuffer<f32>,
        instance_buffer: &VertexBuffer<f32>,
        vox_instances: &Vec<(u32, u32, i32)>
    ) {
        let mut ctx_lock = DEVICE_CONTEXT.write();
        let context = ctx_lock.as_mut().unwrap();
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: depth_view,
                    depth_load_op: wgpu::LoadOp::Load,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                }),
            });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.uniform_bind_group, &[]);
            rpass.set_vertex_buffer(0, vertices.buffer.as_ref().unwrap(), 0, 0);
            rpass.set_vertex_buffer(1, instance_buffer.buffer.as_ref().unwrap(), 0, 0);

            // Render it
            if !vox_instances.is_empty() {
                for (count, i) in vox_instances.iter().enumerate() {
                    rpass.draw(i.0..i.1, count as u32..count as u32 + 1);
                }
            }
        }
        context.queue.submit(&[encoder.finish()]);
    }
}

pub struct SunCamera {
    pub eye: Vec3,
    target: Vec3,
    up: Vec3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl SunCamera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            eye: (128.5, 512.0, 128.0).into(),
            target: (128.0, 0.0, 128.0).into(),
            up: Vec3::unit_y(),
            aspect: width as f32 / height as f32,
            fovy: 0.785398,
            znear: 0.1,
            zfar: 512.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at(self.eye, self.target, self.up);
        //let proj = ultraviolet::projection::perspective_gl(self.fovy, self.aspect, self.znear, self.zfar);
        let proj = ultraviolet::projection::orthographic_gl(-128.0, 128.0, -128.0, 128.0, 0.0, 512.0);
        proj * view
    }

    pub fn update(&mut self, sun_pos: (f32, f32, f32)) {
        self.eye = sun_pos.into();
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SunUniforms {
    pub view_proj: Mat4,
}

unsafe impl bytemuck::Pod for SunUniforms {}
unsafe impl bytemuck::Zeroable for SunUniforms {}
impl UniformBlock for SunUniforms {}

impl SunUniforms {
    pub fn new() -> Self {
        Self {
            view_proj: ultraviolet::mat::Mat4::identity(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &SunCamera) {
        self.view_proj = camera.build_view_projection_matrix();
    }
}
