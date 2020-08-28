use crate::{FloatBuffer, RENDER_CONTEXT, SHADERS};

pub fn pipeline_layout(entries: &[&wgpu::BindGroupLayout], label: &str) -> wgpu::PipelineLayout {
    RENDER_CONTEXT
        .read()
        .as_ref()
        .unwrap()
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(label),
            bind_group_layouts: entries,
            push_constant_ranges: &[],
        })
}

pub fn render_pipeline_simple(
    label: &str,
    layout: &wgpu::PipelineLayout,
    vertex_shader_id: usize,
    fragment_shader_id: usize,
    buffer: &FloatBuffer<f32>,
) -> wgpu::RenderPipeline {
    let rcl = RENDER_CONTEXT.read();
    let rc = rcl.as_ref().unwrap();

    let shaders = SHADERS.read();

    rc.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: Some(layout),
            label: Some(label),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: shaders.get_module(vertex_shader_id),
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: shaders.get_module(fragment_shader_id),
                entry_point: "main",
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
                vertex_buffers: &[buffer.descriptor()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        })
}
