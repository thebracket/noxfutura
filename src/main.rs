#[macro_use]
extern crate lazy_static;

use bengine::*;
mod modes;
mod raws;
use modes::*;
pub use raws::{load_raws, RAWS};
pub mod components;
pub mod planet;
pub mod spatial;
pub mod utils;

pub enum GameMode {
    Loader,
    MainMenu,
    WorldGen1,
    WorldGen2,
    Quitting,
    PlayGame,
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
        self.modes.push(Box::new(Loader::new()));
        self.modes.push(Box::new(MainMenu::new()));
        self.modes.push(Box::new(WorldGen1::new()));
        self.modes.push(Box::new(WorldGen2::new()));
        self.modes.push(Box::new(PlayTheGame::new()))
    }

    fn tick(&mut self, core: &mut Core) -> bool {
        let new_mode = match self.current_mode {
            GameMode::Loader => self.modes[0].tick(core, self.shared_resources.as_ref().unwrap()),
            GameMode::MainMenu => self.modes[1].tick(core, self.shared_resources.as_ref().unwrap()),
            GameMode::WorldGen1 => {
                self.modes[2].tick(core, self.shared_resources.as_ref().unwrap())
            }
            GameMode::WorldGen2 => {
                self.modes[3].tick(core, self.shared_resources.as_ref().unwrap())
            }
            GameMode::Quitting => {
                return false;
            }
            GameMode::PlayGame => self.modes[4].tick(core, self.shared_resources.as_ref().unwrap()),
        };
        self.current_mode = new_mode;
        true
    }
}

fn main() {
    run(NoxFutura::new(), "Nox Futura");
}
