use crate::engine::DEVICE_CONTEXT;

pub struct GBuffer {
    pub albedo: GBufferTarget,
    pub normal: GBufferTarget,
    pub pbr: GBufferTarget,
    pub coords: GBufferTarget,
    pub mouse_buffer: wgpu::Buffer,
}

impl GBuffer {
    pub fn new() -> Self {
        let swap_format = DEVICE_CONTEXT.read().as_ref().unwrap().swapchain_format;

        let albedo = GBufferTarget::make_texture("Albedo", swap_format);
        let normal = GBufferTarget::make_texture("Normal", wgpu::TextureFormat::Rgba32Float);
        let pbr = GBufferTarget::make_texture("PBR", swap_format);
        let coords = GBufferTarget::make_texture("Coords", wgpu::TextureFormat::Rgba32Float);

        let mouse_buffer = {
            let mut ctx_lock = DEVICE_CONTEXT.write();
            let context = ctx_lock.as_mut().unwrap();

            let size = context.size.width as u64
                * context.size.height as u64
                * 4
                * std::mem::size_of::<f32>() as u64;

            context.device.create_buffer(&wgpu::BufferDescriptor {
                size,
                usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
                label: None,
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
        let mut ctx_lock = DEVICE_CONTEXT.write();
        let context = ctx_lock.as_mut().unwrap();
        //let size = context.size.width as u64 * context.size.height as u64 * 4 * std::mem::size_of::<f32>() as u64;

        let command_buffer = {
            let mut encoder = context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            encoder.copy_texture_to_buffer(
                wgpu::TextureCopyView {
                    texture: &self.coords.texture,
                    mip_level: 0,
                    array_layer: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                wgpu::BufferCopyView {
                    buffer: &self.mouse_buffer,
                    offset: 0,
                    bytes_per_row: context.size.width as u32
                        * 4
                        * std::mem::size_of::<f32>() as u32,
                    rows_per_image: 0,
                },
                self.coords.extent,
            );
            encoder.finish()
        };
        context.queue.submit(&[command_buffer]);
    }
}

pub struct GBufferTarget {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub dimensions: (usize, usize, usize),
    pub extent: wgpu::Extent3d,
}

impl GBufferTarget {
    pub fn make_texture(label: &str, format: wgpu::TextureFormat) -> Self {
        let mut ctx_lock = DEVICE_CONTEXT.write();
        let context = ctx_lock.as_mut().unwrap();

        let size = wgpu::Extent3d {
            width: context.size.width as u32,
            height: context.size.height as u32,
            depth: 1,
        };
        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsage::SAMPLED
                | wgpu::TextureUsage::OUTPUT_ATTACHMENT
                | wgpu::TextureUsage::COPY_SRC,
        });

        let view = texture.create_default_view();
        let sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Always,
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
