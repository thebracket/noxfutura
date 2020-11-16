use crate::Region;
use bengine::random::RandomNumberGenerator;
use legion::*;
use nox_spatial::mapidx;

pub fn spawn_building(
    ecs: &mut World,
    tag: &str,
    x: usize,
    y: usize,
    z: usize,
    region_idx: usize,
    complete: bool,
    required_components: &[usize],
) -> usize {
    nox_components::spawner::spawn_building(
        ecs,
        tag,
        mapidx(x, y, z),
        region_idx,
        complete,
        required_components,
    )
}

pub fn spawn_clothing_from_raws_worn(
    ecs: &mut World,
    tag: &str,
    wearer: usize,
    rng: &mut RandomNumberGenerator,
) -> Vec<(usize, (f32, f32, f32))> {
    nox_components::spawner::spawn_clothing_from_raws_worn(ecs, tag, wearer, rng)
}

pub fn spawn_item_on_ground(
    ecs: &mut World,
    tag: &str,
    x: usize,
    y: usize,
    z: usize,
    region: &mut Region,
    material: usize,
) {
    nox_components::spawner::spawn_item_on_ground(ecs, tag, x, y, z, region.world_idx, material);
}

pub fn spawn_item_in_container(ecs: &mut World, tag: &str, container: usize, material: usize) {
    nox_components::spawner::spawn_item_in_container(ecs, tag, container, material);
}

pub fn spawn_item_worn(ecs: &mut World, tag: &str, wearer: usize, material: usize) {
    nox_components::spawner::spawn_item_worn(ecs, tag, wearer, material);
}

pub fn spawn_item_carried(ecs: &mut World, tag: &str, wearer: usize, material: usize) {
    nox_components::spawner::spawn_item_carried(ecs, tag, wearer, material);
}

pub fn spawn_plant(
    ecs: &mut World,
    tag: &str,
    x: usize,
    y: usize,
    z: usize,
    region_idx: usize,
    size: f32,
) {
    nox_components::spawner::spawn_plant(ecs, tag, x, y, z, region_idx, size)
}

pub fn spawn_tree(
    ecs: &mut World,
    x: usize,
    y: usize,
    z: usize,
    region_idx: usize,
    model_id: usize,
    size: f32,
) {
    nox_components::spawner::spawn_tree(ecs, x, y, z, region_idx, model_id, size)
}
