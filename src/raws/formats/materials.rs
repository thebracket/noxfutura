use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Materials {
    pub materials: Vec<MaterialDef>,
    pub material_list : Option<Vec<TextureMap>>
}

impl Materials {
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
            material_list : None
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MaterialTextureSet {
    pub base : Option<String>,
    pub floor : Option<String>,
    pub constructed : Option<String>,
    pub floor_constructed : Option<String>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MaterialDef {
    pub name: String,
    pub layer: MaterialLayer,
    pub hit_points: u32,
    pub mines_to: Vec<MinesTo>,
    pub description: String,
    pub tint: (f32, f32, f32),
    pub texture: Option<MaterialTextureSet>
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureMap {
    pub id: usize,
    pub base: f32,
    pub constructed: f32,
    pub floor: f32,
    pub floor_constructed: f32
}