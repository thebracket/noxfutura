use super::{Block, BlockType, Planet};
use parking_lot::Mutex;
pub mod noise_helper;
mod planet_categories;
mod planet_noise;
mod render_interface;
pub use render_interface::WORLDGEN_RENDER;
mod biomes;
mod rivers;
use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
use std::fs::File;
use crate::region::Region;

#[derive(Clone)]
pub struct PlanetParams {
    pub world_seed: i32,
    pub water_level: i32,
    pub plains_level: i32,
    pub starting_settlers: i32,
    pub strict_beamdown: bool,
    pub extra_noise: bool
}

pub struct PlanetBuilder {
    pub params: PlanetParams,
    planet: Planet,
    done: bool,
    task: String,
    flatmap: bool
}

impl PlanetBuilder {
    fn new() -> Self {
        Self {
            params: PlanetParams {
                world_seed: 0,
                water_level: 3,
                plains_level: 3,
                starting_settlers: 6,
                strict_beamdown: true,
                extra_noise: true
            },
            planet: Planet::new(),
            done: false,
            task: "Initializing".to_string(),
            flatmap: false
        }
    }
}

lazy_static! {
    pub static ref PLANET_BUILD: Mutex<PlanetBuilder> = Mutex::new(PlanetBuilder::new());
}

pub fn start_building_planet(params: PlanetParams) {
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
    biomes::build_biomes();
    rivers::run_rivers();
    // History
    // Save
    save_world();

    // Find crash site
    let crash = find_crash_site();
    let crash_idx = super::planet_idx(crash.x, crash.y);

    // Materialize region
    set_worldgen_status("Erasing the crash site");
    let clone_planet = &PLANET_BUILD.lock().planet.clone();
    let mut region = Region::zeroed(crash_idx, &clone_planet);
    crate::region::builder(&mut region, &clone_planet, crash);

    // It's all done
    set_worldgen_status("Done");
    PLANET_BUILD.lock().done = true;
}

fn save_world() {
    set_worldgen_status("Saving the world. To disk, sadly.");
    let world_file = File::create("world.dat").unwrap();
    let clone_planet = &PLANET_BUILD.lock().planet.clone();
    serde_cbor::to_writer(world_file, &clone_planet).unwrap();
}

fn find_crash_site() -> Point {
    use super::{WORLD_HEIGHT, WORLD_WIDTH};
    set_worldgen_status("Deciding where to crash");
    let seed = PLANET_BUILD.lock().planet.rng_seed;
    let mut rng = RandomNumberGenerator::seeded(seed);
    let mut result;
    loop {
        result = Point::new(
            rng.roll_dice(1, WORLD_WIDTH as i32 - 1),
            rng.roll_dice(1, WORLD_HEIGHT as i32 - 1),
        );
        let pidx = super::planet_idx(result.x, result.y);
        let bt = PLANET_BUILD.lock().planet.landblocks[pidx].btype;
        let h = PLANET_BUILD.lock().planet.landblocks[pidx].height;
        if bt != BlockType::Water && bt != BlockType::Marsh &&  h > PLANET_BUILD.lock().planet.water_height {
            println!("{:?}", bt);
            break;
        }
    }

    result
}

pub fn set_worldgen_status<S: ToString>(status: S) {
    PLANET_BUILD.lock().task = status.to_string();
}

pub fn get_worldgen_status() -> String {
    PLANET_BUILD.lock().task.clone()
}

pub fn get_flatmap_status() -> bool {
    PLANET_BUILD.lock().flatmap
}

pub fn set_flatmap_status(status: bool) {
    PLANET_BUILD.lock().flatmap = status;
}
