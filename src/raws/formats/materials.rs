use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Materials {
    pub materials: Vec<MaterialDef>,
}

impl Materials {
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MaterialDef {
    pub name: String,
    pub layer: MaterialLayer,
    pub hit_points: u32,
    pub mines_to: Vec<MinesTo>,
    pub description: String,
    pub texture: String,
    pub constructed: String,
    pub floor: String,
    pub floor_constructed: String,
    pub tint: (f32, f32, f32),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum MaterialLayer {
    ClusterRock { parent: String },
    Igneous,
    Sedimentary,
    Soil,
    Sand,
    Synthetic
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MinesTo {
    Item { name: String },
    Ore { name: String },
}

pub fn load_materials() -> Materials {
    let mat_path = "resources/raws/materials.ron";
    let f = File::open(&mat_path).expect("Failed opening file");
    let materials: Materials = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load materials: {}", e);
            std::process::exit(1);
        }
    };
    materials
}
