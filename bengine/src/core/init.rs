use wgpu::{Device, Queue};
use crate::{Textures, Shaders, RENDER_CONTEXT, Buffers};

pub struct Initializer<'a> {
    textures: &'a mut Textures,
    shaders: &'a mut Shaders,
    buffers: &'a mut Buffers,
}

impl<'a> Initializer<'a> {
    pub(crate) fn new(
        textures: &'a mut Textures,
        shaders: &'a mut Shaders,
        buffers: &'a mut Buffers,
    ) -> Self {
        Self {
            textures,
            shaders,
            buffers,
        }
    }

    pub fn load_texture_from_bytes(&mut self, bytes: &[u8]) -> usize {
        println!("Call load_texture_from_bytes");
        self.textures.load_texture_from_bytes(
            bytes,
            "Background"
        )
    }

    pub fn load_shader_from_include(&mut self, source: wgpu::ShaderModuleSource) -> usize {
        self.shaders.register_include(source)
    }

    pub fn make_empty_buffer(&mut self, layout: &[usize], capacity: usize, usage: wgpu::BufferUsage) -> usize {
        self.buffers.init_buffer(layout, capacity, usage)
    }

    pub fn make_buffer_with_data(&mut self, layout: &[usize], capacity: usize, usage: wgpu::BufferUsage, data: &[f32]) -> usize {
        let idx = self.buffers.init_buffer(layout, capacity, usage);
        let buf = self.buffers.get_buffer(idx);
        buf.add_slice(data);
        buf.build();
        idx
    }

    pub fn simple_texture_bg_layout(&mut self, label: &str) -> wgpu::BindGroupLayout {
        crate::simple_texture_bg_layout(label)
    }

    pub fn simple_texture_bg(&mut self, layout: &wgpu::BindGroupLayout, texture_id: usize) -> wgpu::BindGroup {
        crate::simple_texture_bg(&self.textures, layout, texture_id)
    }

    pub fn pipeline_layout(&mut self, entries: &[&wgpu::BindGroupLayout], label: &str) -> wgpu::PipelineLayout {
        RENDER_CONTEXT.read().as_ref().unwrap().device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(label),
            bind_group_layouts: entries,
            push_constant_ranges: &[]
        })
    }

    pub fn render_pipeline_simple(
        &mut self, 
        label: &str, 
        layout: &wgpu::PipelineLayout, 
        vertex_shader_id: usize, 
        fragment_shader_id: usize,
        buf_id: usize
    ) -> wgpu::RenderPipeline {
        let rcl = RENDER_CONTEXT.read();
        let rc = rcl.as_ref().unwrap();

        rc
            .device
            .create_render_pipeline(
                &wgpu::RenderPipelineDescriptor{
                    layout: Some(layout),
                    label: Some(label),
                    vertex_stage: wgpu::ProgrammableStageDescriptor {
                        module: self.shaders.get_module(vertex_shader_id),
                        entry_point: "main"
                    },
                    fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                        module: self.shaders.get_module(fragment_shader_id),
                        entry_point: "main"
                    }),
                    rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: wgpu::CullMode::Back,
                        ..Default::default()
                    }),
                    primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                    color_states: &vec![wgpu::ColorStateDescriptor {
                        format: rc.swapchain_format,
                        color_blend: wgpu::BlendDescriptor::REPLACE,
                        alpha_blend: wgpu::BlendDescriptor::REPLACE,
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                    depth_stencil_state: None,
                    vertex_state: wgpu::VertexStateDescriptor {
                        index_format: wgpu::IndexFormat::Uint16,
                        vertex_buffers: &[self.buffers.get_descriptor(buf_id)],
                    },
                    sample_count: 1,
                    sample_mask: !0,
                    alpha_to_coverage_enabled: false,
                }
            )
    }
}