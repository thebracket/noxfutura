use super::resources::SharedResources;
use crate::engine::uniforms::UniformBlock;
use crate::engine::DEVICE_CONTEXT;
use cgmath::{EuclideanSpace, Matrix4, Point3, SquareMatrix, Vector3};
use imgui::*;
use nox_planet::WORLDGEN_RENDER;

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

    pub fn setup(&mut self) {
        use crate::engine::*;

        let mut renderlock = WORLDGEN_RENDER.lock();
        renderlock.vertex_buffer.build(wgpu::BufferUsage::VERTEX);

        self.planet_shader = crate::engine::register_shader(
            "resources/shaders/planetgen.vert",
            "resources/shaders/planetgen.frag",
        );

        // Uniforms and camera etc.
        let size = get_window_size();
        let mut ctx = DEVICE_CONTEXT.write();
        let context = ctx.as_mut().unwrap();

        self.camera = Some(Camera::new(size.width, size.height));
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
                &[renderlock.vertex_buffer.descriptor()],
            )
            .build(&context.device);
        self.planet_pipeline = Some(render_pipeline);
    }

    fn background_and_status(
        &mut self,
        resources: &SharedResources,
        frame: &wgpu::SwapChainOutput,
        ui: &imgui::Ui,
    ) {
        super::helpers::render_menu_background(frame, resources);

        imgui::Window::new(im_str!("Status"))
            .position([10.0, 10.0], Condition::Always)
            .always_auto_resize(true)
            .collapsible(false)
            .build(ui, || {
                ui.text(ImString::new(nox_planet::get_worldgen_status()));
            });
    }

    pub fn tick(
        &mut self,
        resources: &SharedResources,
        frame: &wgpu::SwapChainOutput,
        ui: &imgui::Ui,
        depth_id: usize,
    ) -> super::ProgramMode {
        use crate::engine::renderpass;
        self.background_and_status(resources, frame, ui);

        if let Some(uniforms) = self.uniforms.as_mut() {
            uniforms.update_view_proj(self.camera.as_ref().unwrap());
            uniforms.update_buffer(self.uniform_buffer.as_ref().unwrap());
        }

        let mut renderlock = WORLDGEN_RENDER.lock();
        if renderlock.needs_update {
            renderlock.vertex_buffer.update_buffer();
            renderlock.needs_update = false;
        }

        let mut ctx = DEVICE_CONTEXT.write();
        let context = ctx.as_mut().unwrap();

        if renderlock.vertex_buffer.len() > 0 {
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
                rpass.set_vertex_buffer(
                    0,
                    &renderlock.vertex_buffer.buffer.as_ref().unwrap(),
                    0,
                    0,
                );
                rpass.draw(0..renderlock.vertex_buffer.len(), 0..1);
                //rpass.draw(0..1, 0..1);
            }
            context.queue.submit(&[encoder.finish()]);
        }

        if nox_planet::PLANET_BUILD.lock().done {
            return super::ProgramMode::MainMenu;
        }

        std::mem::drop(renderlock);

        super::ProgramMode::PlanetGen2
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Uniforms {
    view_proj: Matrix4<f32>,
    rot_angle: f32,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}
impl UniformBlock for Uniforms {}

impl Uniforms {
    fn new() -> Self {
        Self {
            view_proj: Matrix4::identity(),
            rot_angle: 0.0,
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix();
        self.rot_angle += 0.001;
    }
}

pub struct Camera {
    eye: Vector3<f32>,
    target: Vector3<f32>,
    up: Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            eye: (0.0, 2.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vector3::unit_y(),
            aspect: width as f32 / height as f32,
            fovy: 0.785398,
            znear: 0.01,
            zfar: 500.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at(
            Point3::from_vec(self.eye),
            Point3::from_vec(self.target),
            self.up,
        );
        let proj = cgmath::perspective(cgmath::Rad(self.fovy), self.aspect, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}
