use super::{Planet, Region};
use crate::planet::{planet_idx, set_worldgen_status, REGION_HEIGHT, REGION_WIDTH};
use bracket_geometry::prelude::Point;
use bracket_random::prelude::RandomNumberGenerator;
mod heightmap;
mod water_features;
mod strata;
pub mod chunks;
mod primitive;
pub use primitive::Primitive;
use crate::modes::WORLDGEN_RENDER;

pub fn builder(region: &mut Region, planet: &Planet, crash_site: Point) {
    crate::planet::set_flatmap_status(true);
    set_worldgen_status("Locating biome information");
    //let biome_info = crate::raws::RAWS.lock().biomes.areas[region.biome_raw_idx].clone();
    let biome = planet.biomes[region.biome_info_idx].clone();
    let mut pooled_water = vec![0u8; REGION_WIDTH as usize * REGION_HEIGHT as usize];
    println!("{}", planet.water_height);
    let mut rng = RandomNumberGenerator::seeded(
        planet.perlin_seed + planet_idx(crash_site.x as usize, crash_site.y as usize) as u64,
    );

    set_worldgen_status("Establishing ground altitude");
    let mut hm = heightmap::build_empty_heightmap();
    WORLDGEN_RENDER
        .lock()
        .region_heightmap(&hm, planet.water_height, &pooled_water);
    heightmap::build_heightmap_from_noise(
        &mut hm,
        crash_site,
        planet.perlin_seed,
        planet.landblocks[planet_idx(crash_site.x as usize, crash_site.y as usize)].variance,
    );
    WORLDGEN_RENDER
        .lock()
        .region_heightmap(&hm, planet.water_height, &pooled_water);

    set_worldgen_status("Locating Sub-Biomes");
    heightmap::create_subregions(
        &mut rng,
        planet.landblocks[planet_idx(crash_site.x as usize, crash_site.y as usize)].variance,
        &mut hm,
        &mut pooled_water,
        &biome,
    );
    WORLDGEN_RENDER
        .lock()
        .region_heightmap(&hm, planet.water_height, &pooled_water);

    set_worldgen_status("Adding water features");
    water_features::just_add_water(planet, region, &mut pooled_water, &mut hm, &mut rng);
    WORLDGEN_RENDER
        .lock()
        .region_heightmap(&hm, planet.water_height, &pooled_water);

    set_worldgen_status("Stratifying");
    set_worldgen_status("Layer cake is yummy");
    strata::layer_cake(&hm, region);
    let mut display_chunks = chunks::Chunks::empty();
    display_chunks.rebuild_all(region);
    let primitives = display_chunks.all_geometry();
    println!("Primitives: {}", primitives.len());
    /*crate::planet::WORLDGEN_RENDER
        .lock()
        .region_display(region.clone());*/
    WORLDGEN_RENDER
        .lock()
        .region_display_primitives(primitives);

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
