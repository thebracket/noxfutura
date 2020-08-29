#[macro_use]
extern crate lazy_static;

use bengine::*;
mod loader;
mod shared_resources;
use shared_resources::SharedResources;
mod main_menu;
mod raws;
pub use raws::{load_raws, RAWS};

pub enum GameMode {
    Loader,
    MainMenu,
}

trait NoxMode {
    fn tick(&mut self, core: &mut Core, shared: &SharedResources) -> GameMode;
}

struct NoxFutura {
    current_mode: GameMode,
    modes: Vec<Box<dyn NoxMode>>,
    shared_resources: Option<SharedResources>,
}

impl NoxFutura {
    fn new() -> Self {
        Self {
            current_mode: GameMode::Loader,
            modes: Vec::new(),
            shared_resources: None,
        }
    }
}

impl BEngineGame for NoxFutura {
    fn init(&mut self) {
        self.shared_resources = Some(SharedResources::new());
        self.modes.push(Box::new(loader::Loader::new()));
        self.modes.push(Box::new(main_menu::MainMenu::new()))
    }

    fn tick(&mut self, core: &mut Core) -> bool {
        let new_mode = match self.current_mode {
            GameMode::Loader => self.modes[0].tick(core, self.shared_resources.as_ref().unwrap()),
            GameMode::MainMenu => self.modes[1].tick(core, self.shared_resources.as_ref().unwrap()),
        };
        self.current_mode = new_mode;
        true
    }
}

fn main() {
    run(NoxFutura::new(), "Nox Futura");
}
