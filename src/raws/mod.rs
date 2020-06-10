mod formats;
pub use formats::*;
use formats::{load_biomes, load_materials};
use parking_lot::RwLock;

pub struct Raws {
    pub biomes: Biomes,
    pub materials: Materials,
}

impl Raws {
    fn new() -> Self {
        Self {
            biomes: Biomes::new(),
            materials: Materials::new(),
        }
    }

    fn load(&mut self) {
        self.biomes = load_biomes();
        self.materials = load_materials();
    }
}

lazy_static! {
    pub static ref RAWS: RwLock<Raws> = RwLock::new(Raws::new());
}

pub fn load_raws() {
    RAWS.write().load();
}
