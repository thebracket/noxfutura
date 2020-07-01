use super::resources::SharedResources;
use crate::components::*;
use crate::planet::*;
use imgui::*;
use legion::prelude::*;
use winit::event::VirtualKeyCode;
mod loadstate;
use crate::engine::uniforms::UniformBlock;
pub use loadstate::*;
mod chunks;
pub mod vox;
use vox::VoxBuffer;
mod render_passes;
pub use render_passes::*;
mod systems;
use ultraviolet::Vec3;
mod shared_state;
pub use shared_state::*;

#[derive(PartialEq, Copy, Clone)]
enum RunState {
    Paused,
    OneStep,
    Running,
}

pub struct PlayGame {
    pub planet: Option<Planet>,
    pub ecs: World,
    pub ecs_resources: Resources,

    // Internals
    rpass: Option<BlockRenderPass>,
    sunlight_pass: Option<SunlightPass>,
    vox_pass: Option<VoxRenderPass>,
    chunk_models: Vec<ChunkModel>,

    // Game stuff that doesn't belong here
    rebuild_geometry: bool,
    chunks: chunks::Chunks,
    scheduler: Option<Schedule>,
    run_state: RunState,
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
            chunk_models: Vec::new(),
            scheduler: None,
            run_state: RunState::Paused,
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
                *REGION.write() = game.current_region;
                self.ecs = crate::components::deserialize_world(game.ecs_text);

                let mut loader_lock = crate::modes::loader::LOADER.write();
                self.rpass = loader_lock.rpass.take();
                self.sunlight_pass = loader_lock.sun_render.take();
                self.vox_pass = loader_lock.vpass.take();

                self.scheduler = Some(systems::build_scheduler());
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
    ) -> super::ProgramMode {
        let camera_z = self.camera_control(&keycode);

        if self.rebuild_geometry {
            self.update_geometry();
        }

        self.run_systems();

        let sun_pos = self.user_interface(frame_time, imgui);
        self.render(camera_z, depth_id, frame, sun_pos);
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

    fn user_interface(&mut self, frame_time: u128, imgui: &Ui) -> Vec3 {
        let mut sun_pos = Vec3::zero();

        if let Some(menu_bar) = imgui.begin_main_menu_bar() {
            if let Some(menu) = imgui.begin_menu(im_str!("Test"), true) {
                MenuItem::new(im_str!("Test Item")).build(imgui);
                menu.end(imgui);
            }
            menu_bar.end(imgui);
        }

        //imgui.spacing();

        /*let title = format!(
            "Playing. Frame time: {} ms. FPS: {}.",
            frame_time,
            imgui.io().framerate
        );
        let title_tmp = ImString::new(title);
        let window = imgui::Window::new(&title_tmp);
        window
            .collapsed(true, Condition::Always)
            .size([300.0, 100.0], Condition::Always)
            .build(imgui, || {});*/

        // Obtain info to display
        let mut hud_time = String::new();
        let query = <Read<Calendar>>::query();
        for c in query.iter(&self.ecs) {
            hud_time = c.get_date_time();
            sun_pos = c.calculate_sun_moon();
        }
        let hud_time_im = ImString::new(hud_time);

        sun_pos
    }

    fn camera_control(&mut self, keycode: &Option<VirtualKeyCode>) -> usize {
        let mut result = 0;
        let pass = self.rpass.as_mut().unwrap();
        let query = <(Write<Position>, Write<CameraOptions>)>::query();
        let mut camera_changed = true;
        for (mut pos, mut camopts) in query.iter_mut(&mut self.ecs) {
            let cam = &mut pass.camera;
            if let Some(keycode) = keycode {
                match keycode {
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
            }
            if camera_changed {
                let size = crate::engine::get_window_size();
                cam.update(&*pos, &*camopts, size.width, size.height);
                pass.uniforms.update_view_proj(&pass.camera);
                //pass.uniforms.view_proj = self.sun_terrain_pass.as_ref().unwrap().uniforms.view_proj; // Comment out
                self.chunks.on_camera_move(&pass.uniforms.view_proj, &*pos);
                pass.uniforms.update_buffer(&pass.uniform_buf);
            }

            result = pos.z;
        }
        result
    }

    fn render(
        &mut self,
        camera_z: usize,
        depth_id: usize,
        frame: &wgpu::SwapChainOutput,
        sun_pos: Vec3,
    ) {
        let pass = self.rpass.as_mut().unwrap();
        if pass.vb.len() > 0 {
            pass.vb.update_buffer();
        }

        // Render terrain building the initial chunk models list
        self.chunk_models.clear();
        pass.render(
            depth_id,
            frame,
            &mut self.chunks,
            camera_z as usize,
            &mut self.chunk_models,
        );

        // Build the voxel instance list
        let vox_pass = self.vox_pass.as_mut().unwrap();
        let vox_instances = vox::build_vox_instances(
            &self.ecs,
            camera_z,
            &vox_pass.vox_models,
            &mut vox_pass.instance_buffer,
            &mut self.chunk_models,
        );

        vox_pass.render(
            depth_id,
            frame,
            &pass.gbuffer,
            &pass.uniform_bind_group,
            &vox_instances,
        );

        // Render z-buffer and g-buffer to 1st pass lighting
        let pass2 = self.sunlight_pass.as_mut().unwrap();
        pass2.render(
            frame,
            sun_pos.into(),
            pass.camera.eye,
            &self.ecs,
            &pass.gbuffer,
        );
    }

    fn run_systems(&mut self) {
        self.scheduler
            .as_mut()
            .unwrap()
            .execute(&mut self.ecs, &mut self.ecs_resources);
    }
}
