mod resources;
use resources::SharedResources;
mod loader;
use loader::Loader;
mod main_menu;
use main_menu::MainMenu;
mod helpers;

pub enum ProgramMode {
    Loader,
    MainMenu,
    Quit
}

pub struct Program {
    mode: ProgramMode,
    resources: SharedResources,
    loader: Loader,
    main_menu : MainMenu
}

impl Program {
    pub fn new() -> Self {
        Self {
            mode: ProgramMode::Loader,
            resources: SharedResources::new(),
            loader: Loader::new(),
            main_menu: MainMenu::new()
        }
    }

    pub fn init(&mut self, context: &mut crate::engine::Context) {
        self.resources.init(context);
    }

    pub fn tick(
        &mut self,
        context: &mut crate::engine::Context,
        frame: &wgpu::SwapChainOutput,
        _depth_id: usize,
        imgui: &imgui::Ui,
    ) -> bool {
        match self.mode {
            ProgramMode::Loader => {
                self.mode = self
                    .loader
                    .tick(&self.resources, frame, context, imgui)
            }
            ProgramMode::MainMenu => {
                self.mode = self
                    .main_menu
                    .tick(&self.resources, frame, context, imgui)
            }
            ProgramMode::Quit => return false
        }
        true
    }
}
