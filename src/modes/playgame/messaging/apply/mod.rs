use super::super::GameStateResource;
use super::{JobStep, MOVER_LIST};
use crate::modes::playgame::systems::REGION;
use legion::*;
use nox_components::*;
use nox_planet::{MiningMode, StairsType, TileType};
use nox_raws::MinesTo;
use nox_spatial::*;
mod job_designations;
use job_designations::*;
mod pathing;
use pathing::*;
mod lumber;
use lumber::*;
mod gamesystem;
use gamesystem::*;

pub fn apply_jobs_queue(ecs: &mut World, resources: &mut Resources) {
    let mut vox_moved = false;
    let mut models_moved = false;
    let mut lights_changed = false;
    let mut tiles_dirty = Vec::new();
    MOVER_LIST.lock().clear();
    loop {
        let js = super::JOBS_QUEUE.lock().pop_front();
        if let Some(mut js) = js {
            match js {
                JobStep::VoxMoved => vox_moved = true,
                JobStep::ModelsMoved => models_moved = true,
                JobStep::LightsChanged => lights_changed = true,
                JobStep::TileDirty { pos } => {
                    tiles_dirty.push(pos);
                    vox_moved = true;
                    lights_changed = true;
                }
                _ => apply(ecs, &mut js),
            }
        } else {
            break;
        }
    }
    movers(ecs, resources);

    if vox_moved || models_moved || lights_changed {
        let mut gs = resources.get_mut::<GameStateResource>();
        let gsr = gs.as_mut().unwrap();
        if vox_moved {
            gsr.vox_moved = true;
        }
        if models_moved {
            gsr.models_moved = true;
        }
        if lights_changed {
            gsr.lights_changed = true;
        }
        if !tiles_dirty.is_empty() {
            gsr.dirty_tiles.extend_from_slice(&tiles_dirty);
        }
    }
}

fn movers(ecs: &mut World, resources: &mut Resources) {
    let movers = MOVER_LIST.lock();
    if !movers.is_empty() {
        if let Some(mut gs) = resources.get_mut::<GameStateResource>() {
            gs.vox_moved = true;
        }
        <(&IdentityTag, &mut Position, &mut FieldOfView)>::query()
            .iter_mut(ecs)
            .filter(|(idt, _, _)| movers.contains_key(&idt.0))
            .for_each(|(id, pos, fov)| {
                if let Some(destination) = movers.get(&id.0) {
                    pos.set_tile_loc(destination);
                    fov.is_dirty = true;
                }
            });
    }
}

fn apply(ecs: &mut World, js: &mut JobStep) {
    match js {
        JobStep::EntityMoved { id, end } => {
            MOVER_LIST
                .lock()
                .insert(*id, (end.x as usize, end.y as usize, end.z as usize));
        }
        JobStep::JobChanged { id, new_job } => {
            <(&mut MyTurn, &IdentityTag)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *id)
                .for_each(|(mut turn, _)| {
                    turn.job = new_job.clone();
                });
        }
        JobStep::JobCancelled { id } => {
            <(&mut MyTurn, &IdentityTag)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *id)
                .for_each(|(mut turn, _)| {
                    REGION.write().jobs_board.restore_job(&turn.job);
                    turn.job = JobType::None;
                });
        }
        JobStep::JobConcluded { id } => {
            println!("Job finished");
            <(&mut MyTurn, &IdentityTag)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *id)
                .for_each(|(mut turn, _)| {
                    turn.job = JobType::None;
                });
        }
        JobStep::FollowJobPath { id } => {
            follow_path(ecs, *id);
        }
        JobStep::DropItem { id, location } => {
            println!("Dropping item #{}, at {}", id, location);
            <(&mut Position, &IdentityTag)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *id)
                .for_each(|(pos, _)| {
                    println!("Item dropped");
                    println!("{:?}", pos);
                    pos.to_ground(*location);
                    println!("{:?}", pos);
                });
        }
        JobStep::RelinquishClaim { tool_id, tool_pos } => {
            /*REGION
                .write()
                .jobs_board
                .relinquish_claim(*tool_id, *tool_pos);*/
        }
        JobStep::EquipItem { id, tool_id } => {
            <(&mut Position, &IdentityTag)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *tool_id)
                .for_each(|(pos, _)| {
                    pos.to_carried(*id);
                });
            super::vox_moved();
        }
        JobStep::DeleteItem { id } => {
            let i = <(Entity, &Position, &IdentityTag)>::query()
                .iter(ecs)
                .filter(|(_, _, idt)| idt.0 == *id)
                .map(|(e, _, _)| *e)
                .nth(0);
            if let Some(i) = i {
                ecs.remove(i);
            }
            super::vox_moved();
        }
        JobStep::TreeChop { id, tree_pos } => {
            chop_tree(ecs, *id, *tree_pos);
        }
        JobStep::DeleteBuilding { building_id } => {
            let i = <(Entity, Read<Position>, Read<IdentityTag>)>::query()
                .iter(ecs)
                .filter(|(_, _, idt)| idt.0 == *building_id)
                .map(|(e, _, _)| *e)
                .nth(0);
            if let Some(i) = i {
                ecs.remove(i);
            }
            super::vox_moved();
            super::lights_changed();
        }

        JobStep::FinishBuilding { building_id } => {
            println!("Finish building called for id {}", building_id);
            let e = <(Entity, Read<Position>, Read<IdentityTag>, Read<Building>)>::query()
                .iter(ecs)
                .filter(|(_, _, idt, _)| idt.0 == *building_id)
                .map(|(e, _, _, _)| *e)
                .nth(0);

            if let Some(e) = e {
                println!("Entity located");
                if let Some(mut en) = ecs.entry(e) {
                    println!("Entry obtained");
                    if let Ok(b) = en.get_component_mut::<Building>() {
                        println!("Building updated");
                        b.complete = true;
                    }
                    if let Ok(mut l) = en.get_component_mut::<Light>() {
                        l.enabled = true;
                        super::lights_changed();
                    }
                }
            }
        }
        JobStep::DigAt { pos, .. } => {
            use bengine::geometry::*;
            println!("Looking for digging to perform at {}", pos);
            let (x, y, z) = idxmap(*pos);
            let my_pos = Point3::new(x, y, z);
            let mut rlock = REGION.write();
            let mut nearby = rlock
                .jobs_board
                .mining_designations
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
                rlock.jobs_board.mining_designations.remove(&mine_id);
                <&mut FieldOfView>::query()
                    .iter_mut(ecs)
                    .for_each(|f| f.is_dirty = true);
            }
        }
        JobStep::BecomeMiner { id } => {
            become_miner(ecs, *id);
        }
        JobStep::BecomeLumberjack { id } => {
            become_lumberjack(ecs, *id);
        }
        JobStep::FireMiner { id } => {
            fire_miner(ecs, *id);
        }
        JobStep::FireLumberjack { id } => {
            fire_lumberjack(ecs, *id);
        }
        JobStep::SpawnItem { pos, tag, qty, material } => {
            let (x, y, z) = idxmap(*pos);
            nox_planet::spawn_item_on_ground(ecs, tag, x, y, z, &mut REGION.write(), *material)
        }
        _ => {}
    }
}
