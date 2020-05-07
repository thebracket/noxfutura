mod formats;
pub use formats::{Biomes};
use formats::load_biomes;
use parking_lot::Mutex;

pub struct Raws {
    pub biomes : Biomes
}

impl Raws {
    fn new() -> Self {
        Self {
            biomes : Biomes::new()
        }
    }

    fn load(&mut self) {
        self.biomes = load_biomes();
    }
}

lazy_static! {
    pub static ref RAWS: Mutex<Raws> =
        Mutex::new(Raws::new());
}

pub fn load_raws() {
    RAWS.lock().load();
}