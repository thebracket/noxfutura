use super::{
    loadstate::*, systems::REGION, Chunks, CursorPass, DesignMode, GBuffer, GrassPass,
    LightingPass, ModelsPass, RunState, TerrainPass, VoxPass,
};
use crate::{GameMode, NoxMode, SharedResources};
use bengine::*;
use legion::*;
use nox_components::{CameraOptions, Position};
use nox_planet::{LumberMap, MiningMap};

pub struct PlayTheGame {
    ready: bool,
    started_loader: bool,
    planet: Option<nox_planet::Planet>,
    ecs: World,
    ecs_resources: Resources,
    chunks: Chunks,

    terrain_pass: Option<TerrainPass>,
    model_pass: Option<ModelsPass>,
    grass_pass: Option<GrassPass>,
    vox_pass: Option<VoxPass>,
    lighting_pass: Option<LightingPass>,
    cursor_pass: Option<CursorPass>,

    palette: Option<Palette>,
    gbuffer: Option<GBuffer>,

    regular_schedule: Schedule,
    paused_schedule: Schedule,

    frame_time_accumulator: f32,
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
            model_pass: None,
            grass_pass: None,
            vox_pass: None,
            lighting_pass: None,
            cursor_pass: None,
            palette: None,
            gbuffer: None,
            regular_schedule: super::systems::build_scheduler(),
            paused_schedule: super::systems::paused_scheduler(),
            frame_time_accumulator: 0.0,
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
                let lg = nox_planet::load_game();
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
                    self.ecs = nox_components::deserialize_world(game.ecs_text);

                    let mut loader_lock = crate::modes::LOADER.write();
                    self.gbuffer = loader_lock.g_buffer.take();
                    self.terrain_pass = loader_lock.terrain_pass.take();
                    self.model_pass = loader_lock.model_pass.take();
                    self.grass_pass = loader_lock.grass_pass.take();
                    self.vox_pass = loader_lock.vox_pass.take();
                    self.palette = loader_lock.palette.take();
                    self.lighting_pass = loader_lock.lighting_pass.take();
                    self.cursor_pass = loader_lock.cursor_pass.take();

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
                    self.ecs_resources.insert(RunState::Paused);
                    self.ecs_resources.insert(MiningMap::new());
                    self.ecs_resources.insert(LumberMap::new());
                    println!("Finished loading");
                    self.ready = true;
                }

                LoadState::Idle => {}
            }
        }
    }

    #[inline(always)]
    fn update_camera(&mut self) {
        if let Some(mut shared_state) = self.ecs_resources.get_mut::<super::GameStateResource>() {
            if shared_state.camera_changed {
                shared_state.camera_changed = false;
                let mut camera_changed = <(&Position, &CameraOptions)>::query();
                for (pos, camopts) in camera_changed.iter(&self.ecs) {
                    let size = RENDER_CONTEXT.read().as_ref().unwrap().size;
                    let pass = self.terrain_pass.as_mut().unwrap();
                    pass.camera
                        .update(&*pos, &*camopts, size.width, size.height);
                    let camera_matrix = pass.camera.build_view_projection_matrix();
                    self.chunks.on_camera_move(&camera_matrix, &*pos);
                    self.model_pass.as_mut().unwrap().models_changed = true;
                    self.grass_pass.as_mut().unwrap().models_changed = true;
                    self.vox_pass.as_mut().unwrap().models_changed = true;
                    pass.uniforms.update_view_proj(&pass.camera);
                }
            }
            if shared_state.vox_moved {
                shared_state.vox_moved = false;
                self.vox_pass.as_mut().unwrap().models_changed = true;
            }
            if shared_state.models_moved {
                shared_state.models_moved = false;
                self.model_pass.as_mut().unwrap().models_changed = true;
            }
            if shared_state.lights_changed {
                shared_state.lights_changed = false;
                self.lighting_pass.as_mut().unwrap().lighting_changed = true;
            }
            if !shared_state.dirty_tiles.is_empty() {
                self.chunks.mark_dirty(&shared_state.dirty_tiles);
                //TODO: This could be parallel
                {
                    let mut rlock = REGION.write();
                    rlock.reset_all_flags();
                    nox_planet::rebuild_flags(&mut rlock);
                }

                self.chunks.rebuild_all();
                shared_state.dirty_tiles.clear();
                self.ecs_resources
                    .get_mut::<MiningMap>()
                    .as_mut()
                    .unwrap()
                    .is_dirty = true;
            }
        }
    }
}

impl NoxMode for PlayTheGame {
    fn get_mouse_buffer(&self) -> Option<&gpu::Buffer> {
        if let Some(gb) = self.gbuffer.as_ref() {
            Some(&gb.mouse_buffer)
        } else {
            None
        }
    }

    fn on_resize(&mut self) {
        if self.gbuffer.is_some() {
            self.gbuffer = Some(GBuffer::new());
            self.lighting_pass
                .as_mut()
                .unwrap()
                .on_resize(self.gbuffer.as_ref().unwrap());
        }
    }

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

            let run_state = self.ecs_resources.get::<RunState>().unwrap().clone();
            match run_state {
                RunState::Paused | RunState::Design { .. } => self
                    .paused_schedule
                    .execute(&mut self.ecs, &mut self.ecs_resources),
                RunState::SlowMo => {
                    self.frame_time_accumulator += core.frame_time;
                    if self.frame_time_accumulator > 0.3 {
                        self.frame_time_accumulator = 0.0;
                        self.regular_schedule
                            .execute(&mut self.ecs, &mut self.ecs_resources);
                    } else {
                        self.paused_schedule
                            .execute(&mut self.ecs, &mut self.ecs_resources);
                    }
                }
                RunState::Running => {
                    self.frame_time_accumulator += core.frame_time;
                    if self.frame_time_accumulator > 0.1 {
                        self.frame_time_accumulator = 0.0;
                        self.regular_schedule
                            .execute(&mut self.ecs, &mut self.ecs_resources);
                    } else {
                        self.paused_schedule
                            .execute(&mut self.ecs, &mut self.ecs_resources);
                    }
                }
                RunState::FullSpeed => {
                    self.regular_schedule
                        .execute(&mut self.ecs, &mut self.ecs_resources);
                }
            }
            std::mem::drop(run_state);

            // 1a -> Handle messages
            super::messaging::process_queues(&mut self.ecs, &mut self.ecs_resources);

            // Phase 2: Actually render stuff
            self.update_camera();

            let mut query = <(&Position, &CameraOptions)>::query();
            let mut camera_z = 0;
            for (pos, _camopts) in query.iter(&self.ecs) {
                camera_z = pos.as_point3().z;
            }
            let terrain_pass = self.terrain_pass.as_ref().unwrap();
            terrain_pass.render(core, &self.chunks, camera_z, self.gbuffer.as_ref().unwrap());

            self.model_pass.as_mut().unwrap().render(
                core,
                &mut self.ecs,
                &self.chunks.frustrum,
                self.gbuffer.as_ref().unwrap(),
            );

            self.grass_pass.as_mut().unwrap().render(
                core,
                &mut self.ecs,
                &self.chunks.frustrum,
                self.gbuffer.as_ref().unwrap(),
            );

            {
                let run_state = self.ecs_resources.get::<RunState>().unwrap().clone();
                self.vox_pass.as_mut().unwrap().render(
                    core,
                    &mut self.ecs,
                    &self.chunks.frustrum,
                    self.palette.as_ref().unwrap(),
                    self.gbuffer.as_ref().unwrap(),
                    &run_state,
                );
            }

            self.lighting_pass.as_mut().unwrap().render(
                core,
                &mut self.ecs,
                self.gbuffer.as_ref().unwrap(),
            );

            let mut rs = self.ecs_resources.get_mut::<RunState>();
            let run_state = rs.as_mut().unwrap();
            self.cursor_pass
                .as_mut()
                .unwrap()
                .render(core, &mut self.ecs, &run_state);

            // Phase 3: Draw the UI
            super::ui::draw_tooltips(&self.ecs, &core.mouse_world_pos, &core.imgui);
            super::ui::draw_main_menu(&self.ecs, run_state, &core.imgui);
            let mut mine_state = self.ecs_resources.get_mut::<MiningMap>();
            let mut lumber_state = self.ecs_resources.get_mut::<LumberMap>();
            let ms = mine_state.as_mut().unwrap();
            let ls = lumber_state.as_mut().unwrap();
            design_ui(run_state, core, &mut self.ecs, ms, ls);
        }

        result
    }
}

fn design_ui(
    run_state: &mut RunState,
    core: &mut Core,
    ecs: &mut World,
    mine_state: &mut MiningMap,
    lumber_state: &mut LumberMap,
) {
    match run_state {
        RunState::Design {
            mode: DesignMode::Lumberjack,
        } => {
            super::ui::lumberjack_display(core.imgui, ecs, &core.mouse_world_pos, lumber_state);
        }
        RunState::Design {
            mode: DesignMode::Buildings { bidx, .. },
        } => {
            let (bidx, vox) =
                super::ui::building_display(core.imgui, ecs, &core.mouse_world_pos, *bidx);
            *run_state = RunState::Design {
                mode: DesignMode::Buildings { bidx, vox },
            };
        }
        RunState::Design {
            mode: DesignMode::Mining { mode },
        } => {
            super::ui::mining_display(core.imgui, ecs, &core.mouse_world_pos, mode, mine_state);
        }
        RunState::Design {
            mode: DesignMode::SettlerList,
        } => {
            super::ui::settler_list_display(core.imgui, ecs);
        }
        _ => {}
    }
}
