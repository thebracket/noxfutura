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

pub enum ProgramMode {
    Loader,
    MainMenu,
    PlanetGen,
    PlanetGen2,
    Quit,
}

pub struct Program {
    mode: ProgramMode,
    resources: SharedResources,
    loader: Loader,
    main_menu: MainMenu,
    planet_gen: PlanetGen,
    planet_gen2: PlanetGen2,
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
        }
    }

    pub fn init(&mut self, context: &mut crate::engine::Context) {
        self.resources.init(context);
        self.planet_gen2.setup(context);
    }

    pub fn tick(
        &mut self,
        context: &mut crate::engine::Context,
        frame: &wgpu::SwapChainOutput,
        depth_id: usize,
        imgui: &imgui::Ui,
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
            ProgramMode::Quit => return false,
        }
        true
    }
}
