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
use render_passes::*;

pub struct PlayGame {
    pub planet: Option<Planet>,
    pub current_region: Option<Region>,
    pub ecs: legion::prelude::World,

    // Internals
    rpass: Option<BlockRenderPass>,
    gbuffer_pass: Option<GBufferTestPass>,
    vox_pass: Option<VoxRenderPass>,
    chunk_models: Vec<ChunkModel>,

    // Game stuff that doesn't belong here
    rebuild_geometry: bool,
    chunks: chunks::Chunks,
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
            rebuild_geometry: true,
            ecs: universe.create_world(),
            chunks: chunks::Chunks::empty(),
            vox_pass: None,
            chunk_models: Vec::new(),
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
        crate::raws::load_raws();
        self.rpass = Some(BlockRenderPass::new(context));
        self.gbuffer_pass = Some(GBufferTestPass::new(
            context,
            &self.rpass.as_ref().unwrap().gbuffer,
        ));
        self.vox_pass = Some(VoxRenderPass::new(
            context,
            &self.rpass.as_ref().unwrap().uniform_bind_group_layout,
        ));
    }

    pub fn on_resize(&mut self, context: &mut crate::engine::Context) {
        self.rpass.as_mut().unwrap().on_resize(context);
        self.gbuffer_pass = Some(GBufferTestPass::new(
            context,
            &self.rpass.as_ref().unwrap().gbuffer,
        ));
    }

    pub fn tick(
        &mut self,
        _resources: &SharedResources,
        frame: &wgpu::SwapChainOutput,
        context: &mut crate::engine::Context,
        imgui: &imgui::Ui,
        depth_id: usize,
        keycode: Option<VirtualKeyCode>,
        frame_time: u128,
    ) -> super::ProgramMode {
        //super::helpers::render_menu_background(context, frame, resources);

        let camera_z = self.camera_control(&keycode, context);
        let pass = self.rpass.as_mut().unwrap();

        if self.rebuild_geometry {
            println!("Rebuilding geometry");
            if let Some(region) = &self.current_region {
                // Rebuild chunks that need it
                pass.vb.clear();
                self.chunks.rebuild_all(region, context);
            }

            // Update the chunk frustrum system
            let query = <(Read<Position>, Read<CameraOptions>)>::query();
            for (pos, camopts) in query.iter(&self.ecs) {
                pass.camera
                    .update(&*pos, &*camopts, context.size.width, context.size.height);
                let camera_matrix = pass.camera.build_view_projection_matrix();
                self.chunks.on_camera_move(&camera_matrix, &*pos);
                pass.uniforms.update_buffer(context, &pass.uniform_buf);
            }

            // Mark clean
            self.rebuild_geometry = false;
        }

        if pass.vb.len() > 0 {
            pass.vb.update_buffer(context);
        }

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
            menu_bar.end(imgui);
        }

        self.chunk_models.clear();
        pass.render(
            context,
            depth_id,
            frame,
            &mut self.chunks,
            camera_z as usize,
            &mut self.chunk_models,
        );
        self.vox_pass.as_mut().unwrap().render(
            context,
            depth_id,
            frame,
            &pass.gbuffer,
            &pass.uniform_bind_group,
            camera_z as usize,
            &self.ecs,
            &self.chunk_models,
        );

        let pass2 = self.gbuffer_pass.as_mut().unwrap();
        pass2.render(context, frame);

        super::ProgramMode::PlayGame
    }

    fn camera_control(
        &mut self,
        keycode: &Option<VirtualKeyCode>,
        context: &crate::engine::Context,
    ) -> usize {
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
                cam.update(&*pos, &*camopts, context.size.width, context.size.height);
                pass.uniforms.update_view_proj(&pass.camera);
                self.chunks.on_camera_move(&pass.uniforms.view_proj, &*pos);
                pass.uniforms.update_buffer(context, &pass.uniform_buf);
            }

            result = pos.z;
        }
        result
    }
}
