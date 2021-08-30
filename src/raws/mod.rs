mod formats;
pub use formats::*;
use parking_lot::RwLock;
use lazy_static::*;
mod block_type;
pub use block_type::BlockType;

pub struct Raws {
    pub biomes: Biomes,
    pub materials: Materials,
    //pub matmap: material_map::MaterialMap,
    pub plants: Plants,
    pub buildings: Buildings,
    pub vox: VoxelModels,
    pub species: Species,
    pub names: Names,
    pub professions: Professions,
    pub clothing: Clothing,
    pub items: Items,
    pub reactions: Reactions,
    pub obj_models: WavefrontModels,
}

impl Raws {
    fn new() -> Self {
        Self {
            biomes: Biomes::new(),
            materials: Materials::new(),
            //matmap: material_map::MaterialMap::new(),
            plants: Plants::new(),
            buildings: Buildings::new(),
            vox: VoxelModels::new(),
            species: Species::new(),
            names: Names::new(),
            professions: Professions::new(),
            clothing: Clothing::new(),
            items: Items::new(),
            reactions: Reactions::new(),
            obj_models: WavefrontModels::new(),
        }
    }

    fn load_index(&self) -> Vec<String> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open("raws/index.txt").unwrap();
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|l| l.unwrap())
            .filter(|l| !l.is_empty() && !l.starts_with("# "))
            .collect()
    }

    fn load(&mut self) {
        self.names = load_names();

        let bundles = self.load_index();
        bundles.iter().for_each(|bf| {
            let bundle = RawBundle::load(&bf);
            bundle.merge(self);
        });
    }
}

lazy_static! {
    pub static ref RAWS: RwLock<Raws> = RwLock::new(Raws::new());
}

pub fn load_raws() {
    RAWS.write().load();
}
