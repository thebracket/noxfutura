use super::{Region, Planet};
use crate::planet::set_worldgen_status;

pub fn builder(region : &mut Region, planet: &Planet) {
    set_worldgen_status("Locating biome information");
    let biome_info = crate::raws::RAWS.lock().biomes.areas[region.biome_raw_idx].clone();

    set_worldgen_status("Establishing ground altitude");
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