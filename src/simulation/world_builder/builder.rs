use crate::simulation::world_builder::planet::Planet;
use bracket_noise::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator};
use crate::display::WORLD_GEN_STATUS;

pub struct WorldBuilder {
    seed: u64,
}

impl WorldBuilder {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn go(&mut self) {
        *WORLD_GEN_STATUS.lock() = format!("Starting to build the world with seed {}", self.seed);
        let noise = self.build_noise();
        let planet = Planet::whole_world(self.seed);

        let altitude_map = self.build_base_altitude(&noise, &planet);
        let erosion_map = self.erode(&altitude_map, &planet);
        println!("Most erosion: {}", erosion_map.iter().max().unwrap());
        println!("Most altitude: {}", altitude_map.iter().max().unwrap());
        println!("Done");
    }

    fn build_noise(&self) -> FastNoise {
        let mut noise = FastNoise::seeded(self.seed);
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.5);
        noise.set_fractal_lacunarity(1.0); // TODO: Make editable
        noise.set_frequency(0.01);
        noise
    }

    fn build_base_altitude(&self, noise: &FastNoise, planet: &Planet) -> Vec<i16> {
        let whole_width = planet.landblock_dimensions.0;
        let whole_height = planet.landblock_dimensions.1;
        let whole_total = whole_height * whole_width;
        let mut base_altitude = vec![0i16; whole_total];

        base_altitude
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, h)| {
                let x = idx % whole_width;
                let y = idx / whole_width;

                let x_extent = x as f32 / whole_width as f32;
                let y_extent = y as f32 / whole_height as f32;
                let lat = (180.0 * x_extent) + 90.0;
                let lon = (360.0 * y_extent) + 180.0;

                let height = noise.get_noise3d(lon, lat, 0.0);
                let height_i = (height * 1_000.0) as i16;
                *h = height_i;
            });

        base_altitude
    }

    fn erode(&self, base_map: &[i16], planet: &Planet) -> Vec<u8> {
        let whole_width = planet.landblock_dimensions.0;
        let whole_height = planet.landblock_dimensions.1;
        let whole_total = whole_height * whole_width;

        let mut erosion = vec![0u8; whole_total];
        let mut water = vec![0u8; whole_total];
        for i in 0..20 {
            *WORLD_GEN_STATUS.lock() = format!("Erosion round {}", i);

            let above_ocean : Vec<usize> = base_map
                .par_iter()
                .enumerate()
                .filter_map(|(idx, height)| {
                    if *height - erosion[idx] as i16 > 0 {
                        Some(idx)
                    } else {
                        None
                    }
                })
                .collect();

            above_ocean.iter().for_each(|idx| water[*idx] += 1);

            // While there is still water in the system
            while water.iter().filter(|w| **w > 0).count() > 0 {
                let mut water_change = Vec::<(usize, i8)>::new();
                let mut candidates = Vec::new();
                for (idx, _) in water.iter_mut().enumerate().filter(|(_,w)| **w > 0) {
                    let x = idx % whole_width;
                    let y = idx / whole_width;
                    if x > 0 {
                        candidates.push((idx-1, base_map[idx-1]-erosion[idx-1] as i16));
                    }
                    if x < whole_width-1 {
                        candidates.push((idx+1, base_map[idx+1]-erosion[idx+1] as i16));
                    }
                    if y > 0 {
                        candidates.push((idx-whole_width, base_map[idx-whole_width]-erosion[idx-whole_width] as i16));
                    }
                    if y < whole_height-1 {
                        candidates.push((idx+whole_width, base_map[idx+whole_width]-erosion[idx+whole_width] as i16));
                    }

                    if !candidates.is_empty() {
                        candidates.sort_by(|a,b| a.1.cmp(&b.1));
                        water_change.push((candidates[0].0, 1));
                        water_change.push((idx, -1));
                        erosion[idx] += 1;
                    }
                    candidates.clear();
                }

                water_change.iter().for_each(|(idx, amount)| {
                    let w = water[*idx] as i8 + amount;
                    water[*idx] = w as u8;
                });

                // Remove water that has reached the ocean
                water.iter_mut().enumerate().for_each(|(idx, w)| {
                    if *w > 0 { *w -= 1; }
                    if *w > 0 && (base_map[idx] - erosion[idx] as i16) < 1 {
                        *w = 0;
                    }
                });
            }
        }

        erosion
    }
}
