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

pub struct PlayGame {
    pub planet: Option<Planet>,
    pub current_region: Option<Region>,
    pub ecs: legion::prelude::World,
    pub ecs_resources: legion::prelude::Resources,

    // Internals
    rpass: Option<BlockRenderPass>,
    gbuffer_pass: Option<GBufferTestPass>,
    vox_pass: Option<VoxRenderPass>,
    sun_terrain_pass: Option<SunDepthTerrainPass>,
    chunk_models: Vec<ChunkModel>,

    // Game stuff that doesn't belong here
    rebuild_geometry: bool,
    chunks: chunks::Chunks,
    scheduler: Option<Schedule>
}

impl PlayGame {
    pub fn new() -> Self {
        *LOAD_STATE.lock() = LoadState::Idle;
        let universe = legion::prelude::Universe::new();
        Self {
            planet: None,
            current_region: None,
            rpass: None,
            gbuffer_pass: None,
            sun_terrain_pass: None,
            rebuild_geometry: true,
            ecs: universe.create_world(),
            ecs_resources: legion::prelude::Resources::default(),
            chunks: chunks::Chunks::empty(),
            vox_pass: None,
            chunk_models: Vec::new(),
            scheduler: None
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

                let mut loader_lock = crate::modes::loader::LOADER.write();
                self.rpass = loader_lock.rpass.take();
                self.gbuffer_pass = loader_lock.gpass.take();
                self.vox_pass = loader_lock.vpass.take();
                self.sun_terrain_pass = loader_lock.sun_terrain.take();

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
    ) -> super::ProgramMode 
    {
        let camera_z = self.camera_control(&keycode);

        if self.rebuild_geometry {
            self.update_geometry();
        }

        self.run_systems();

        let sun_position = self.user_interface(frame_time, imgui);
        self.render(camera_z, depth_id, frame, sun_position);
        super::ProgramMode::PlayGame
    }

    fn update_geometry(&mut self) {
        let pass = self.rpass.as_mut().unwrap();
        if let Some(region) = &self.current_region {
            // Rebuild chunks that need it
            pass.vb.clear();
            self.chunks.rebuild_all(region);
        }

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

    fn user_interface(&mut self, frame_time: u128, imgui: &Ui) -> (f32, f32, f32) {
        let mut result = (0.0, 0.0, 0.0);
        let title = format!(
            "Playing. Frame time: {} ms. FPS: {}.",
            frame_time,
            imgui.io().framerate
        );
        let title_tmp = ImString::new(title);
        let window = imgui::Window::new(&title_tmp);
        window
            .collapsed(true, Condition::FirstUseEver)
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(imgui, || {});

        // Obtain info to display
        let mut hud_time = String::new();
        let query = <Read<Calendar>>::query();
        for c in query.iter(&self.ecs) {
            hud_time = c.get_date_time();
            result = c.calculate_sun_moon();
        }
        let hud_time_im = ImString::new(hud_time);

        // Show the menu
        if let Some(menu_bar) = imgui.begin_main_menu_bar() {
            if let Some(menu) = imgui.begin_menu(im_str!("Nox Futura"), true) {
                menu.end(imgui);
            }
            if let Some(menu) = imgui.begin_menu(im_str!("Design"), true) {
                menu.end(imgui);
            }
            if let Some(menu) = imgui.begin_menu(im_str!("Units"), true) {
                menu.end(imgui);
            }
            let status_size = imgui.calc_text_size(&hud_time_im, false, 0.0);
            imgui.same_line(imgui.window_content_region_width() - (status_size[0] + 10.0));
            imgui.text(hud_time_im);
            menu_bar.end(imgui);
        }

        result
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
                self.chunks.on_camera_move(&pass.uniforms.view_proj, &*pos);
                pass.uniforms.update_buffer(&pass.uniform_buf);
            }

            result = pos.z;
        }
        result
    }

    fn render(&mut self, camera_z: usize, depth_id: usize, frame: &wgpu::SwapChainOutput, sun_pos: (f32, f32, f32)) {
        let pass = self.rpass.as_mut().unwrap();
        if pass.vb.len() > 0 {
            pass.vb.update_buffer();
        }

        self.chunk_models.clear();
        self.sun_terrain_pass.as_mut().unwrap().render(
            &mut self.chunks,
            &mut self.chunk_models,
            (255.0, 256.0, 128.0),
            &pass.uniform_bind_group
        );

        self.chunk_models.clear();
        pass.render(
            depth_id,
            frame,
            &mut self.chunks,
            camera_z as usize,
            &mut self.chunk_models,
        );
        self.vox_pass.as_mut().unwrap().render(
            depth_id,
            frame,
            &pass.gbuffer,
            &pass.uniform_bind_group,
            camera_z as usize,
            &self.ecs,
            &self.chunk_models,
        );

        let pass2 = self.gbuffer_pass.as_mut().unwrap();
        pass2.render(frame);
    }

    fn run_systems(&mut self) {
        self.scheduler.as_mut().unwrap().execute(&mut self.ecs, &mut self.ecs_resources);
    }
}
