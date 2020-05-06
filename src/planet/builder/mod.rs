use parking_lot::Mutex;
use super::{Planet, Block, BlockType};
mod planet_noise;
mod noise_helper;
mod planet_categories;

#[derive(Clone)]
pub struct PlanetParams {
    pub world_seed: i32,
    pub water_level: i32,
    pub plains_level: i32,
    pub starting_settlers: i32,
    pub strict_beamdown : bool
}

struct PlanetBuilder {
    params : PlanetParams,
    planet : Planet,
    done : bool,
    task : String
}

impl PlanetBuilder {
    fn new() -> Self {
        Self {
            params : PlanetParams{
                world_seed: 0,
                water_level : 3,
                plains_level : 3,
                starting_settlers: 6,
                strict_beamdown: true,
            },
            planet : Planet::new(),
            done : false,
            task : "Initializing".to_string()
        }
    }
}

lazy_static! {
    static ref PLANET_BUILD : Mutex<PlanetBuilder> = Mutex::new(PlanetBuilder::new());
}

pub fn start_building_planet(params : PlanetParams) {
    let mut lock = PLANET_BUILD.lock();
    lock.planet.rng_seed = params.world_seed as u64;
    lock.planet.water_divisor = params.water_level;
    lock.planet.plains_divisor = params.plains_level;
    lock.planet.starting_settlers = params.starting_settlers;
    lock.planet.strict_beamdown = params.strict_beamdown;
    lock.params = params;
    std::mem::drop(lock);
    std::thread::spawn(threaded_builder);
}

fn threaded_builder() {
    planet_noise::zero_fill();
    planet_noise::planetary_noise();
    planet_categories::planet_type_allocation();
    planet_categories::planet_coastlines();
    planet_categories::planet_rainfall();
    // Biomes
    // Rivers
    // History
    // Save
    // Find crash site
    // Materialize region
    // It's all done
}

fn set_worldgen_status<S:ToString>(status : S) {
    PLANET_BUILD.lock().task = status.to_string();
}

pub fn get_worldgen_status() -> String {
    PLANET_BUILD.lock().task.clone()
}