pub struct VertexBuffer<T>
where
    T: bytemuck::Pod,
{
    pub data: Vec<T>,
    pub attributes: Vec<wgpu::VertexAttributeDescriptor>,
    total_size: wgpu::BufferAddress,
    row_len: usize,
    pub buffer: Option<wgpu::Buffer>,
}

impl<T> VertexBuffer<T>
where
    T: bytemuck::Pod,
{
    pub fn new(layout: &[usize]) -> Self {
        let mut attributes = Vec::new();

        let mut cumulative_len = 0;
        let mut cumulative_size = 0;
        for (i, size) in layout.iter().enumerate() {
            let attribute = wgpu::VertexAttributeDescriptor {
                offset: cumulative_size,
                shader_location: i as u32,
                format: match size {
                    1 => wgpu::VertexFormat::Float,
                    2 => wgpu::VertexFormat::Float2,
                    3 => wgpu::VertexFormat::Float3,
                    4 => wgpu::VertexFormat::Float4,
                    _ => {
                        panic!("Vertices must be 1-4 floats");
                    }
                },
            };
            attributes.push(attribute);
            cumulative_size += (std::mem::size_of::<T>() * size) as wgpu::BufferAddress;
            cumulative_len += size;
        }

        Self {
            data: Vec::new(),
            attributes,
            total_size: cumulative_size,
            row_len: cumulative_len,
            buffer: None,
        }
    }

    pub fn descriptor(&self) -> wgpu::VertexBufferDescriptor {
        wgpu::VertexBufferDescriptor {
            stride: self.total_size,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &self.attributes,
        }
    }

    pub fn instance_descriptor(&self) -> wgpu::VertexBufferDescriptor {
        wgpu::VertexBufferDescriptor {
            stride: self.total_size,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &self.attributes,
        }
    }

    pub fn build(&mut self, usage: wgpu::BufferUsage) {
        use crate::DEVICE_CONTEXT;
        let mut ctx = DEVICE_CONTEXT.write();
        let context = ctx.as_mut().unwrap();
        let device = &context.device;
        if let Some(buf) = &mut self.buffer {
            std::mem::drop(buf);
        }
        self.buffer = Some(device.create_buffer_with_data(bytemuck::cast_slice(&self.data), usage));
    }

    pub fn update_buffer(&mut self) {
        self.build(wgpu::BufferUsage::VERTEX);
    }

    pub fn len(&self) -> u32 {
        (self.data.len() / self.row_len) as u32
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn add_slice(&mut self, slice: &[T]) {
        self.data.extend_from_slice(slice);
    }

    pub fn add(&mut self, f: T) {
        self.data.push(f);
    }

    pub fn add2(&mut self, f: T, f1: T) {
        self.data.push(f);
        self.data.push(f1);
    }

    pub fn add3(&mut self, f: T, f1: T, f2: T) {
        self.data.push(f);
        self.data.push(f1);
        self.data.push(f2);
    }

    pub fn add4(&mut self, f: T, f1: T, f2: T, f3: T) {
        self.data.push(f);
        self.data.push(f1);
        self.data.push(f2);
        self.data.push(f3);
    }
}
