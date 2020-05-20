use crate::opengl::*;
use super::resources::SharedResources;
use crate::planet::WORLDGEN_RENDER;
use imgui::*;
use ultraviolet::{
    mat::Mat4,
    vec::{Vec3, Vec4},
};
use std::ffi::CString;

pub struct PlanetGen2 {
    shader: Option<Shader>,
    camera: Option<Camera>,
    uniforms: Option<Uniforms>,
    vertex_buffer: Option<VertexArray>
}

impl PlanetGen2 {
    pub fn new() -> Self {
        Self {
            shader: None,
            camera: None,
            uniforms: None,
            vertex_buffer: None
        }
    }

    pub fn init(&mut self, gl: &Gl) {

        self.shader = Some(Shader::new(
            gl,
            include_str!("../../resources/shaders/planetgen.vert"),
            include_str!("../../resources/shaders/planetgen.frag")
        ));

        // Uniforms and camera etc.
        self.camera = Some(Camera::new(800, 600));
        self.uniforms = Some(Uniforms::new());
    }

    fn background_and_status(
        &mut self,
        resources: &SharedResources,
        gl: &Gl,
        ui: &imgui::Ui,
    ) {
        super::helpers::render_menu_background(gl, resources);

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
        gl: &Gl,
        resources: &SharedResources,
        ui: &imgui::Ui,
    ) -> super::ProgramMode {
        self.background_and_status(resources, gl, ui);

        if let Some(uniforms) = self.uniforms.as_mut() {
            if !crate::planet::get_flatmap_status() {
                uniforms.update_view_proj(self.camera.as_ref().unwrap());
            } else {
                uniforms.update_view_proj_flat(self.camera.as_mut().unwrap());
            }
            uniforms.update_uniforms(gl, self.shader.as_ref().unwrap());
        }


        if self.vertex_buffer.is_none() {
            self.vertex_buffer = Some(
                VertexArray::float_builder(gl, &[VertexArrayEntry{index: 0, size: 3}, VertexArrayEntry{index:1, size: 4}], 2000)
            );
        }

        let mut renderlock = WORLDGEN_RENDER.lock();
        if renderlock.needs_update {
            if let Some(vb) = &mut self.vertex_buffer {
                vb.vertex_buffer.clear();
                for v in &renderlock.vertex_buffer {
                    vb.vertex_buffer.push(*v);
                }
                vb.upload_buffers(gl);
                renderlock.needs_update = false;
            }
        }

        if self.vertex_buffer.as_ref().unwrap().vertex_buffer.len() > 0 {
            self.vertex_buffer.as_ref().unwrap().draw_elements_no_texture(gl, &self.shader.as_ref().unwrap());
        }

        std::mem::drop(renderlock);

        super::ProgramMode::PlanetGen2
    }
}

#[derive(Debug, Copy, Clone)]
struct Uniforms {
    view_proj: ultraviolet::mat::Mat4,
    rot_angle: f32,
}

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

    fn update_view_proj_flat(&mut self, camera: &mut Camera) {
        camera.down_cam();
        self.rot_angle = 0.0;
        self.view_proj = camera.build_view_projection_matrix();
    }

    fn update_uniforms(&self, gl: &Gl, shader: &Shader) {
        shader.activate(gl);
        unsafe {
            let matloc = gl.GetUniformLocation(shader.0, CString::new("u_view_proj").unwrap().as_ptr());
            let rotloc = gl.GetUniformLocation(shader.0, CString::new("rot_angle").unwrap().as_ptr());
            gl.UniformMatrix4fv(matloc, 1, 0, self.view_proj.as_ptr());
            gl.Uniform1f(rotloc, self.rot_angle);
        }
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
            fovy: 45.0,
            znear: 0.01,
            zfar: 100.0,
        }
    }

    pub fn down_cam(&mut self) {
        self.eye = (1.0, 0.5, 0.75).into();
        self.target = (-0.5, 0.0, 0.0).into();
        self.up = Vec3::unit_z();
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
