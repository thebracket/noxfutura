use serde::{Deserialize, Serialize};

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

    pub fn building_by_tag(&self, tag: &str) -> Option<&BuildingDef> {
        for b in self.buildings.iter() {
            if b.tag == tag {
                return Some(b);
            }
        }
        println!("Unable to find building tag: {}", tag);
        None
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BuildingDef {
    pub tag: String,
    pub name: String,
    pub components: Vec<BuildingComponent>,
    pub skill: Vec<BuildingSkill>,
    pub vox: String,
    pub description: String,
    pub blocked: Option<String>,
    pub provides: Vec<BuildingProvides>,
    pub dimensions: Option<(i32, i32, i32)>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BuildingComponent {
    pub item: String,
    pub qty: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BuildingSkill {
    pub skill: String,
    pub difficulty: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BuildingProvides {
    Light {
        radius: usize,
        color: (f32, f32, f32),
    },
    Sleep,
    Storage,
    Generator {
        energy: i32,
    },
    EnergyStorage {
        energy: i32,
    },
}