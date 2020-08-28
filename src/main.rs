use bengine::*;
mod loader;

pub enum GameMode {
    Loader,
}

trait NoxMode {
    fn tick(&mut self, core: &mut Core) -> GameMode;
}

struct NoxFutura {
    current_mode: GameMode,
    modes: Vec<Box<dyn NoxMode>>,
}

impl NoxFutura {
    fn new() -> Self {
        Self {
            current_mode: GameMode::Loader,
            modes: Vec::new(),
        }
    }
}

impl BEngineGame for NoxFutura {
    fn init(&mut self) {
        self.modes.push(Box::new(loader::Loader::new()));
    }

    fn tick(&mut self, core: &mut Core) -> bool {
        let _new_mode = match self.current_mode {
            GameMode::Loader => self.modes[0].tick(core),
        };
        true
    }
}

fn main() {
    run(NoxFutura::new(), "Nox Futura");
}
