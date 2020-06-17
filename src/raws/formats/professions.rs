use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Professions {
    pub professions: Vec<ProfessionDef>,
}

impl Professions {
    pub fn new() -> Self {
        Self {
            professions: Vec::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProfessionDef {
    pub tag: String,
    pub name: String,
    pub modifiers: ProfModifiers,
    pub clothing: ProfClothing,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProfModifiers {
    pub str: Option<i32>,
    pub dex: Option<i32>,
    pub con: Option<i32>,
    pub int: Option<i32>,
    pub wis: Option<i32>,
    pub cha: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProfClothing {
    pub male: Vec<ProfCloth>,
    pub female: Vec<ProfCloth>,
    pub both: Vec<ProfCloth>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProfCloth {
    pub tag: String,
    pub loc: ProfClothLoc,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ProfClothLoc {
    Torso,
    Legs,
    Shoes,
    Head,
}

pub fn load_professions() -> Professions {
    let p_path = "resources/raws/professions.ron";
    let f = File::open(&p_path).expect("Failed opening file");
    let profs: Professions = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load professions: {}", e);
            std::process::exit(1);
        }
    };
    profs
}
