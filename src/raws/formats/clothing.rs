use super::ProfClothLoc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Clothing {
    pub clothing: Vec<ClothingDef>,
}

impl Clothing {
    pub fn new() -> Self {
        Self {
            clothing: Vec::new(),
        }
    }

    pub fn clothing_by_tag(&self, tag: &str) -> Option<ClothingDef> {
        for c in self.clothing.iter() {
            if c.tag == tag {
                return Some(c.clone());
            }
        }
        None
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ClothingDef {
    pub tag: String,
    pub name: String,
    pub description: String,
    pub armor: f32,
    pub model: String,
    pub item_model: String,
    pub colors: Vec<String>,
    pub slot: ProfClothLoc,
}