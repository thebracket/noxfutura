use crate::{GameMode, NoxMode, SharedResources};
use bengine::*;
use super::loadstate::*;
use legion::*;
use super::systems::REGION;

pub struct PlayTheGame {
    ready: bool,
    started_loader: bool,
    planet: Option<crate::planet::Planet>,
    pub ecs: World,
    pub ecs_resources: Resources,
}

impl PlayTheGame {
    pub fn new() -> Self {
        LOAD_STATE.write().state = LoadState::Idle;
        Self {
            ready: false,
            started_loader: false,
            planet: None,
            ecs: World::default(),
            ecs_resources: Resources::default()
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
                },
                LoadState::Loaded { game } => {
                    LOAD_STATE.write().state = LoadState::Idle;
                    self.planet = Some(game.planet);
                    *REGION.write() = game.current_region;
                    self.ecs = crate::components::deserialize_world(game.ecs_text);

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
                _ => panic!("Not meant to go here."),
            }
        }
    }
}

impl NoxMode for PlayTheGame {
    fn tick(&mut self, core: &mut Core, shared: &SharedResources) -> GameMode {
        use gui::*;
        let mut result = GameMode::PlayGame;

        if !self.ready {
            self.load();
            shared.quad_render.render(shared.background_image, core);
            let window = gui::Window::new(im_str!("Loading saved game - please wait"));
            window
                .size([300.0, 100.0], Condition::FirstUseEver)
                .collapsed(true, Condition::FirstUseEver)
                .build(core.imgui, || {
                });
        }

        result
    }
}
