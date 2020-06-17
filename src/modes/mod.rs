mod resources;
use resources::SharedResources;
mod loader;
use loader::Loader;
mod main_menu;
use main_menu::MainMenu;
mod helpers;
mod planetgen;
use planetgen::PlanetGen;
mod planetgen2;
use planetgen2::PlanetGen2;
mod render_interface;
pub use render_interface::WORLDGEN_RENDER;
mod playgame;
use playgame::{LoadState, PlayGame, LOAD_STATE};
use std::time::Instant;

pub enum ProgramMode {
    Loader,
    MainMenu,
    PlanetGen,
    PlanetGen2,
    Resume,
    PlayGame,
    Quit,
}

pub struct Program {
    mode: ProgramMode,
    resources: SharedResources,
    loader: Loader,
    main_menu: MainMenu,
    planet_gen: PlanetGen,
    planet_gen2: PlanetGen2,
    play: PlayGame,
    last_frame: Instant,
}

impl Program {
    pub fn new() -> Self {
        Self {
            mode: ProgramMode::Loader,
            resources: SharedResources::new(),
            loader: Loader::new(),
            main_menu: MainMenu::new(),
            planet_gen: PlanetGen::new(),
            planet_gen2: PlanetGen2::new(),
            play: PlayGame::new(),
            last_frame: Instant::now(),
        }
    }

    pub fn init(&mut self, context: &mut crate::engine::Context) {
        self.resources.init(context);
        self.planet_gen2.setup(context);
        self.play.setup(context);
    }

    pub fn on_resize(&mut self, context: &mut crate::engine::Context) {
        self.play.on_resize(context);
    }

    pub fn tick(
        &mut self,
        context: &mut crate::engine::Context,
        frame: &wgpu::SwapChainOutput,
        depth_id: usize,
        imgui: &imgui::Ui,
        keycode: Option<winit::event::VirtualKeyCode>,
    ) -> bool {
        match self.mode {
            ProgramMode::Loader => {
                self.mode = self.loader.tick(&self.resources, frame, context, imgui)
            }
            ProgramMode::MainMenu => {
                self.mode = self.main_menu.tick(&self.resources, frame, context, imgui)
            }
            ProgramMode::PlanetGen => {
                self.mode = self.planet_gen.tick(&self.resources, frame, context, imgui)
            }
            ProgramMode::PlanetGen2 => {
                self.mode = self
                    .planet_gen2
                    .tick(&self.resources, frame, context, imgui, depth_id)
            }
            ProgramMode::Resume => {
                helpers::render_menu_background(context, frame, &self.resources);
                use imgui::*;
                let ls = LOAD_STATE.lock().clone();
                match ls {
                    LoadState::Idle => self.play.load(),
                    LoadState::Loading => {
                        let window = imgui::Window::new(im_str!("Loading game"));
                        window
                            .size([300.0, 100.0], Condition::FirstUseEver)
                            .build(imgui, || {
                                imgui.text(im_str!("Please wait..."));
                            });
                    }
                    LoadState::Loaded { .. } => {
                        self.play.finish_loading();
                        self.mode = ProgramMode::PlayGame;
                    }
                }
            }
            ProgramMode::PlayGame => {
                self.mode = self.play.tick(
                    &self.resources,
                    frame,
                    context,
                    imgui,
                    depth_id,
                    keycode,
                    self.last_frame.elapsed().as_millis(),
                );
            }
            ProgramMode::Quit => return false,
        }

        self.last_frame = Instant::now();
        true
    }
}
