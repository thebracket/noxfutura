use crate::{planet_idx, set_worldgen_status, Planet, Region};
use bengine::geometry::Point;
use bengine::random::RandomNumberGenerator;
use nox_spatial::{REGION_HEIGHT, REGION_WIDTH};
mod beaches;
mod buildings;
mod debris;
mod game_components;
mod heightmap;
mod map_components;
mod plants;
mod primitive;
mod ramping;
mod settlers;
mod strata;
mod trees;
mod water_features;
use legion::*;
pub use primitive::Primitive;
mod flags;
pub use flags::set_flags as rebuild_flags;
use nox_raws::RAWS;

pub fn builder(region: &mut Region, planet: &Planet, crash_site: Point) -> World {
    set_worldgen_status("Locating biome information");
    let biome_info = RAWS.read().biomes.areas[region.biome_raw_idx].clone();
    let biome = planet.biomes[region.biome_info_idx].clone();
    let mut pooled_water =
        vec![planet.water_height; REGION_WIDTH as usize * REGION_HEIGHT as usize];
    let mut rng = RandomNumberGenerator::seeded(
        planet.perlin_seed + planet_idx(crash_site.x as usize, crash_site.y as usize) as u64,
    );
    println!(
        "BUILDING FOR: {:?}",
        RAWS.read().biomes.areas[biome.biome_type].name
    );

    set_worldgen_status("Establishing ground altitude");
    let mut hm = heightmap::build_empty_heightmap();
    heightmap::build_heightmap_from_noise(
        &mut hm,
        crash_site,
        planet.perlin_seed,
        planet.lacunarity,
    );

    set_worldgen_status("Locating Sub-Biomes");
    heightmap::create_subregions(
        &mut rng,
        planet.landblocks[planet_idx(crash_site.x as usize, crash_site.y as usize)].variance,
        &mut hm,
        &mut pooled_water,
        &biome,
        planet.water_height,
    );

    set_worldgen_status("Adding water features");
    water_features::just_add_water(planet, region, &mut pooled_water, &mut hm, &mut rng);
    water_features::set_water_tiles(region, &pooled_water, planet.water_height as usize);

    set_worldgen_status("Stratifying");
    let region_strata = strata::build_strata(&mut rng, &mut hm, &biome_info, planet.perlin_seed);

    set_worldgen_status("Layer cake is yummy");
    strata::layer_cake(&hm, region, &region_strata, &mut rng);

    set_worldgen_status("Ramping");
    ramping::build_ramps(region);

    set_worldgen_status("Beaches");
    beaches::build_beaches(region);

    set_worldgen_status("Building an ECS");
    let mut world = World::default();
    game_components::add_game_components(&mut world, &hm, crash_site);

    set_worldgen_status("Seeding the lawn");
    plants::grow_plants(region, &mut world, biome.mean_temperature, &mut rng);

    set_worldgen_status("Crashing the ship");
    let ship_loc = Point::new(128, 128);
    let crash_z = buildings::build_escape_pod(region, &ship_loc, &mut world);

    set_worldgen_status("Trees");
    trees::plant_trees(region, &biome_info, &mut rng, &mut world);

    set_worldgen_status("Blight");
    set_worldgen_status("Trail of debris");
    debris::debris_trail(region, ship_loc, &mut world);

    set_worldgen_status("Settlers");
    settlers::spawn_settlers(
        &mut world,
        &mut rng,
        &ship_loc,
        crash_z,
        region.world_idx,
        planet.starting_settlers as usize,
        None,
    );

    set_worldgen_status("Features");

    set_worldgen_status("Making perfectly nice things into entities");
    map_components::transform_terrain_to_ecs(region, &mut world);

    set_worldgen_status("Looking for the map");
    flags::set_flags(region);

    world
}
