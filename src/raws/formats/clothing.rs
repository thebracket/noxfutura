use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;
use super::ProfClothLoc;

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
    pub slot: ProfClothLoc
}

pub fn load_clothing() -> Clothing {
    let mat_path = "resources/raws/clothing.ron";
    let f = File::open(&mat_path).expect("Failed opening file");
    let clothing: Clothing = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load clothing: {}", e);
            std::process::exit(1);
        }
    };
    clothing
}
