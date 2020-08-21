use super::Context;
use crate::DEVICE_CONTEXT;

pub trait UniformBlock<T: bytemuck::Pod = Self>: bytemuck::Pod {
    fn create_buffer_with_data(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_with_data(
            bytemuck::cast_slice(&[*self]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        )
    }

    fn create_bindgroup_layout(
        &self,
        device: &wgpu::Device,
        binding: u32,
        label: &str,
    ) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            }],
            label: Some(label),
        })
    }

    fn create_bind_group(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        binding: u32,
        buffer: &wgpu::Buffer,
        label: &str,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            bindings: &[wgpu::Binding {
                binding,
                resource: wgpu::BindingResource::Buffer {
                    buffer,
                    range: 0..std::mem::size_of_val(self) as wgpu::BufferAddress,
                },
            }],
            label: Some(label),
        })
    }

    fn update_buffer(&self, uniform_buffer: &wgpu::Buffer) {
        let mut ctx = DEVICE_CONTEXT.write();
        let context = ctx.as_mut().unwrap();
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("update encoder"),
            });

        let staging_buffer = context
            .device
            .create_buffer_with_data(bytemuck::cast_slice(&[*self]), wgpu::BufferUsage::COPY_SRC);

        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &uniform_buffer,
            0,
            std::mem::size_of_val(self) as wgpu::BufferAddress,
        );

        context.queue.submit(&[encoder.finish()]);
        staging_buffer.unmap();
        std::mem::drop(staging_buffer);
    }

    fn create_buffer_layout_and_group(
        &self,
        context: &Context,
        binding: u32,
        label: &str,
    ) -> (wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup) {
        let uniform_buffer = self.create_buffer_with_data(&context.device);
        let uniform_bind_group_layout =
            self.create_bindgroup_layout(&context.device, binding, label);
        let uniform_bind_group = self.create_bind_group(
            &context.device,
            &uniform_bind_group_layout,
            binding,
            &uniform_buffer,
            label,
        );
        (
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
        )
    }
}
