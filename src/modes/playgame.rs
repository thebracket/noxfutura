use super::resources::SharedResources;
use crate::engine::uniforms::UniformBlock;
use crate::modes::WORLDGEN_RENDER;
use imgui::*;
use ultraviolet::{
    mat::Mat4,
    vec::{Vec3, Vec4},
};
use crate::planet::{Planet, Region};
use crate::engine::*;

pub struct PlayGame {
    pub planet : Option<Planet>,
    pub current_region : Option<Region>,

    // Internals
    planet_shader: usize,
    planet_pipeline: Option<wgpu::RenderPipeline>,
    uniform_bind_group: Option<wgpu::BindGroup>,
    uniforms: Option<Uniforms>,
    camera: Option<Camera>,
    uniform_buffer: Option<wgpu::Buffer>,
    vb : VertexBuffer<f32>
}

impl PlayGame {
    pub fn new() -> Self {
        Self {
            planet: None,
            current_region : None,
            planet_shader : 0,
            planet_pipeline: None,
            uniform_bind_group: None,
            uniforms: None,
            camera: None,
            uniform_buffer: None,
            vb: VertexBuffer::new(&[3, 4])
        }
    }

    pub fn load(&mut self) {
        println!("Loading game");
        let lg = crate::planet::load_game();
        self.planet = Some(lg.planet);
        self.current_region = Some(lg.current_region);
        println!("Loaded game");
    }

    pub fn setup(&mut self, context: &mut crate::engine::Context) {

        crate::utils::add_cube_geometry(&mut self.vb, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        self.vb.build(&context.device, wgpu::BufferUsage::VERTEX);

        self.planet_shader = context.register_shader(
            "resources/shaders/regionblocks.vert",
            "resources/shaders/regionblocks.frag",
        );

        // Uniforms and camera etc.
        self.camera = Some(Camera::new(context.size.width, context.size.height));
        self.uniforms = Some(Uniforms::new());
        self.uniforms
            .as_mut()
            .unwrap()
            .update_view_proj(self.camera.as_ref().unwrap());
        let (uniform_buffer, uniform_bind_group_layout, uniform_bind_group) = self
            .uniforms
            .as_mut()
            .unwrap()
            .create_buffer_layout_and_group(&context, 0, "some_uniforms");
        self.uniform_bind_group = Some(uniform_bind_group);
        self.uniform_buffer = Some(uniform_buffer);

        // Pipeline
        let pipeline_layout = context.create_pipeline_layout(&[&uniform_bind_group_layout]);
        let render_pipeline = pipelines::RenderPipelineBuilder::new(context.swapchain_format)
            .layout(&pipeline_layout)
            .vf_shader(&context, self.planet_shader)
            .depth_buffer()
            .vertex_state(
                wgpu::IndexFormat::Uint16,
                &[self.vb.descriptor()],
            )
            .build(&context.device);
        self.planet_pipeline = Some(render_pipeline);
    }

    pub fn tick(
        &mut self,
        _resources: &SharedResources,
        frame: &wgpu::SwapChainOutput,
        context: &mut crate::engine::Context,
        _ui: &imgui::Ui,
        depth_id: usize,
    ) -> super::ProgramMode {
        self.uniforms.as_mut().unwrap().update_view_proj(&self.camera.as_ref().unwrap());
        self.uniforms.as_ref().unwrap().update_buffer(context, &self.uniform_buffer.as_ref().unwrap());
        self.vb.update_buffer(context);

        if self.vb.len() > 0 {
            let mut encoder = renderpass::get_encoder(&context);
            {
                let mut rpass = renderpass::get_render_pass_with_depth(
                    context,
                    &mut encoder,
                    frame,
                    depth_id,
                    wgpu::LoadOp::Load,
                );
                rpass.set_pipeline(&self.planet_pipeline.as_ref().unwrap());
                rpass.set_bind_group(0, &self.uniform_bind_group.as_ref().unwrap(), &[]);
                rpass.set_vertex_buffer(
                    0,
                    &self.vb.buffer.as_ref().unwrap(),
                    0,
                    0,
                );
                rpass.draw(0..self.vb.len(), 0..1);
                //rpass.draw(0..1, 0..1);
            }
            context.queue.submit(&[encoder.finish()]);
        }

        super::ProgramMode::PlayGame
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Uniforms {
    view_proj: ultraviolet::mat::Mat4,
    rot_angle: f32,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}
impl UniformBlock for Uniforms {}

impl Uniforms {
    fn new() -> Self {
        Self {
            view_proj: ultraviolet::mat::Mat4::identity(),
            rot_angle: 0.0,
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix();
        self.rot_angle += 0.001;
    }
}

pub struct Camera {
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            eye: (0.0, 2.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vec3::unit_y(),
            aspect: width as f32 / height as f32,
            fovy: 0.785398,
            znear: 0.01,
            zfar: 500.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let opengl_to_wgpu_matrix : Mat4 = Mat4::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.5, 0.0),
            Vec4::new(0.0, 0.0, 0.5, 1.0),
        );

        let view = Mat4::look_at(self.eye, self.target, self.up);
        let proj =
            ultraviolet::projection::perspective_gl(self.fovy, self.aspect, self.znear, self.zfar);
        opengl_to_wgpu_matrix * proj * view
    }
}
