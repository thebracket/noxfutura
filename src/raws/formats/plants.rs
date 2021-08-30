use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Plants {
    pub plants: Vec<PlantDef>,
}

impl Plants {
    pub fn new() -> Self {
        Self { plants: Vec::new() }
    }

    pub fn plant_by_tag(&self, tag: &str) -> Option<&PlantDef> {
        for b in self.plants.iter() {
            if b.tag == tag {
                return Some(b);
            }
        }
        println!("Unable to find plant tag: {}", tag);
        None
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
    pub tag: String,
    pub name: String,
    pub description: String,
    pub harvest: Vec<String>,
    pub min_hardiness: u8,
    pub max_hardiness: u8,
    pub soil_quality: u8,
    pub vox: String,
}