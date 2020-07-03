use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Plants {
    pub plants: Vec<PlantDef>,
}

impl Plants {
    pub fn new() -> Self {
        Self { plants: Vec::new() }
    }

    pub fn plants_by_hardiness_and_soil_quality(
        &self,
        temperature_c: i8,
        soil_quality: u8,
    ) -> Vec<usize> {
        let zone = Plants::temp_to_hardiness(temperature_c);
        self.plants
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                zone >= p.min_hardiness && zone <= p.max_hardiness && soil_quality >= p.soil_quality
            })
            .map(|(i, _)| i)
            .collect()
    }

    fn temp_to_hardiness(temperature_c: i8) -> u8 {
        //println!("Mean temperature: {}", temperature_c);
        let tmp = temperature_c + 60;
        let tmp_2 = tmp / 10;
        u8::min(15, tmp_2 as u8)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlantDef {
    pub name: String,
    pub harvest: Vec<String>,
    pub min_hardiness: u8,
    pub max_hardiness: u8,
    pub soil_quality: u8,
}

pub fn load_plants() -> Plants {
    let mat_path = "resources/raws/plants.ron";
    let f = File::open(&mat_path).expect("Failed opening file");
    let plants: Plants = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load materials: {}", e);
            std::process::exit(1);
        }
    };
    plants
}
