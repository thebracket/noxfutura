use legion::*;
use nox_components::*;
use nox_planet::{StairsType, TileType};
use nox_raws::MinesTo;
use super::REGION;
use nox_spatial::*;
use super::skill_check;
use bengine::geometry::*;

pub(crate) fn dig_at(ecs: &mut World, actor_id: usize, pos: usize) {
    let mining_designations : Vec<(usize, MiningMode)> = <(&MiningMode, &Position)>::query()
            .iter(ecs)
            .map(|(mm, pos)| (pos.get_idx(), *mm))
            .collect();

    println!("Looking for digging to perform at {}", pos);
    let (x, y, z) = idxmap(pos);
    let my_pos = Point3::new(x, y, z);
    let mut rlock = REGION.write();
    let mut nearby = mining_designations
        .iter()
        .map(|(idx, task)| {
            let (mx, my, mz) = idxmap(*idx);
            let distance =
                DistanceAlg::Pythagoras.distance3d(my_pos, Point3::new(mx, my, mz));
            (idx, task, distance)
        })
        .filter(|(_idx, _task, distance)| *distance < 1.2)
        .map(|(idx, task, distance)| (*idx, *task, distance))
        .collect::<Vec<(usize, MiningMode, f32)>>();

    println!("Nearby jobs: {:?}", nearby);

    if !nearby.is_empty() {
        if skill_check(ecs, actor_id, Skill::Mining, 12) > 0 {
            nearby.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
            println!("Applying: {:?}", nearby[0]);
            let (mine_id, task, _distance) = nearby[0];
            match task {
                MiningMode::Dig => {
                    println!("Changed tile");
                    rlock.tile_types[mine_id] = TileType::Floor;
                    super::super::tile_dirty(mine_id);
                    let material_idx = rlock.material_idx[mine_id];
                    let mat_info = nox_raws::RAWS.read().materials.materials[material_idx].clone();
                    for mt in mat_info.mines_to.iter() {
                        let (x,y,z) = idxmap(mine_id);
                        match mt {
                            MinesTo::Item{name} => {
                                nox_planet::spawn_item_on_ground(ecs,
                                    name,
                                    x, y, z,
                                    &mut rlock,
                                    material_idx
                                );
                            }
                            MinesTo::Ore{name} => {
                                nox_planet::spawn_item_on_ground(ecs,
                                    name,
                                    x, y, z,
                                    &mut rlock,
                                    material_idx
                                );
                            }
                        }
                    }
                }
                MiningMode::Channel => {
                    println!("Changed tile");
                    rlock.tile_types[mine_id] = TileType::Empty;
                    rlock.tile_types[mine_id - (REGION_WIDTH * REGION_HEIGHT)] =
                        TileType::Floor;
                    super::super::tile_dirty(mine_id);
                    super::super::tile_dirty(mine_id - (REGION_WIDTH * REGION_HEIGHT));
                }
                MiningMode::Up => {
                    println!("Changed tile");
                    rlock.tile_types[mine_id] = TileType::Stairs {
                        direction: StairsType::Up,
                    };
                    super::super::tile_dirty(mine_id);
                }
                MiningMode::Down => {
                    println!("Changed tile");
                    rlock.tile_types[mine_id] = TileType::Stairs {
                        direction: StairsType::Down,
                    };
                    super::super::tile_dirty(mine_id);
                }
                MiningMode::UpDown => {
                    println!("Changed tile");
                    rlock.tile_types[mine_id] = TileType::Stairs {
                        direction: StairsType::UpDown,
                    };
                    super::super::tile_dirty(mine_id);
                }
                _ => {}
            }
            println!("Undesignating");
            let to_remove : Vec<Entity> = <(Entity, &MiningMode, &Position)>::query()
                .iter(ecs)
                .filter(|(_, _, pos)| pos.get_idx() == mine_id)
                .map(|(e, _, _)| *e)
                .collect();
            to_remove.iter().for_each(|e| { ecs.remove(*e); });
            <&mut FieldOfView>::query()
                .iter_mut(ecs)
                .for_each(|f| f.is_dirty = true);
        }
    }
}