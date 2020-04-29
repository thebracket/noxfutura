pub fn create_buffer_with_data<U : bytemuck::Pod>(device: &wgpu::Device, uniform : &U) -> wgpu::Buffer {
    device.create_buffer_with_data(
        bytemuck::cast_slice(&[*uniform]),
        wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    )
}

pub fn create_uniform_bindgroup_layout(device: &wgpu::Device, binding: u32) -> wgpu::BindGroupLayout {
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
            label: Some("uniform_bind_group_layout"),
        }
    )
}

pub fn create_uniform_bind_group<U : bytemuck::Pod>(
    device: &wgpu::Device, 
    layout: &wgpu::BindGroupLayout,
    binding: u32, 
    buffer: &wgpu::Buffer,
    uniform: &U
) -> wgpu::BindGroup
{
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &layout,
        bindings: &[
            wgpu::Binding {
                binding,
                resource: wgpu::BindingResource::Buffer {
                    buffer: buffer,
                    range: 0..std::mem::size_of_val(uniform) as wgpu::BufferAddress,
                }
            }
        ],
        label: Some("uniform_bind_group"),
    })
}