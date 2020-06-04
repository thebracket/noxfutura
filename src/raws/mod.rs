mod formats;
use formats::{load_biomes, load_materials};
pub use formats::*;
use parking_lot::Mutex;

pub struct Raws {
    pub biomes: Biomes,
    pub materials: Materials
}

impl Raws {
    fn new() -> Self {
        Self {
            biomes: Biomes::new(),
            materials: Materials::new()
        }
    }

    fn load(&mut self) {
        self.biomes = load_biomes();
        self.materials = load_materials();
    }
}

lazy_static! {
    pub static ref RAWS: Mutex<Raws> = Mutex::new(Raws::new());
}

pub fn load_raws() {
    RAWS.lock().load();
}
