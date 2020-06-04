use super::resources::SharedResources;
use crate::engine::*;
use crate::planet::*;
use imgui::*;
use legion::prelude::*;
use crate::components::*;
use winit::event::VirtualKeyCode;
mod loadstate;
pub use loadstate::*;
mod uniforms;
use uniforms::*;
mod camera;
use camera::*;
use crate::engine::uniforms::UniformBlock;

pub struct PlayGame {
    pub planet: Option<Planet>,
    pub current_region: Option<Region>,
    pub ecs: legion::prelude::World,

    // Internals
    planet_shader: usize,
    planet_pipeline: Option<wgpu::RenderPipeline>,
    uniform_bind_group: Option<wgpu::BindGroup>,
    uniforms: Option<Uniforms>,
    camera: Option<Camera>,
    uniform_buffer: Option<wgpu::Buffer>,
    vb: VertexBuffer<f32>,
    rebuild_geometry: bool,

    // Game stuff that doesn't belong here
    counter: usize
}

impl PlayGame {
    pub fn new() -> Self {
        *LOAD_STATE.lock() = LoadState::Idle;
        let universe = legion::prelude::Universe::new();
        Self {
            planet: None,
            current_region: None,
            planet_shader: 0,
            planet_pipeline: None,
            uniform_bind_group: None,
            uniforms: None,
            camera: None,
            uniform_buffer: None,
            vb: VertexBuffer::new(&[3, 4, 3]),
            rebuild_geometry: true,
            ecs: universe.create_world(),
            counter: 0
        }
    }

    pub fn load(&mut self) {
        *LOAD_STATE.lock() = LoadState::Loading;
        std::thread::spawn(|| {
            let lg = crate::planet::load_game();
            *LOAD_STATE.lock() = LoadState::Loaded { game: lg };
        });
    }

    pub fn finish_loading(&mut self) {
        println!("Finishing load");
        let locker = LOAD_STATE.lock().clone();
        match locker {
            LoadState::Loaded { game } => {
                self.planet = Some(game.planet);
                self.current_region = Some(game.current_region);
                self.ecs = crate::components::deserialize_world(game.ecs_text);
            }
            _ => panic!("Not meant to go here."),
        }
        *LOAD_STATE.lock() = LoadState::Idle;
    }

    pub fn setup(&mut self, context: &mut crate::engine::Context) {
        self.vb.clear();
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
            .update_view_proj(self.camera.as_ref().unwrap(), self.counter);
        self.counter += 1;
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
            .vertex_state(wgpu::IndexFormat::Uint16, &[self.vb.descriptor()])
            .build(&context.device);
        self.planet_pipeline = Some(render_pipeline);
    }

    pub fn tick(
        &mut self,
        resources: &SharedResources,
        frame: &wgpu::SwapChainOutput,
        context: &mut crate::engine::Context,
        imgui: &imgui::Ui,
        depth_id: usize,
        keycode: Option<VirtualKeyCode>
    ) -> super::ProgramMode {
        super::helpers::render_menu_background(context, frame, resources);

        if self.rebuild_geometry {
            if let Some(region) = &self.current_region {
                self.vb.clear();
                let mut chunks = crate::planet::chunks::Chunks::empty();
                chunks.rebuild_all(region);
                for p in chunks.all_geometry().iter() {
                    match *p {
                        Primitive::Cube { x, y, z, w, h, d } => {
                            crate::utils::add_cube_geometry(
                                &mut self.vb,
                                x as f32,
                                y as f32,
                                z as f32,
                                w as f32,
                                h as f32,
                                d as f32,
                            );
                        }
                    }
                }
            }
            self.rebuild_geometry = false;
        }

        let query = <(Write<Position>, Write<CameraOptions>)>::query();
        for (mut pos, mut camopts) in query.iter_mut(&mut self.ecs) {
            let cam = self.camera.as_mut().unwrap();
            if let Some(keycode) = keycode {
                match keycode {
                    VirtualKeyCode::Left => pos.x -= 1,
                    VirtualKeyCode::Right => pos.x += 1,
                    VirtualKeyCode::Up => pos.y -= 1,
                    VirtualKeyCode::Down => pos.y += 1,
                    VirtualKeyCode::Comma => pos.z += 1,
                    VirtualKeyCode::Period => pos.z -= 1,
                    VirtualKeyCode::Minus => camopts.zoom_level -=1,
                    VirtualKeyCode::Add => camopts.zoom_level +=1,
                    VirtualKeyCode::Tab => {
                        match camopts.mode {
                            CameraMode::TopDown => camopts.mode = CameraMode::Front,
                            CameraMode::Front => camopts.mode = CameraMode::DiagonalNW,
                            CameraMode::DiagonalNW => camopts.mode = CameraMode::DiagonalNE,
                            CameraMode::DiagonalNE => camopts.mode = CameraMode::DiagonalSW,
                            CameraMode::DiagonalSW => camopts.mode = CameraMode::DiagonalSE,
                            CameraMode::DiagonalSE => camopts.mode = CameraMode::TopDown,
                        }
                    }
                    _ => {}
                }
            }
            cam.update(&*pos, &*camopts);
        }

        self.uniforms
            .as_mut()
            .unwrap()
            .update_view_proj(&self.camera.as_ref().unwrap(), self.counter);
        self.uniforms
            .as_ref()
            .unwrap()
            .update_buffer(context, &self.uniform_buffer.as_ref().unwrap());
        self.vb.update_buffer(context);
        self.counter += 1;

        let window = imgui::Window::new(im_str!("Playing"));
        window
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(imgui, || {
                imgui.text(im_str!("Test"));
            });

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
                rpass.set_vertex_buffer(0, &self.vb.buffer.as_ref().unwrap(), 0, 0);
                rpass.draw(0..self.vb.len(), 0..1);
                //println!("{}", self.vb.len());
                //rpass.draw(0..1, 0..1);
            }
            context.queue.submit(&[encoder.finish()]);
        }

        super::ProgramMode::PlayGame
    }
}
