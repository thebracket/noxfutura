pub struct SharedResources {
    pub background_image: usize,
    pub quad_vb: crate::engine::VertexBuffer<f32>,
    pub quad_tex_shader: usize,
    pub quad_pipeline: Option<wgpu::RenderPipeline>,
    pub quad_bind_group: Option<wgpu::BindGroup>,
}

impl SharedResources {
    pub fn new() -> Self {
        Self {
            background_image: 0,
            quad_vb: crate::engine::VertexBuffer::<f32>::new(&[2, 2]),
            quad_tex_shader: 0,
            quad_pipeline: None,
            quad_bind_group: None,
        }
    }

    pub fn init(&mut self, context: &mut crate::engine::Context) {
        // Load the background image
        self.background_image = context.register_texture(
            include_bytes!("../../resources/images/background_image.png"),
            "NF Background",
        );

        /*context.register_texture(
            include_bytes!("../../resources/avon-and-guards.png"),
            "Kerr Avon",
        );*/

        // Setup the helper quad VB
        self.quad_vb.add_slice(&[
            -1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
        ]);
        self.quad_vb
            .build(&context.device, wgpu::BufferUsage::VERTEX);

        // Quad shader
        self.quad_tex_shader = context.register_shader(
            "resources/shaders/quad_tex.vert",
            "resources/shaders/quad_tex.frag",
        );

        // Texture buffer descriptions
        let texture_bind_group_layout =
            context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    bindings: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: wgpu::TextureViewDimension::D2,
                                component_type: wgpu::TextureComponentType::Uint,
                            },
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Sampler { comparison: false },
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                });

        let diffuse_bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &context.textures[self.background_image].view,
                        ),
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(
                            &context.textures[self.background_image].sampler,
                        ),
                    },
                ],
                label: Some("diffuse_bind_group"),
            });
        self.quad_bind_group = Some(diffuse_bind_group);

        // Pipeline for displaying the quad
        let pipeline_layout = context.create_pipeline_layout(&[&texture_bind_group_layout]);
        let render_pipeline =
            crate::engine::pipelines::RenderPipelineBuilder::new(context.swapchain_format)
                .layout(&pipeline_layout)
                .vf_shader(&context, self.quad_tex_shader)
                .vertex_state(wgpu::IndexFormat::Uint16, &[self.quad_vb.descriptor()])
                .build(&context.device);
        self.quad_pipeline = Some(render_pipeline);
    }
}
