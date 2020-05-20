use crate::opengl::*;
mod loader;
mod resources;
mod helpers;
mod main_menu;
mod planetgen;
mod planetgen2;

#[derive(Copy, Clone)]
pub enum ProgramMode {
    Loader,
    MainMenu,
    PlanetGen,
    PlanetGen2,
    Quit,
}

pub struct Program {
    mode: ProgramMode,
    resources: resources::SharedResources,
    loader : loader::Loader,
    mainmenu : main_menu::MainMenu,
    planetgen : planetgen::PlanetGen,
    planetgen2 : planetgen2::PlanetGen2
}

impl Program {
    pub fn new() -> Self {
        Self {
            mode: ProgramMode::Loader,
            resources: resources::SharedResources::new(),
            loader: loader::Loader::new(),
            mainmenu : main_menu::MainMenu::new(),
            planetgen: planetgen::PlanetGen::new(),
            planetgen2 : planetgen2::PlanetGen2::new()
        }
    }

    pub fn init(&mut self, gl: &Gl) {
        self.resources.init(gl);
        self.planetgen2.init(gl);
    }

    pub fn tick(
        &mut self,
        ui: &imgui::Ui,
        gl: &Gl
    ) -> bool {
        self.mode = match self.mode {
            ProgramMode::Loader => self.loader.tick(ui, gl, &self.resources),
            ProgramMode::MainMenu => self.mainmenu.tick(gl, &self.resources, ui),
            ProgramMode::PlanetGen => self.planetgen.tick(gl, &self.resources, ui),
            ProgramMode::PlanetGen2 => self.planetgen2.tick(gl, &self.resources, ui),
            _ => self.mode
        };
        true
    }
}
