use super::{loadstate::*, systems::REGION, Chunks, TerrainPass};
use crate::components::{CameraOptions, Position};
use crate::{GameMode, NoxMode, SharedResources};
use bengine::*;
use legion::*;

pub struct PlayTheGame {
    ready: bool,
    started_loader: bool,
    planet: Option<crate::planet::Planet>,
    ecs: World,
    ecs_resources: Resources,
    chunks: Chunks,

    terrain_pass: Option<TerrainPass>,

    regular_schedule: Schedule,
    paused_schedule: Schedule,
}

impl PlayTheGame {
    pub fn new() -> Self {
        LOAD_STATE.write().state = LoadState::Idle;
        Self {
            ready: false,
            started_loader: false,
            planet: None,
            ecs: World::default(),
            ecs_resources: Resources::default(),
            chunks: Chunks::empty(),
            terrain_pass: None,
            regular_schedule: super::systems::build_scheduler(),
            paused_schedule: super::systems::paused_scheduler(),
        }
    }

    fn load(&mut self) {
        if !self.started_loader {
            {
                println!("Starting loader");
                self.started_loader = true;
            }
            std::thread::spawn(|| {
                LOAD_STATE.write().state = LoadState::Loading;
                let lg = crate::planet::load_game();
                println!("Loader process complete");
                LOAD_STATE.write().state = LoadState::Loaded { game: lg };
                println!("Unlocked loader");
            });
        } else {
            let locker = LOAD_STATE.read().state.clone();
            match locker {
                LoadState::Loading => {
                    // Do nothing while the loader spins
                }
                LoadState::Loaded { game } => {
                    LOAD_STATE.write().state = LoadState::Idle;
                    self.planet = Some(game.planet);
                    *REGION.write() = game.current_region;
                    self.ecs = crate::components::deserialize_world(game.ecs_text);

                    let mut loader_lock = crate::modes::LOADER.write();
                    self.terrain_pass = loader_lock.terrain_pass.take();

                    self.chunks.rebuild_all();
                    let mut query = <(&Position, &CameraOptions)>::query();
                    for (pos, camopts) in query.iter(&self.ecs) {
                        let size = RENDER_CONTEXT.read().as_ref().unwrap().size;
                        let pass = self.terrain_pass.as_mut().unwrap();
                        pass.camera
                            .update(&*pos, &*camopts, size.width, size.height);
                        let camera_matrix = pass.camera.build_view_projection_matrix();
                        self.chunks.on_camera_move(&camera_matrix, &*pos);
                        pass.uniforms.update_view_proj(&pass.camera);
                    }

                    self.ecs_resources.insert(super::GameStateResource::new());
                    /*
                    self.planet = Some(game.planet);
                    *REGION.write() = game.current_region;
                    self.ecs = nox_components::deserialize_world(game.ecs_text);

                    let mut loader_lock = crate::modes::loader::LOADER.write();
                    self.rpass = loader_lock.rpass.take();
                    self.sunlight_pass = loader_lock.sun_render.take();
                    self.vox_pass = loader_lock.vpass.take();
                    self.cursor_pass = loader_lock.cpass.take();

                    self.scheduler = Some(systems::build_scheduler());
                    self.paused_scheduler = Some(systems::paused_scheduler());
                    */
                    println!("Finished loading");
                    self.ready = true;
                }

                LoadState::Idle => {}
            }
        }
    }

    #[inline(always)]
    fn update_camera(&mut self) {
        let mut shared_state = self.ecs_resources.get_mut::<super::GameStateResource>();
        if shared_state.as_mut().unwrap().camera_changed {
            shared_state.as_mut().unwrap().camera_changed = false;
            let mut camera_changed = <(&Position, &CameraOptions)>::query();
            for (pos, camopts) in camera_changed.iter(&self.ecs) {
                let size = RENDER_CONTEXT.read().as_ref().unwrap().size;
                let pass = self.terrain_pass.as_mut().unwrap();
                pass.camera
                    .update(&*pos, &*camopts, size.width, size.height);
                let camera_matrix = pass.camera.build_view_projection_matrix();
                self.chunks.on_camera_move(&camera_matrix, &*pos);
                pass.uniforms.update_view_proj(&pass.camera);
            }
        }
    }
}

impl NoxMode for PlayTheGame {
    fn tick(&mut self, core: &mut Core, shared: &SharedResources) -> GameMode {
        use gui::*;
        let result = GameMode::PlayGame;

        if !self.ready {
            self.load();
            shared.quad_render.render(shared.background_image, core);
            let window = gui::Window::new(im_str!("Loading saved game - please wait"));
            window
                .size([300.0, 100.0], Condition::FirstUseEver)
                .collapsed(true, Condition::FirstUseEver)
                .build(core.imgui, || {});
        } else {
            // Phase 1: Execute the ECS
            {
                let mut shared_state = self.ecs_resources.get_mut::<super::GameStateResource>();
                shared_state.as_mut().unwrap().frame_update(core.keycode);
            }
            self.regular_schedule
                .execute(&mut self.ecs, &mut self.ecs_resources);

            // Phase 2: Actually render stuff
            self.update_camera();

            let mut query = <(&Position, &CameraOptions)>::query();
            let mut camera_z = 0;
            for (pos, _camopts) in query.iter(&self.ecs) {
                camera_z = pos.as_point3().z;
            }
            self.terrain_pass
                .as_ref()
                .unwrap()
                .render(core, &self.chunks, camera_z);
        }

        result
    }
}
