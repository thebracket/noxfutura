use serde::{Deserialize, Serialize};

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
    Soil { quality: u8 },
    Sand,
    Synthetic,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MinesTo {
    Item { name: String },
    Ore { name: String },
}
