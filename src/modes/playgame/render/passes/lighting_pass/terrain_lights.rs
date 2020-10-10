use nox_spatial::REGION_TILES_COUNT;
use bengine::*;
use gpu::util::DeviceExt;

pub struct TerrainLights {
    pub flags: Vec<u32>,
    pub storage_buffer: gpu::Buffer,
    pub bind_group_layout: gpu::BindGroupLayout,
    pub bind_group: gpu::BindGroup,
    //pub dirty: bool,
}

impl TerrainLights {
    pub fn new() -> Self {
        let flags = vec![0; REGION_TILES_COUNT];

        let ctl = RENDER_CONTEXT.read();
        let ctx = ctl.as_ref().unwrap();
        let storage_buffer = ctx
            .device
            .create_buffer_init(&gpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&flags),
                usage: gpu::BufferUsage::STORAGE,
            });

        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&gpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[gpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: gpu::ShaderStage::FRAGMENT,
                        ty: gpu::BindingType::StorageBuffer {
                            dynamic: false,
                            min_binding_size: gpu::BufferSize::new(64),
                            readonly: true,
                        },
                        count: None,
                    }],
                });

        let bind_group = ctx.device.create_bind_group(&gpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[gpu::BindGroupEntry {
                binding: 0,
                resource: gpu::BindingResource::Buffer(storage_buffer.slice(..)),
            }],
        });

        Self {
            flags,
            storage_buffer,
            bind_group_layout,
            bind_group,
            //dirty: true,
        }
    }

    pub fn update_buffer(&mut self) {
        let ctl = RENDER_CONTEXT.read();
        let ctx = ctl.as_ref().unwrap();

        let storage_buffer = ctx
            .device
            .create_buffer_init(&gpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.flags),
                usage: gpu::BufferUsage::STORAGE,
            });

        let bind_group = ctx.device.create_bind_group(&gpu::BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_layout,
            entries: &[gpu::BindGroupEntry {
                binding: 0,
                resource: gpu::BindingResource::Buffer(storage_buffer.slice(..)),
            }],
        });

        self.storage_buffer = storage_buffer;
        self.bind_group = bind_group;
    }
}
