use crate::Region;
use bracket_random::prelude::RandomNumberGenerator;
use legion::*;
use nox_components::*;
use nox_spatial::mapidx;

fn add_tool_info(ecs: &World, item_id: usize, region: &mut Region, claimed: Option<usize>) {
    <(Read<Tool>, Read<IdentityTag>)>::query()
        .iter(ecs)
        .filter(|(_, id)| id.0 == item_id)
        .for_each(|(tool, _)| {
            let mut effective_location = 0;

            if claimed.is_none() {
                <(Read<Position>, Read<IdentityTag>)>::query()
                    .iter(ecs)
                    .filter(|(_, id)| id.0 == item_id)
                    .for_each(|(pos, _)| effective_location = pos.effective_location(ecs));
            }

            println!(
                "Adding tool to list. {:?}, at {}",
                tool.usage, effective_location
            );
            region
                .jobs_board
                .add_tool(item_id, claimed, tool.usage, effective_location);
        });
}

pub fn spawn_building(
    ecs: &mut World,
    tag: &str,
    x: usize,
    y: usize,
    z: usize,
    region_idx: usize,
    complete: bool,
) -> usize {
    nox_components::spawner::spawn_building(ecs, tag, mapidx(x, y, z), region_idx, complete)
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
) {
    if let Some(id) =
        nox_components::spawner::spawn_item_on_ground(ecs, tag, x, y, z, region.world_idx)
    {
        add_tool_info(ecs, id, region, None);
    }
}

pub fn spawn_item_in_container(ecs: &mut World, tag: &str, container: usize, region: &mut Region) {
    println!("Container spawn");
    if let Some(id) = nox_components::spawner::spawn_item_in_container(ecs, tag, container) {
        add_tool_info(ecs, id, region, None);
    }
}

pub fn spawn_item_worn(ecs: &mut World, tag: &str, wearer: usize, region: &mut Region) {
    if let Some(id) = nox_components::spawner::spawn_item_worn(ecs, tag, wearer) {
        add_tool_info(ecs, id, region, Some(wearer));
    }
}

pub fn spawn_item_carried(ecs: &mut World, tag: &str, wearer: usize, region: &mut Region) {
    if let Some(id) = nox_components::spawner::spawn_item_carried(ecs, tag, wearer) {
        add_tool_info(ecs, id, region, Some(wearer));
    }
}

pub fn spawn_plant(ecs: &mut World, tag: &str, x: usize, y: usize, z: usize, region_idx: usize) {
    nox_components::spawner::spawn_plant(ecs, tag, x, y, z, region_idx)
}

pub fn spawn_tree(ecs: &mut World, x: usize, y: usize, z: usize, region_idx: usize) {
    nox_components::spawner::spawn_tree(ecs, x, y, z, region_idx)
}
