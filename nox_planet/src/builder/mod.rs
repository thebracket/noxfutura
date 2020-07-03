use super::{Block, BlockType, Planet};
use parking_lot::Mutex;
mod biomes;
pub mod noise_helper;
mod planet_categories;
mod planet_noise;
mod rivers;
use super::Region;
use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;

#[derive(Clone)]
pub struct PlanetParams {
    pub world_seed: i32,
    pub water_level: i32,
    pub plains_level: i32,
    pub starting_settlers: i32,
    pub strict_beamdown: bool,
    pub extra_noise: bool,
}

pub struct PlanetBuilder {
    pub params: PlanetParams,
    planet: Planet,
    pub done: bool,
    task: String,
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
                extra_noise: true,
            },
            planet: Planet::new(),
            done: false,
            task: "Initializing".to_string(),
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

    // Find crash site
    let crash = find_crash_site();
    let crash_idx = super::planet_idx(crash.x as usize, crash.y as usize);

    // Materialize region
    set_worldgen_status("Erasing the crash site");
    let clone_planet = &PLANET_BUILD.lock().planet.clone();
    let mut region = Region::zeroed(crash_idx, &clone_planet);
    let world = super::region::builder(&mut region, &clone_planet, crash);

    // Save
    save_world(region, world);

    // It's all done
    set_worldgen_status("Done");
    PLANET_BUILD.lock().done = true;
}

fn save_world(region: Region, world: legion::prelude::World) {
    use super::{save_world, SavedGame};
    set_worldgen_status("Saving the world. To disk, sadly.");
    let pclone = PLANET_BUILD.lock().planet.clone();
    save_world(SavedGame {
        planet: pclone,
        current_region: region,
        ecs_text: nox_components::serialize_world(&world),
    });
}

fn find_crash_site() -> Point {
    use super::{WORLD_HEIGHT, WORLD_WIDTH};
    set_worldgen_status("Deciding where to crash");
    let seed = PLANET_BUILD.lock().planet.rng_seed;
    let mut rng = RandomNumberGenerator::seeded(seed);
    let mut result;
    loop {
        result = Point::new(
            (WORLD_WIDTH / 2) - 32 + rng.roll_dice(1, 64) as usize,
            (WORLD_HEIGHT / 2) - 32 + rng.roll_dice(1, 64) as usize,
        );
        let pidx = super::planet_idx(result.x as usize, result.y as usize);
        let bt = PLANET_BUILD.lock().planet.landblocks[pidx].btype;
        let h = PLANET_BUILD.lock().planet.landblocks[pidx].height;
        if bt != BlockType::Water
            && bt != BlockType::Marsh
            && bt != BlockType::Coastal
            && h > PLANET_BUILD.lock().planet.water_height
            && PLANET_BUILD.lock().planet.landblocks[pidx].temperature > 5
        {
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
