use crate::opengl::*;
use crate::prelude::*;
use std::ffi::CString;
use ultraviolet::{
    mat::Mat4,
    vec::Vec3,
};

pub struct PlayGame {
    planet : Planet,
    current_region : Region,
    universe : Universe,
    ecs : World,
    mapchunks: Chunks,
    temp_shader: Shader,
    camera : Camera,
    uniforms: Uniforms,
    world_buffer: VertexArray
}

impl PlayGame {
    pub fn new(save : SavedGame, gl: &Gl, ctx: &EngineContext) -> Self {
        let universe = Universe::new();
        let ecs = universe.create_world();
        let mut mapchunks = Chunks::empty();
        mapchunks.rebuild_all(&save.current_region);
        let geometry = mapchunks.all_geometry();
        let temp_shader = Shader::new(
            gl,
            include_str!("../../resources/shaders/planetgen.vert"),
            include_str!("../../resources/shaders/planetgen.frag")
        );
        let mut world_buffer = VertexArray::float_builder(gl, &[VertexArrayEntry{index: 0, size: 3}, VertexArrayEntry{index: 1, size: 4}], 20000);
        all_region_primitives(geometry, &mut world_buffer);
        world_buffer.upload_buffers(gl);
        Self{
            planet : save.planet,
            current_region : save.current_region,
            universe,
            ecs,
            mapchunks,
            temp_shader,
            camera: Camera::new(ctx.screen_size.x as u32, ctx.screen_size.y as u32),
            uniforms: Uniforms::new(),
            world_buffer
        }
    }

    pub fn tick(
        &mut self,
        gl: &Gl,
        _ui: &imgui::Ui,
        ctx: &EngineContext
    ) -> super::ProgramMode 
    {
        self.uniforms.update_view_proj_flat(&mut self.camera, ctx);
        self.uniforms.update_uniforms(gl, &self.temp_shader);
        self.world_buffer.draw_elements_no_texture(gl, &self.temp_shader);

        super::ProgramMode::PlayGame
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

    fn update_view_proj(&mut self, camera: &mut Camera, ctx: &EngineContext) {
        self.view_proj = camera.build_view_projection_matrix(ctx);
        self.rot_angle += 0.001;
    }

    fn update_view_proj_flat(&mut self, camera: &mut Camera, ctx: &EngineContext) {
        camera.down_cam();
        self.rot_angle = 0.0;
        self.view_proj = camera.build_view_projection_matrix(ctx);
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
            eye: (0.0, 256.0, 0.0).into(),
            target: (128.0, 0.0, 128.0).into(),
            up: Vec3::unit_y(),
            aspect: width as f32 / height as f32,
            fovy: 1.5708, // 90 Degrees in radians
            znear: 0.01,
            zfar: 1000.0,
        }
    }

    pub fn down_cam(&mut self) {
        //self.eye = (1.0, 0.5, 0.75).into();
        //self.target = (-0.5, 0.0, 0.0).into();
        //self.up = Vec3::unit_z();
    }

    pub fn build_view_projection_matrix(&mut self, ctx: &EngineContext) -> Mat4 {
        self.aspect = ctx.screen_size.x as f32 / ctx.screen_size.y as f32;
        let view = Mat4::look_at(self.eye, self.target, self.up);
        let proj =
            ultraviolet::projection::perspective_gl(self.fovy, self.aspect, self.znear, self.zfar);
        proj * view
    }
}
