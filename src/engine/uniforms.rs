use super::Context;

pub trait UniformBlock<T: bytemuck::Pod = Self>: bytemuck::Pod {
    fn create_buffer_with_data(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_with_data(
            bytemuck::cast_slice(&[*self]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        )
    }

    fn create_bindgroup_layout(&self, device: &wgpu::Device, binding: u32, label: &str) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutEntry {
                        binding,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::UniformBuffer {
                            dynamic: false,
                        },
                    }
                ],
                label: Some(label),
            }
        )
    }

    fn create_bind_group(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        binding: u32, 
        buffer: &wgpu::Buffer,
        label: &str
    ) -> wgpu::BindGroup
    {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            bindings: &[
                wgpu::Binding {
                    binding,
                    resource: wgpu::BindingResource::Buffer {
                        buffer,
                        range: 0..std::mem::size_of_val(self) as wgpu::BufferAddress,
                    }
                }
            ],
            label: Some(label),
        })
    }

    fn update_buffer(&self, context : &Context, uniform_buffer : &wgpu::Buffer) {
        let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("update encoder"),
        });

        let staging_buffer = context.device.create_buffer_with_data(
            bytemuck::cast_slice(&[*self]),
            wgpu::BufferUsage::COPY_SRC,
        );

        encoder.copy_buffer_to_buffer(&staging_buffer, 0, &uniform_buffer, 0, std::mem::size_of_val(self) as wgpu::BufferAddress);

        context.queue.submit(&[encoder.finish()]);
    }
}
