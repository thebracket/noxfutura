use crate::engine::uniforms::UniformBlock;
use super::{
    ChunkModel,
};
use crate::engine::VertexBuffer;
use crate::engine::DEVICE_CONTEXT;
use crate::modes::loader_progress;
use crate::modes::playgame::chunks::Chunks;
use ultraviolet::Mat4;
use ultraviolet::Vec3;

pub struct SunDepthTerrainPass {
    pub vb: VertexBuffer<f32>,
    pub shader_id: usize,
    pub render_pipeline: wgpu::RenderPipeline,
    pub depth_tex : wgpu::Texture,
    pub depth_view : wgpu::TextureView,
    pub depth_sampler : wgpu::Sampler,

    pub camera : SunCamera,
    pub uniforms: SunUniforms,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_buf: wgpu::Buffer,
}

impl SunDepthTerrainPass {
    pub fn new() -> Self {
        let size = crate::engine::get_window_size();
        let camera = SunCamera::new(size.width, size.height);
        let mut uniforms = SunUniforms::new();
        uniforms.update_view_proj(&camera);

        let (depth_tex, depth_view, depth_sampler) = create_depth_texture();

        // Initialize the vertex buffer for cube geometry
        let mut vb = VertexBuffer::<f32>::new(&[3, 1, 2, 1]);
        let mut tmp = 0;
        crate::utils::add_floor_geometry(&mut vb.data, &mut tmp, 1.0, 1.0, 1.0, 1.0, 1.0, 0);
        vb.build(wgpu::BufferUsage::VERTEX);

        loader_progress(0.7, "Lighting the Sun", false);

        // Shader
        let shader_id = crate::engine::register_shader(
            "resources/shaders/sun_terrain_depth.vert",
            "resources/shaders/sun_terrain_depth.frag",
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
        std::mem::drop(ctx);

        // Build the result
        let builder = Self {
            vb,
            shader_id,
            render_pipeline,
            depth_tex,
            depth_view,
            depth_sampler,
            camera,
            uniforms,
            uniform_bind_group_layout,
            uniform_bind_group,
            uniform_buf
        };
        builder
    }

    pub fn render(
        &mut self,
        chunks: &Chunks,
        render_models: &mut Vec<ChunkModel>,
        sun_pos: (f32, f32, f32)
    ) {
        self.camera.update();
        self.uniforms.update_view_proj(&self.camera);
        self.uniforms.update_buffer(&self.uniform_buf);

        let mut ctx_lock = DEVICE_CONTEXT.write();
        let context = ctx_lock.as_mut().unwrap();
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_view,
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

            if self.vb.len() > 0 {
                rpass.set_vertex_buffer(0, &self.vb.buffer.as_ref().unwrap(), 0, 0);
                rpass.draw(0..self.vb.len(), 0..1);
            }

            for chunk in chunks.all_chunks().iter() {
                let buffer = chunk.maybe_render_chunk(512, render_models);
                if let Some(buffer) = buffer {
                    rpass.set_vertex_buffer(0, buffer.0.buffer.as_ref().unwrap(), 0, 0);
                    rpass.draw(0..buffer.1, 0..1);
                }
            }
        }
        context.queue.submit(&[encoder.finish()]);
    }
}

pub fn create_depth_texture() -> (wgpu::Texture, wgpu::TextureView, wgpu::Sampler) {
    let sz = crate::engine::get_window_size();
    let mut ctx_lock = DEVICE_CONTEXT.write();
    let context = ctx_lock.as_mut().unwrap();

    let size = wgpu::Extent3d {
        width: sz.width,
        height: sz.height,
        depth: 1,
    };
    let desc = wgpu::TextureDescriptor {
        label: Some("SunDepth"),
        size,
        array_layer_count: 1,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT
            | wgpu::TextureUsage::SAMPLED
    };
    let texture = context.device.create_texture(&desc);

    let view = texture.create_default_view();
    let sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare: wgpu::CompareFunction::LessEqual,
    });

    (
        texture,
        view,
        sampler,
    )
}

pub struct SunCamera {
    eye: Vec3,
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
        let proj = ultraviolet::projection::orthographic_gl(-128.0, 128.0, -128.0, 128.0, 128.0, 512.0);
        proj * view
    }

    pub fn update(&mut self) {
        self.eye = (128.5, 512.0, 128.0).into();
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
