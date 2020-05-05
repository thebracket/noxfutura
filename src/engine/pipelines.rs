#![allow(dead_code)]
use super::texture;
use super::Context;

pub struct RenderPipelineBuilder<'a> {
    layout: Option<&'a wgpu::PipelineLayout>,
    vertex_shader: Option<wgpu::ProgrammableStageDescriptor<'a>>,
    fragment_shader: Option<wgpu::ProgrammableStageDescriptor<'a>>,
    rasterization_state: wgpu::RasterizationStateDescriptor,
    primitive_topology: wgpu::PrimitiveTopology,
    color_states: Vec<wgpu::ColorStateDescriptor>,
    depth_stencil: Option<wgpu::DepthStencilStateDescriptor>,
    vertex_state: Option<wgpu::VertexStateDescriptor<'a>>,
    sample_count: u32,
    sample_mask: u32,
    alpha_to_coverage_enabled: bool,
}

impl<'a> RenderPipelineBuilder<'a> {
    pub fn new(swapchain_format: wgpu::TextureFormat) -> Self {
        Self {
            layout: None,
            vertex_shader: None,
            fragment_shader: None,
            rasterization_state: wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            },
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: vec![wgpu::ColorStateDescriptor {
                format: swapchain_format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil: None,
            vertex_state: None,
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        }
    }

    pub fn layout(mut self, layout: &'a wgpu::PipelineLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    pub fn vertex_shader(mut self, context: &'a Context, shader_id: usize) -> Self {
        self.vertex_shader = Some(wgpu::ProgrammableStageDescriptor {
            module: &context.shaders[shader_id].vs_module,
            entry_point: "main",
        });
        self
    }

    pub fn fragment_shader(mut self, context: &'a Context, shader_id: usize) -> Self {
        self.fragment_shader = Some(wgpu::ProgrammableStageDescriptor {
            module: &context.shaders[shader_id].fs_module,
            entry_point: "main",
        });
        self
    }

    pub fn vf_shader(self, context: &'a Context, shader_id: usize) -> Self {
        self.vertex_shader(context, shader_id)
            .fragment_shader(context, shader_id)
    }

    pub fn depth_buffer(mut self) -> Self {
        self.depth_stencil = Some(wgpu::DepthStencilStateDescriptor {
            format: texture::Texture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_read_mask: 0,
            stencil_write_mask: 0,
        });
        self
    }

    pub fn vertex_state(
        mut self,
        index_format: wgpu::IndexFormat,
        vbs: &'a [wgpu::VertexBufferDescriptor],
    ) -> Self {
        self.vertex_state = Some(wgpu::VertexStateDescriptor {
            index_format,
            vertex_buffers: vbs,
        });
        self
    }

    pub fn build(self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: self.layout.expect("Must have a pipeline layout"),
            vertex_stage: self.vertex_shader.expect("Must have a vertex shader"),
            fragment_stage: self.fragment_shader,
            rasterization_state: Some(self.rasterization_state),
            primitive_topology: self.primitive_topology,
            color_states: &self.color_states,
            depth_stencil_state: self.depth_stencil,
            vertex_state: self.vertex_state.expect("Must have a vertex state"),
            sample_count: self.sample_count,
            sample_mask: self.sample_mask,
            alpha_to_coverage_enabled: self.alpha_to_coverage_enabled,
        })
    }
}
