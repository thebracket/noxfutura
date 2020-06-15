use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Buildings {
    pub buildings: Vec<BuildingDef>,
}

impl Buildings {
    pub fn new() -> Self {
        Self {
            buildings: Vec::new(),
        }
    }

    pub fn get_building_idx(&self, tag: &str) -> usize {
        for (i,b) in self.buildings.iter().enumerate() {
            if b.tag == tag {
                return i;
            }
        }
        println!("No vox match for {}", tag);
        0
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BuildingDef {
    pub tag: String,
    pub name: String,
    pub components: Vec<BuildingComponent>,
    pub skill: Vec<BuildingSkill>,
    pub vox : String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BuildingComponent {
    pub item: String,
    pub qty: i32
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BuildingSkill {
    pub skill: String,
    pub difficulty: i32
}

pub fn load_buildings() -> Buildings {
    let mat_path = "resources/raws/buildings.ron";
    let f = File::open(&mat_path).expect("Failed opening file");
    let buildings: Buildings = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load materials: {}", e);
            std::process::exit(1);
        }
    };
    buildings
}
