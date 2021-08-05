use super::water_particle::WaterParticle;
use crate::display::{PlanetBuilder, WORLD_GEN_DISPLAY, WORLD_GEN_STATUS};
use crate::simulation::{WORLD_HEIGHT, WORLD_WIDTH};
use crate::simulation::world_builder::planet::Planet;
use bracket_noise::prelude::*;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct WorldBuilder {
    seed: u64,
}

impl WorldBuilder {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn go(&mut self) {
        *WORLD_GEN_STATUS.lock() = format!("Dividing the Heavens and Earth");
        let noise = self.build_noise();
        //let planet = Planet::whole_world(self.seed);

        let mut altitude_map = self.build_base_altitude(&noise);
        self.send_base_map(&altitude_map);
        let mut min_altitude: Vec<i16> = altitude_map
            .par_iter()
            .map(|h| if *h > 0 { h / 2 } else { *h })
            .collect();
        self.erode(&mut altitude_map, &mut min_altitude);
        std::mem::drop(min_altitude);
        *WORLD_GEN_STATUS.lock() = format!("Erosion Done");
        println!("Done");
    }

    fn build_noise(&self) -> FastNoise {
        let mut noise = FastNoise::seeded(self.seed);
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(7);
        noise.set_fractal_gain(0.3);
        noise.set_fractal_lacunarity(3.0);
        noise.set_frequency(0.008);
        noise
    }

    fn sphere_vertex(&self, altitude: f32, lat: f32, lon: f32) -> (f32, f32, f32) {
        (
            altitude * f32::cos(lat) * f32::cos(lon),
            altitude * f32::cos(lat) * f32::sin(lon),
            altitude * f32::sin(lat)
        )
    }

    fn build_base_altitude(&self, noise: &FastNoise) -> Vec<i16> {
        let mut base_altitude = vec![0i16; WORLD_HEIGHT * WORLD_WIDTH];

        base_altitude
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, h)| {
                let x = idx % WORLD_WIDTH;
                let y = idx / WORLD_WIDTH;

                let x_extent = x as f32 / WORLD_WIDTH as f32;
                let y_extent = y as f32 / WORLD_HEIGHT as f32;
                let lat = ((180.0 * x_extent) - 90.0)  * 0.017_453_3;
                let lon = ((360.0 * y_extent) - 180.0)  * 0.017_453_3;
                let coords = self.sphere_vertex(100.0, lat, lon);

                let height = noise.get_noise3d(coords.0, coords.1, coords.2);
                let height_i = (height * 20_000.0) as i16;
                *h = height_i;
            });

        base_altitude
    }

    fn erode(&self, base_map: &mut [i16], min_altitude: &mut [i16]) {
        for i in 0..2 {
            *WORLD_GEN_STATUS.lock() = format!("Just Add Water: {}%", (i+1)*50);
            let max_altitude = base_map.iter().max().unwrap();
            let mut water_particles: Vec<WaterParticle> = base_map
                .par_iter()
                .enumerate()
                .filter_map(|(idx, height)| if *height > max_altitude/4 { Some(idx) } else { None })
                .map(|idx| WaterParticle::new(idx))
                .collect();

            while !water_particles.is_empty() {
                water_particles.par_iter_mut().for_each(|p| {
                    p.flow(base_map);
                });

                // Do some erosion here
                water_particles.iter().filter(|p| p.done).for_each(|p| {
                    for pidx in 0..p.history.len()-1 {
                        let idx = p.history[pidx];
                        if min_altitude[idx] < base_map[idx] {
                            base_map[idx] -= 1;
                        }
                    }
                    // Deposition would go here
                    self.send_base_map(base_map);
                });

                water_particles.retain(|p| !p.done);
            }
        }
    }

    fn send_base_map(&self, altitude_map: &[i16]) {
        let max_altitude = *(altitude_map.iter().max().unwrap()) as f32;
        let mut wg_lock = WORLD_GEN_DISPLAY.lock();
        let x_step = WORLD_WIDTH / 256;
        let y_step = WORLD_HEIGHT / 256;
        for y in 0..256 {
            let world_y = y * y_step;
            for x in 0..256 {
                let world_x = x * x_step;
                let idx = (world_y * WORLD_WIDTH) + world_x;
                let h = altitude_map[idx];
                let altitude = if h < 1 {
                    0
                } else {
                    (h as f32 * (255.0 / max_altitude)) as u8
                };
                wg_lock.base_altitude[(y * 256)+x] = altitude;
            }
        }
        wg_lock.dirty = true;
    }
}
