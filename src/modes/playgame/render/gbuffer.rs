use bengine::*;

pub struct GBuffer {
    pub albedo: GBufferTarget,
    pub normal: GBufferTarget,
    pub coords: GBufferTarget,
    pub mouse_buffer: gpu::Buffer,
    pub bind_group_layout : gpu::BindGroupLayout,
    pub bind_group : gpu::BindGroup
}

impl GBuffer {
    pub fn new() -> Self {
        let swap_format = RENDER_CONTEXT.read().as_ref().unwrap().swapchain_format;

        let albedo = GBufferTarget::make_texture(
            "Albedo",
            swap_format,
            gpu::TextureUsage::SAMPLED | gpu::TextureUsage::OUTPUT_ATTACHMENT,
        );
        let normal = GBufferTarget::make_texture(
            "Normal",
            swap_format,
            gpu::TextureUsage::SAMPLED | gpu::TextureUsage::OUTPUT_ATTACHMENT,
        );
        let coords = GBufferTarget::make_texture(
            "Coords",
            gpu::TextureFormat::Rgba32Float,
            gpu::TextureUsage::SAMPLED
                | gpu::TextureUsage::OUTPUT_ATTACHMENT
                | gpu::TextureUsage::COPY_SRC,
        );

        let (mouse_buffer, bind_group_layout, bind_group) = {
            let mut ctx_lock = RENDER_CONTEXT.write();
            let context = ctx_lock.as_mut().unwrap();

            let size = 4 * std::mem::size_of::<f32>() as u64;

            let buffer = context.device.create_buffer(&gpu::BufferDescriptor {
                size,
                usage: gpu::BufferUsage::MAP_READ | gpu::BufferUsage::COPY_DST | gpu::BufferUsage::STORAGE,
                label: None,
                mapped_at_creation: false,
            });

            let bind_group_layout =
                context.device
                    .create_bind_group_layout(&gpu::BindGroupLayoutDescriptor {
                        label: None,
                        entries: &[gpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: gpu::ShaderStage::FRAGMENT,
                            ty: gpu::BindingType::StorageBuffer {
                                dynamic: false,
                                min_binding_size: gpu::BufferSize::new(16),
                                readonly: false,
                            },
                            count: None,
                        }],
                    });

            let bind_group = context.device.create_bind_group(&gpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[gpu::BindGroupEntry {
                    binding: 0,
                    resource: gpu::BindingResource::Buffer(buffer.slice(..)),
                }],
            });

            (buffer, bind_group_layout, bind_group)
        };

        Self {
            albedo,
            normal,
            coords,
            mouse_buffer,
            bind_group_layout,
            bind_group
        }
    }
}

pub struct GBufferTarget {
    pub texture: gpu::Texture,
    pub view: gpu::TextureView,
    pub sampler: gpu::Sampler,
    pub dimensions: (usize, usize, usize),
    pub extent: gpu::Extent3d,
}

impl GBufferTarget {
    pub fn make_texture(label: &str, format: gpu::TextureFormat, usage: gpu::TextureUsage) -> Self {
        let mut ctx_lock = RENDER_CONTEXT.write();
        let context = ctx_lock.as_mut().unwrap();

        let size = gpu::Extent3d {
            width: context.size.width as u32,
            height: context.size.height as u32,
            depth: 1,
        };
        let texture = context.device.create_texture(&gpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: gpu::TextureDimension::D2,
            format,
            usage,
        });

        let view = texture.create_view(&gpu::TextureViewDescriptor::default());
        let sampler = context.device.create_sampler(&gpu::SamplerDescriptor {
            address_mode_u: gpu::AddressMode::Repeat,
            address_mode_v: gpu::AddressMode::Repeat,
            address_mode_w: gpu::AddressMode::Repeat,
            mag_filter: gpu::FilterMode::Linear,
            min_filter: gpu::FilterMode::Linear,
            mipmap_filter: gpu::FilterMode::Linear,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: Some(gpu::CompareFunction::Always),
            label: None,
            anisotropy_clamp: None,
        });

        // Return something useful
        Self {
            texture,
            view,
            sampler,
            dimensions: (context.size.width as usize, context.size.height as usize, 1),
            extent: size,
        }
    }
}
