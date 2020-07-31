mod resources;
use resources::SharedResources;
mod loader;
pub use loader::loader_progress;
use loader::Loader;
mod main_menu;
use main_menu::MainMenu;
mod helpers;
mod planetgen;
use planetgen::PlanetGen;
mod planetgen2;
use planetgen2::PlanetGen2;
mod playgame;
pub use playgame::{DesignMode, RunState, MiningMode};
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

impl<'a> Program {
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

    pub fn init(&mut self) {
        println!("Init started");
        self.resources.init();
        println!("Resources initialized");
        self.planet_gen2.setup();
        println!("Planetgen initialized, starting to build play mode");
        self.play.setup();
        println!("Init finished");
    }

    pub fn on_resize(&mut self) {
        self.play.on_resize();
    }

    pub fn tick(
        &mut self,
        frame: &wgpu::SwapChainOutput,
        depth_id: usize,
        imgui: &imgui::Ui,
        keycode: Option<winit::event::VirtualKeyCode>,
        mouse_world_pos: &(usize, usize, usize),
    ) -> bool {
        match self.mode {
            ProgramMode::Loader => self.mode = self.loader.tick(&self.resources, frame, imgui),
            ProgramMode::MainMenu => self.mode = self.main_menu.tick(&self.resources, frame, imgui),
            ProgramMode::PlanetGen => {
                self.mode = self.planet_gen.tick(&self.resources, frame, imgui)
            }
            ProgramMode::PlanetGen2 => {
                self.mode = self
                    .planet_gen2
                    .tick(&self.resources, frame, imgui, depth_id)
            }
            ProgramMode::Resume => {
                helpers::render_menu_background(frame, &self.resources);
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
                    imgui,
                    depth_id,
                    keycode,
                    self.last_frame.elapsed().as_millis(),
                    mouse_world_pos,
                );
            }
            ProgramMode::Quit => return false,
        }

        self.last_frame = Instant::now();
        true
    }

    pub fn get_mouse_buffer(&self) -> Option<&wgpu::Buffer> {
        self.play.get_mouse_buffer()
    }
}
