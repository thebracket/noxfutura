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
mod render;

pub struct PlayGame {
    pub planet: Option<Planet>,
    pub current_region: Option<Region>,
    pub ecs: legion::prelude::World,

    // Internals
    rpass : Option<render::BlockRenderPass>,

    // Game stuff that doesn't belong here
    rebuild_geometry: bool,
    counter: usize
}

impl PlayGame {
    pub fn new() -> Self {
        *LOAD_STATE.lock() = LoadState::Idle;
        let universe = legion::prelude::Universe::new();
        Self {
            planet: None,
            current_region: None,
            rpass: None,
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
        self.rpass = Some(render::BlockRenderPass::new(context));
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
        let pass = self.rpass.as_mut().unwrap();

        if self.rebuild_geometry {
            if let Some(region) = &self.current_region {
                pass.vb.clear();
                let mut chunks = crate::planet::chunks::Chunks::empty();
                chunks.rebuild_all(region);
                for p in chunks.all_geometry().iter() {
                    match *p {
                        Primitive::Cube { x, y, z, w, h, d } => {
                            crate::utils::add_cube_geometry(
                                &mut pass.vb,
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
            let cam = &mut pass.camera;
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

        pass.uniforms.update_view_proj(&pass.camera, self.counter);
        pass.uniforms.update_buffer(context, &pass.uniform_buf);
        pass.vb.update_buffer(context);
        self.counter += 1;

        let window = imgui::Window::new(im_str!("Playing"));
        window
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(imgui, || {
                imgui.text(im_str!("Test"));
            });

        if pass.vb.len() > 0 {
            pass.render(context, depth_id, frame);
        }

        super::ProgramMode::PlayGame
    }
}
