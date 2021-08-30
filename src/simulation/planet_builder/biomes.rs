use super::{Planet, PlanetMesh, PLANET_GEN};

pub fn planet_biomes(planet: &mut Planet) {
    use crate::raws::{BiomeType, RAWS};
    use bracket_random::prelude::RandomNumberGenerator;
    let biome_reader = RAWS.read();
    let mut rng = RandomNumberGenerator::seeded(planet.rng_seed);
    for i in 0..planet.landblocks.len() {
        let lb = &planet.landblocks[i];
        let possible_biomes: Vec<(usize, &BiomeType)> = biome_reader
            .biomes
            .areas
            .iter()
            .enumerate()
            .filter(|b| b.1.occurs.contains(&lb.btype))
            .filter(|b| {
                lb.temperature_c >= b.1.min_temp as f32 && lb.temperature_c < b.1.max_temp as f32
            })
            .filter(|b| {
                lb.rainfall_mm >= b.1.min_rain as i32 && lb.rainfall_mm < b.1.max_rain as i32
            })
            .collect();

        if possible_biomes.is_empty() {
            panic!("No biomes for {:#?}", lb);
        } else {
            if let Some(choice) = rng.random_slice_entry(&possible_biomes) {
                planet.landblocks[i].biome_idx = choice.0;
                //println!("Selected: {:?} : {}", planet.landblocks[i].btype, choice.1.name);
            }
        }

        // Render Result
        if i % 200 == 0 {
            let mut bumpy_planet = PlanetMesh::new();
            bumpy_planet.with_biomes(&planet);
            PLANET_GEN.write().globe_info = Some(bumpy_planet);
        }
    }
}
