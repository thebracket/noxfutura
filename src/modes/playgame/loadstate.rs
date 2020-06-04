use crate::planet::SavedGame;
use parking_lot::Mutex;

#[derive(Clone)]
pub enum LoadState {
    Idle,
    Loading,
    Loaded { game: SavedGame },
}

lazy_static! {
    pub static ref LOAD_STATE: Mutex<LoadState> = Mutex::new(LoadState::Idle);
}
