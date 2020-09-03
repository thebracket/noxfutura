use crate::planet::SavedGame;
use parking_lot::RwLock;

#[derive(Clone)]
pub enum LoadState {
    Idle,
    Loading,
    Loaded { game: SavedGame },
}

pub struct GameLoader {
    pub state: LoadState,
}

lazy_static! {
    pub static ref LOAD_STATE: RwLock<GameLoader> = RwLock::new(GameLoader {
        state: LoadState::Idle
    });
}
