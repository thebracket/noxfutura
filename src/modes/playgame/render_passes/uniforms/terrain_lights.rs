use crate::planet::REGION_TILES_COUNT;
use crate::engine::DEVICE_CONTEXT;
use zerocopy::AsBytes as _;

pub struct TerrainLights {
    pub flags: Vec<u32>,
    pub staging_buffer : wgpu::Buffer,
    pub storage_buffer : wgpu::Buffer,
    pub bind_group_layout : wgpu::BindGroupLayout,
    pub bind_group : wgpu::BindGroup
}

impl TerrainLights {
    pub fn new() -> Self {
        let flags = vec![0; REGION_TILES_COUNT];

        let mut ctx = DEVICE_CONTEXT.write();
        let context = ctx.as_mut().unwrap();
        let size = (std::mem::size_of::<u32>() * REGION_TILES_COUNT) as wgpu::BufferAddress;

        let staging_buffer = context.device.create_buffer_with_data(
            flags.as_slice().as_bytes(),
            wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC,
        );

        let storage_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            size,
            usage: wgpu::BufferUsage::STORAGE
                | wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::COPY_SRC,
            label: None,
        });

        let bind_group_layout = context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::StorageBuffer {
                    dynamic: false,
                    readonly: false,
                },
            }],
            label: None,
        });

        let bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &storage_buffer,
                    range: 0 .. size,
                },
            }],
            label: None,
        });

        let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(&staging_buffer, 0, &storage_buffer, 0, size);
        context.queue.submit(&[encoder.finish()]);

        Self {
            flags,
            staging_buffer,
            storage_buffer,
            bind_group_layout,
            bind_group
        }
    }
}