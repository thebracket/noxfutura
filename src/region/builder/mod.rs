use super::{Region, Planet};
use crate::planet::{set_worldgen_status, planet_idx};
use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
mod heightmap;

pub fn builder(region : &mut Region, planet: &Planet, crash_site : Point) {
    set_worldgen_status("Locating biome information");
    let biome_info = crate::raws::RAWS.lock().biomes.areas[region.biome_raw_idx].clone();
    let mut rng = RandomNumberGenerator::seeded(planet.perlin_seed + planet_idx(crash_site.x, crash_site.y) as u64);

    set_worldgen_status("Establishing ground altitude");
    let mut hm = heightmap::build_empty_heightmap();
    crate::planet::WORLDGEN_RENDER.lock().region_heightmap(&hm);
    heightmap::build_heightmap_from_noise(&mut hm, crash_site, planet.perlin_seed);
    crate::planet::WORLDGEN_RENDER.lock().region_heightmap(&hm);

    set_worldgen_status("Locating Sub-Biomes");
    heightmap::create_subregions(
        &mut rng,
        planet.landblocks[planet_idx(crash_site.x, crash_site.y)].variance,
        &mut hm
    );
    crate::planet::WORLDGEN_RENDER.lock().region_heightmap(&hm);

    set_worldgen_status("Adding water");
    set_worldgen_status("Sub-regions");
    set_worldgen_status("Stratifying");
    set_worldgen_status("Layer cake");
    set_worldgen_status("Ramping");
    set_worldgen_status("Beaches");
    set_worldgen_status("Crashing the ship");
    set_worldgen_status("Building an ECS");
    set_worldgen_status("Trees");
    set_worldgen_status("Blight");
    set_worldgen_status("Trail of debris");
    set_worldgen_status("Escape pod");
    set_worldgen_status("Settlers");
    set_worldgen_status("Features");
    set_worldgen_status("Looking for the map");
    set_worldgen_status("Saving region");
}