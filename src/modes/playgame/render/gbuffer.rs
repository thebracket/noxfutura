use bengine::*;

pub struct GBuffer {
    pub albedo: GBufferTarget,
    pub normal: GBufferTarget,
    pub pbr: GBufferTarget,
    pub coords: GBufferTarget,
    pub mouse_buffer: gpu::Buffer,
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
            gpu::TextureFormat::Rgba32Float,
            gpu::TextureUsage::SAMPLED | gpu::TextureUsage::OUTPUT_ATTACHMENT,
        );
        let pbr = GBufferTarget::make_texture(
            "PBR",
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

        let mouse_buffer = {
            let mut ctx_lock = RENDER_CONTEXT.write();
            let context = ctx_lock.as_mut().unwrap();

            let size = context.size.width as u64
                * context.size.height as u64
                * 4
                * std::mem::size_of::<f32>() as u64;

            context.device.create_buffer(&gpu::BufferDescriptor {
                size,
                usage: gpu::BufferUsage::MAP_READ | gpu::BufferUsage::COPY_DST,
                label: None,
                mapped_at_creation: false,
            })
        };

        Self {
            albedo,
            normal,
            pbr,
            coords,
            mouse_buffer,
        }
    }

    pub fn copy_mouse_buffer(&self) {
        let mut ctx_lock = RENDER_CONTEXT.write();
        let context = ctx_lock.as_mut().unwrap();

        let command_buffer = {
            let mut encoder = context
                .device
                .create_command_encoder(&gpu::CommandEncoderDescriptor { label: None });
            encoder.copy_texture_to_buffer(
                gpu::TextureCopyView {
                    texture: &self.coords.texture,
                    mip_level: 0,
                    origin: gpu::Origin3d::ZERO,
                },
                gpu::BufferCopyView {
                    buffer: &self.mouse_buffer,
                    layout: gpu::TextureDataLayout {
                        offset: 0,
                        bytes_per_row: context.size.width as u32
                            * 4
                            * std::mem::size_of::<f32>() as u32,
                        rows_per_image: 0,
                    },
                },
                self.coords.extent,
            );
            encoder.finish()
        };
        context.queue.submit(Some(command_buffer));
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
