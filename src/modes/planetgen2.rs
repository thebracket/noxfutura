use super::resources::SharedResources;
use crate::engine::uniforms::UniformBlock;
use crate::planet::WORLDGEN_RENDER;
use imgui::*;
use ultraviolet::{
    mat::Mat4,
    vec::{Vec3, Vec4},
};

pub struct PlanetGen2 {
    planet_shader: usize,
    planet_pipeline: Option<wgpu::RenderPipeline>,
    uniform_bind_group: Option<wgpu::BindGroup>,
    uniforms: Option<Uniforms>,
    camera: Option<Camera>,
    uniform_buffer: Option<wgpu::Buffer>,
}

impl PlanetGen2 {
    pub fn new() -> Self {
        Self {
            planet_shader: 0,
            planet_pipeline: None,
            uniform_bind_group: None,
            uniforms: None,
            camera: None,
            uniform_buffer: None,
        }
    }

    pub fn setup(&mut self, context: &mut crate::engine::Context) {
        use crate::engine::*;

        let mut renderlock = WORLDGEN_RENDER.lock();
        renderlock.vertex_buffer.build(
            &context.device,
            wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        );
        self.planet_shader = context.register_shader(
            "resources/shaders/planetgen.vert",
            "resources/shaders/planetgen.frag",
        );

        // Uniforms and camera etc.
        let camera = Camera::new(context.size.width, context.size.height);
        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);
        let (uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
            uniforms.create_buffer_layout_and_group(&context, 0, "some_uniforms");

        // Pipeline
        let pipeline_layout = context.create_pipeline_layout(&[&uniform_bind_group_layout]);
        let render_pipeline = pipelines::RenderPipelineBuilder::new(context.swapchain_format)
            .layout(&pipeline_layout)
            .vf_shader(&context, self.planet_shader)
            .depth_buffer()
            .vertex_state(
                wgpu::IndexFormat::Uint16,
                &[renderlock.vertex_buffer.descriptor()],
            )
            .build(&context.device);
        self.planet_pipeline = Some(render_pipeline);

        // Move
        self.uniform_bind_group = Some(uniform_bind_group);
        self.uniforms = Some(uniforms);
        self.camera = Some(camera);
        self.uniform_buffer = Some(uniform_buffer);
    }

    fn background_and_status(
        &mut self,
        resources: &SharedResources,
        frame: &wgpu::SwapChainOutput,
        context: &mut crate::engine::Context,
        ui: &imgui::Ui,
    ) {
        super::helpers::render_menu_background(context, frame, resources);

        imgui::Window::new(im_str!("Status"))
            .position([10.0, 10.0], Condition::Always)
            .always_auto_resize(true)
            .collapsible(false)
            .build(ui, || {
                ui.text(ImString::new(crate::planet::get_worldgen_status()));
            });
    }

    pub fn tick(
        &mut self,
        resources: &SharedResources,
        frame: &wgpu::SwapChainOutput,
        context: &mut crate::engine::Context,
        ui: &imgui::Ui,
        depth_id: usize,
    ) -> super::ProgramMode {
        use crate::engine::renderpass;
        self.background_and_status(resources, frame, context, ui);

        if let Some(mut uniforms) = self.uniforms {
            uniforms.update_view_proj(self.camera.as_ref().unwrap());
            uniforms.update_buffer(context, self.uniform_buffer.as_ref().unwrap());
        }

        let mut renderlock = WORLDGEN_RENDER.lock();
        if renderlock.needs_update {
            renderlock.vertex_buffer.update_buffer(&context);
            renderlock.needs_update = false;
        }

        let mut encoder = renderpass::get_encoder(&context);
        {
            let mut rpass = renderpass::get_render_pass_with_depth(
                context,
                &mut encoder,
                frame,
                depth_id,
                wgpu::LoadOp::Load,
            );
            rpass.set_pipeline(self.planet_pipeline.as_ref().unwrap());
            rpass.set_bind_group(0, self.uniform_bind_group.as_ref().unwrap(), &[]);
            rpass.set_vertex_buffer(0, &renderlock.vertex_buffer.buffer.as_ref().unwrap(), 0, 0);
            rpass.draw(0..renderlock.vertex_buffer.len(), 0..1);
        }
        context.queue.submit(&[encoder.finish()]);

        std::mem::drop(renderlock);

        super::ProgramMode::PlanetGen2
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
        //self.rot_angle += 0.1;
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
            eye: (0.0, 1.0, 1.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vec3::unit_y(),
            aspect: width as f32 / height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
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
