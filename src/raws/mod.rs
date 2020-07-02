mod formats;
pub use formats::*;
use formats::{load_biomes, load_materials};
use parking_lot::RwLock;
mod material_map;
pub use material_map::MappedTexture;
use crate::modes::loader_progress;

pub struct Raws {
    pub biomes: Biomes,
    pub materials: Materials,
    pub matmap: material_map::MaterialMap,
    pub plants: Plants,
    pub buildings: Buildings,
    pub vox: VoxelModels,
    pub species: Species,
    pub names: Names,
    pub professions: Professions,
    pub clothing: Clothing,
    pub items: Items
}

impl Raws {
    fn new() -> Self {
        Self {
            biomes: Biomes::new(),
            materials: Materials::new(),
            matmap: material_map::MaterialMap::new(),
            plants: Plants::new(),
            buildings: Buildings::new(),
            vox: VoxelModels::new(),
            species: Species::new(),
            names: Names::new(),
            professions: Professions::new(),
            clothing: Clothing::new(),
            items: Items::new()
        }
    }

    fn load(&mut self) {
        loader_progress(0.01, "Loading Biome Data", false);
        self.biomes = load_biomes();
        loader_progress(0.02, "Loading Material Data", false);
        self.materials = load_materials();
        loader_progress(0.03, "Loading Plant Data", false);
        self.plants = load_plants();
        loader_progress(0.04, "Loading Voxel Data", false);
        self.vox = load_vox();
        loader_progress(0.05, "Loading Building Data", false);
        self.buildings = load_buildings();
        loader_progress(0.06, "Loading Species Data", false);
        self.species = load_species();
        loader_progress(0.07, "Loading Name Data", false);
        self.names = load_names();
        loader_progress(0.08, "Loading Profession Data", false);
        self.professions = load_professions();
        loader_progress(0.09, "Loading Clothing Data", false);
        self.clothing = load_clothing();
        loader_progress(0.091, "Loading Item Data", false);
        self.items = load_items();
    }
}

lazy_static! {
    pub static ref RAWS: RwLock<Raws> = RwLock::new(Raws::new());
}

pub fn load_raws() {
    RAWS.write().load();
}

pub fn get_material_by_tag(name: &str) -> Option<usize> {
    let lock = RAWS.read();
    let finder = lock
        .materials
        .materials
        .iter()
        .enumerate()
        .find(|(_, m)| m.name == name);
    if finder.is_some() {
        Some(finder.unwrap().0)
    } else {
        None
    }
}
