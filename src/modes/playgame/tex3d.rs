pub struct Texture3D {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub dimensions : (usize, usize, usize)
}

impl Texture3D {
    pub fn blank(
        context: &crate::engine::Context,
        label: Option<&str>,
        width: usize,
        height: usize,
        depth: usize
    ) -> Result<Self, failure::Error> {
        let rgba = vec![255u8; width*height*depth*4];

        let size = wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth: depth as u32,
        };
        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let buffer = context.device.create_buffer_with_data(&rgba, wgpu::BufferUsage::COPY_SRC);

        let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("texture3_buffer_copy_encoder"),
        });

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: 4 * width as u32,
                rows_per_image: depth as u32,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );

        let cmd_buffer = encoder.finish();
        context.queue.submit(&[cmd_buffer]);

        let view = texture.create_default_view();
        let sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Always,
        });

        Ok(
            Self {
                texture,
                view,
                sampler,
                dimensions : (width, height, depth)
            }
        )
    }

    pub fn copy_slice_to_texture(&mut self, context: &crate::engine::Context, rgba : &[u8]) {
        let buffer = context.device.create_buffer_with_data(&rgba, wgpu::BufferUsage::COPY_SRC);

        let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("texture3_buffer_copy_encoder"),
        });

        let size = wgpu::Extent3d {
            width: self.dimensions.0 as u32,
            height: self.dimensions.1 as u32,
            depth: self.dimensions.2 as u32,
        };

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: 4 * size.width,
                rows_per_image: size.height,
            },
            wgpu::TextureCopyView {
                texture: &self.texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );

        let cmd_buffer = encoder.finish();
        context.queue.submit(&[cmd_buffer]);
    }
}
