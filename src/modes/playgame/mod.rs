use super::resources::SharedResources;
use imgui::*;
use legion::prelude::*;
use nox_components::*;
use nox_planet::*;
use winit::event::VirtualKeyCode;
mod loadstate;
use crate::engine::uniforms::UniformBlock;
pub use loadstate::*;
mod chunks;
pub mod vox;
use vox::VoxBuffer;
mod render_passes;
use crate::messaging;
use crate::systems;
pub use render_passes::*;
use ultraviolet::Vec3;

#[derive(PartialEq, Copy, Clone)]
pub enum DesignMode {
    Lumberjack,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    OneStep,
    Running,
    FullSpeed,
    Design { mode: DesignMode },
}

pub struct PlayGame {
    pub planet: Option<Planet>,
    pub ecs: World,
    pub ecs_resources: Resources,

    // Internals
    rpass: Option<BlockRenderPass>,
    sunlight_pass: Option<SunlightPass>,
    vox_pass: Option<VoxRenderPass>,
    vox_instances: Vec<(u32, u32, u32)>,
    vox_changed: bool,
    lights_changed: bool,
    first_run: bool,

    // Game stuff that doesn't belong here
    rebuild_geometry: bool,
    chunks: chunks::Chunks,
    scheduler: Option<Schedule>,
    paused_scheduler: Option<Schedule>,
    run_state: RunState,

    time_accumulator: u128,
}

impl PlayGame {
    pub fn new() -> Self {
        *LOAD_STATE.lock() = LoadState::Idle;
        let universe = Universe::new();
        Self {
            planet: None,
            rpass: None,
            sunlight_pass: None,
            rebuild_geometry: true,
            ecs: universe.create_world(),
            ecs_resources: Resources::default(),
            chunks: chunks::Chunks::empty(),
            vox_pass: None,
            scheduler: None,
            paused_scheduler: None,
            run_state: RunState::Paused,
            vox_instances: Vec::new(),
            vox_changed: true,
            lights_changed: true,
            first_run: true,
            time_accumulator: 0,
        }
    }

    pub fn load(&mut self) {
        *LOAD_STATE.lock() = LoadState::Loading;
        std::thread::spawn(|| {
            let lg = nox_planet::load_game();
            *LOAD_STATE.lock() = LoadState::Loaded { game: lg };
        });
    }

    pub fn finish_loading(&mut self) {
        use crate::systems::REGION;
        println!("Finishing load");
        let locker = LOAD_STATE.lock().clone();
        match locker {
            LoadState::Loaded { game } => {
                self.planet = Some(game.planet);
                *REGION.write() = game.current_region;
                self.ecs = nox_components::deserialize_world(game.ecs_text);

                let mut loader_lock = crate::modes::loader::LOADER.write();
                self.rpass = loader_lock.rpass.take();
                self.sunlight_pass = loader_lock.sun_render.take();
                self.vox_pass = loader_lock.vpass.take();

                self.scheduler = Some(systems::build_scheduler());
                self.paused_scheduler = Some(systems::paused_scheduler());
            }
            _ => panic!("Not meant to go here."),
        }
        *LOAD_STATE.lock() = LoadState::Idle;
    }

    pub fn setup(&mut self) {
        // Moved to the loader
    }

    pub fn on_resize(&mut self) {
        println!("Resize called");
        if self.rpass.is_none() {
            return;
        }
        self.rpass.as_mut().unwrap().on_resize();
        //self.gbuffer_pass = Some(GBufferTestPass::new(&self.rpass.as_ref().unwrap().gbuffer));
    }

    pub fn tick(
        &mut self,
        _resources: &SharedResources,
        frame: &wgpu::SwapChainOutput,
        imgui: &imgui::Ui,
        depth_id: usize,
        keycode: Option<VirtualKeyCode>,
        frame_time: u128,
        mouse_world_pos: &(usize, usize, usize),
    ) -> super::ProgramMode {
        //println!("{:?}", mouse_world_pos);
        let camera_z = self.camera_control(&keycode, imgui);

        if self.rebuild_geometry {
            self.update_geometry();
        }

        messaging::reset();
        self.run_systems(frame_time);
        let rf = messaging::get_render_flags();
        if rf.lights_changed {
            self.lights_changed = true;
        }
        if rf.models_changed {
            self.vox_changed = true;
        }
        if rf.terrain_changed {
            self.rebuild_geometry = true;
        }

        let sun_pos = self.user_interface(frame_time, imgui, mouse_world_pos);
        self.render(camera_z, depth_id, frame, &sun_pos);
        super::ProgramMode::PlayGame
    }

    fn update_geometry(&mut self) {
        let pass = self.rpass.as_mut().unwrap();

        // Rebuild chunks that need it
        pass.vb.clear();
        self.chunks.rebuild_all();

        // Update the chunk frustrum system
        let query = <(Read<Position>, Read<CameraOptions>)>::query();
        for (pos, camopts) in query.iter(&self.ecs) {
            let size = crate::engine::get_window_size();
            pass.camera
                .update(&*pos, &*camopts, size.width, size.height);
            let camera_matrix = pass.camera.build_view_projection_matrix();
            self.chunks.on_camera_move(&camera_matrix, &*pos);
            pass.uniforms.update_buffer(&pass.uniform_buf);
        }

        // Mark clean
        self.rebuild_geometry = false;
    }

    fn user_interface(
        &mut self,
        frame_time: u128,
        imgui: &Ui,
        mouse_world_pos: &(usize, usize, usize),
    ) -> (Vec3, Vec3) {
        use crate::systems::{draw_main_menu, draw_tooltips, fps_display, lumberjack_display};
        let sun_pos = draw_main_menu(&self.ecs, &mut self.run_state, imgui);
        fps_display(imgui, frame_time);
        draw_tooltips(&self.ecs, mouse_world_pos, imgui);

        if let RunState::Design { mode } = self.run_state {
            match mode {
                DesignMode::Lumberjack => {
                    lumberjack_display(imgui, &self.ecs, mouse_world_pos);
                }
            }
        }

        sun_pos
    }

    fn camera_control(&mut self, keycode: &Option<VirtualKeyCode>, imgui: &Ui) -> usize {
        let mut result = 0;
        let pass = self.rpass.as_mut().unwrap();
        let query = <(Write<Position>, Write<CameraOptions>)>::query();
        let mut camera_changed = true;
        for (mut pos, mut camopts) in query.iter_mut(&mut self.ecs) {
            let cam = &mut pass.camera;
            if imgui.io().want_capture_keyboard {
                camera_changed = false;
            } else {
                if let Some(keycode) = keycode {
                    match keycode {
                        VirtualKeyCode::Space => {
                            self.run_state = RunState::Paused;
                            camera_changed = false;
                        }
                        VirtualKeyCode::Grave => {
                            self.run_state = RunState::OneStep;
                            camera_changed = false;
                        }
                        VirtualKeyCode::Key1 => {
                            self.run_state = RunState::Running;
                            camera_changed = false;
                        }
                        VirtualKeyCode::Key2 => {
                            self.run_state = RunState::FullSpeed;
                            camera_changed = false;
                        }
                        VirtualKeyCode::Slash => {
                            self.run_state = RunState::OneStep;
                            camera_changed = false;
                        }
                        VirtualKeyCode::Left => pos.x -= 1,
                        VirtualKeyCode::Right => pos.x += 1,
                        VirtualKeyCode::Up => pos.y -= 1,
                        VirtualKeyCode::Down => pos.y += 1,
                        VirtualKeyCode::Comma => {
                            pos.z += 1;
                        }
                        VirtualKeyCode::Period => {
                            pos.z -= 1;
                        }
                        VirtualKeyCode::Minus => camopts.zoom_level -= 1,
                        VirtualKeyCode::Add => camopts.zoom_level += 1,
                        VirtualKeyCode::Tab => match camopts.mode {
                            CameraMode::TopDown => camopts.mode = CameraMode::Front,
                            CameraMode::Front => camopts.mode = CameraMode::DiagonalNW,
                            CameraMode::DiagonalNW => camopts.mode = CameraMode::DiagonalNE,
                            CameraMode::DiagonalNE => camopts.mode = CameraMode::DiagonalSW,
                            CameraMode::DiagonalSW => camopts.mode = CameraMode::DiagonalSE,
                            CameraMode::DiagonalSE => camopts.mode = CameraMode::TopDown,
                        },
                        _ => camera_changed = false,
                    }
                } else {
                    camera_changed = false;
                }
            }
            if camera_changed | self.first_run {
                let size = crate::engine::get_window_size();
                cam.update(&*pos, &*camopts, size.width, size.height);
                pass.uniforms.update_view_proj(&pass.camera);
                //pass.uniforms.view_proj = self.sun_terrain_pass.as_ref().unwrap().uniforms.view_proj; // Comment out
                self.chunks.on_camera_move(&pass.uniforms.view_proj, &*pos);
                pass.uniforms.update_buffer(&pass.uniform_buf);
                self.vox_changed = true;
                self.lights_changed = true;
                self.first_run = false;
            }

            result = pos.z;
        }
        result
    }

    #[inline(always)]
    fn render(
        &mut self,
        camera_z: usize,
        depth_id: usize,
        frame: &wgpu::SwapChainOutput,
        sun_pos: &(Vec3, Vec3),
    ) {
        let pass = self.rpass.as_mut().unwrap();
        // Render terrain building the initial chunk models list
        pass.render(depth_id, frame, &mut self.chunks, camera_z as usize);

        // Build the voxel instance list
        let vox_pass = self.vox_pass.as_mut().unwrap();
        if self.vox_changed {
            vox::build_vox_instances2(
                &self.ecs,
                camera_z,
                &vox_pass.vox_models,
                &mut vox_pass.instance_buffer,
                &mut self.vox_instances,
                &self.chunks.frustrum,
                &self.chunks,
            );
            self.vox_changed = false;
        }

        vox_pass.render(
            depth_id,
            frame,
            &pass.gbuffer,
            &pass.uniform_bind_group,
            &self.vox_instances,
        );

        // Render z-buffer and g-buffer to 1st pass lighting
        let pass2 = self.sunlight_pass.as_mut().unwrap();
        pass2.render(
            frame,
            sun_pos,
            pass.camera.eye,
            &self.ecs,
            &pass.gbuffer,
            self.lights_changed,
        );
        self.lights_changed = false;
        pass.gbuffer.copy_mouse_buffer();
    }

    fn run_systems(&mut self, frame_time: u128) {
        match self.run_state {
            RunState::FullSpeed => {
                self.scheduler
                    .as_mut()
                    .unwrap()
                    .execute(&mut self.ecs, &mut self.ecs_resources);
            }
            RunState::Running | RunState::OneStep => {
                self.time_accumulator += frame_time;
                if self.time_accumulator > 33 {
                    self.time_accumulator = 0;
                    self.scheduler
                        .as_mut()
                        .unwrap()
                        .execute(&mut self.ecs, &mut self.ecs_resources);
                }
            }
            _ => {
                self.paused_scheduler
                    .as_mut()
                    .unwrap()
                    .execute(&mut self.ecs, &mut self.ecs_resources);
            }
        }
    }

    pub fn get_mouse_buffer(&self) -> Option<&wgpu::Buffer> {
        if let Some(pass) = &self.rpass {
            return Some(&pass.gbuffer.mouse_buffer);
        } else {
            return None;
        }
    }
}
