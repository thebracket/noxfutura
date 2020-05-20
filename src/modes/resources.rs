use crate::opengl::*;

pub struct SharedResources {
    pub background_image: Option<Texture>,
    pub quad_shader: Option<Shader>,
    pub quad_vao: Option<VertexArray>
    /*pub quad_vb: crate::engine::VertexBuffer<f32>,
    pub quad_tex_shader: usize,
    pub quad_pipeline: Option<wgpu::RenderPipeline>,
    pub quad_bind_group: Option<wgpu::BindGroup>,*/
}

impl SharedResources {
    pub fn new() -> Self {
        Self {
            background_image: None,
            quad_shader: None,
            quad_vao: None
        }
    }

    pub fn init(&mut self, gl: &Gl) {
        self.background_image = Some(Texture::from_file(gl, "resources/images/background_image.png"));
        self.quad_shader = Some(Shader::new(
            gl,
            include_str!("../../resources/shaders/quad_tex.vert"),
            include_str!("../../resources/shaders/quad_tex.frag")
        ));
        const QUAD_ENTRIES : [f32; 24] = [
            -1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
        ];
        let mut quad_vao = VertexArray::float_builder(
            gl,
            &[
                VertexArrayEntry{index: 0, size: 2},
                VertexArrayEntry{index: 1, size: 2},
            ],
            24
        );
        for f in QUAD_ENTRIES.iter() {
            quad_vao.vertex_buffer.push(*f);
        }
        quad_vao.upload_buffers(gl);
        self.quad_vao = Some(quad_vao);
    }
}
