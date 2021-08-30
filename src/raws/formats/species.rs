use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Species {
    pub species: Vec<SpeciesDef>,
}

impl Species {
    pub fn new() -> Self {
        Self {
            species: Vec::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpeciesDef {
    pub name: String,
    pub male_name: String,
    pub female_name: String,
    pub group_name: String,
    pub description: String,
    pub parts: Vec<SpeciesPart>,
    pub diet: Diet,
    pub alignment: Alignment,
    pub max_age: i32,
    pub infant_age: i32,
    pub child_age: i32,
    pub skin_colors: Vec<ColorDef>,
    pub hair_colors: Vec<ColorDef>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpeciesPart {
    pub tag: String,
    pub qty: i32,
    pub size: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ColorDef {
    pub tag: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Diet {
    Omnivore,
    Carnivore,
    Herbivore,
    Cannibal,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Alignment {
    Good,
    Neutral,
    Evil,
}