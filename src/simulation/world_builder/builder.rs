use crate::display::{WORLD_GEN_DISPLAY, WORLD_GEN_STATUS};

use crate::simulation::{WORLD_HEIGHT, WORLD_WIDTH};
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
        *WORLD_GEN_STATUS.lock() = format!("Raining on Everyone's Parade");
        let mut min_altitude: Vec<i16> = altitude_map
            .par_iter()
            .map(|h| if *h > 0 { h / 2 } else { *h })
            .collect();
        super::erosion::erode(&mut altitude_map, &mut min_altitude);
        std::mem::drop(min_altitude);
        self.send_base_map(&altitude_map);
        *WORLD_GEN_STATUS.lock() = format!("It's Grim Up North");
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
            altitude * f32::sin(lat),
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
                let lat = ((180.0 * x_extent) - 90.0) * 0.017_453_3;
                let lon = ((360.0 * y_extent) - 180.0) * 0.017_453_3;
                let coords = self.sphere_vertex(100.0, lat, lon);

                let height = noise.get_noise3d(coords.0, coords.1, coords.2);
                let height_i = (height * 9500.0) as i16;
                *h = height_i;
            });

        base_altitude
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
                wg_lock.base_altitude[(y * 256) + x] = altitude;
            }
        }
        wg_lock.dirty = true;
    }
}
