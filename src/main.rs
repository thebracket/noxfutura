use bengine::*;
mod loader;

enum GameMode {
    Loader
}

struct NoxFutura {
    current_mode: GameMode,
    loader: Option<loader::Loader>,
}

impl NoxFutura {
    fn new() -> Self {
        Self {
            current_mode: GameMode::Loader,
            loader: None,
        }
    }
}

impl BEngineGame for NoxFutura {
    fn init(&mut self, init: &mut Initializer) {
        self.loader = Some(loader::Loader::new(init));
    }

    fn tick(&mut self, core: &mut Core) -> bool {
        match self.current_mode {
            GameMode::Loader => self.loader.as_mut().unwrap().render(core)
        }
    }
}

fn main() {
    run(NoxFutura::new() );
}
