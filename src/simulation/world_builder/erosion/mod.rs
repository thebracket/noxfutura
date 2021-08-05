mod water_particle;
use water_particle::WaterParticle;

use crate::simulation::world_builder::planet::{
    average_precipitation_mm_by_latitude, average_temperature_by_latitude,
    temperature_decrease_by_altitude,
};
use crate::simulation::{WORLD_HEIGHT, WORLD_WIDTH};
use crate::types::Degrees;
use parking_lot::Mutex;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub(crate) fn erode(base_map: &mut [i16], min_altitude: &mut [i16]) {
    let mut water_particles: Vec<WaterParticle> = base_map
        .par_iter()
        .enumerate()
        .filter_map(|(idx, height)| {
            if *height > 4000 {
                let y = idx / WORLD_WIDTH;
                let lat = Degrees::new(((y as f32 / WORLD_HEIGHT as f32) * 180.0) - 90.0);
                let precipitation = average_precipitation_mm_by_latitude(lat);
                let temperature = average_temperature_by_latitude(lat)
                    - temperature_decrease_by_altitude(*height as f32);
                //println!("Lat: {}, Altitude: {}, Precipitation: {} mm, Temperature: {}c", lat.0, base_map[idx], precipitation, temperature);
                if precipitation > 1500.0 && temperature > 0.0 {
                    Some(idx)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .map(|idx| WaterParticle::new(idx))
        .collect();

    while !water_particles.is_empty() {
        water_particles.par_iter_mut().for_each(|p| {
            p.flow(base_map);
        });

        // Do some erosion here
        let changes = Mutex::new(Vec::<(usize, i16)>::new());
        water_particles.par_iter().filter(|p| p.done).for_each(|p| {
            for pidx in 0..p.history.len() - 1 {
                let idx = p.history[pidx];
                if min_altitude[idx] < base_map[idx] {
                    changes.lock().push((idx, -1));
                }
            }
            // Deposition would go here
        });
        changes
            .lock()
            .iter()
            .for_each(|(idx, change)| base_map[*idx] += *change);

        water_particles.retain(|p| !p.done);
    }
}
