use crate::systems::REGION;
use bracket_geometry::prelude::Point3;
use legion::*;
use nox_components::*;
use nox_spatial::idxmap;
use super::MOVER_LIST;

pub enum JobStep {
    EntityMoved { id: usize, end: Point3 },
    JobChanged { id: usize, new_job: JobType },
    JobCancelled { id: usize },
    JobConcluded { id: usize },
    FollowJobPath { id: usize },
    DropItem { id: usize, location: usize },
    RelinquishClaim { tool_id: usize, tool_pos: usize },
    EquipItem { id: usize, tool_id: usize },
    TreeChop { id: usize, tree_id: usize },
    DeleteItem { id: usize },
    FinishBuilding { building_id: usize },
    DeleteBuilding { building_id: usize }
}

pub fn apply_jobs_queue(ecs: &mut World) {
    MOVER_LIST.lock().clear();
    loop {
        let js = super::JOBS_QUEUE.lock().pop_front();
        if let Some(mut js) = js {
            apply(ecs, &mut js);
        } else {
            break;
        }
    }
}

fn apply(ecs: &mut World, js: &mut JobStep) {
    match js {
        JobStep::EntityMoved { id, end } => {
            MOVER_LIST.lock().insert(*id, (end.x as usize, end.y as usize, end.z as usize));
        }
        JobStep::JobChanged { id, new_job } => {
            let idtag = IdentityTag(*id);
            <(Write<MyTurn>, Read<IdentityTag>)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *id)
                .for_each(|(mut turn, _)| {
                    turn.job = new_job.clone();
                });
        }
        JobStep::JobCancelled { id } => {
            let idtag = IdentityTag(*id);
            <(Write<MyTurn>, Read<IdentityTag>)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *id)
                .for_each(|(mut turn, _)| {
                    REGION.write().jobs_board.restore_job(&turn.job);
                    turn.job = JobType::None;
                });
        }
        JobStep::JobConcluded { id } => {
            println!("Job finished");
            let idtag = IdentityTag(*id);
            <(Write<MyTurn>, Read<IdentityTag>)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *id)
                .for_each(|(mut turn, _)| {
                    match &turn.job {
                        JobType::FellTree {
                            tree_id,
                            tree_pos: _,
                            step: _,
                            tool_id: _,
                        } => {
                            // Un-designate the tree
                            let mut rlock = REGION.write();
                            rlock.jobs_board.remove_tree(&tree_id);
                        }
                        _ => {}
                    }
                    turn.job = JobType::None;
                });
        }
        JobStep::FollowJobPath { id } => {
            let idtag = IdentityTag(*id);
            <(Write<MyTurn>, Read<IdentityTag>)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *id)
                .for_each(|(mut turn, _)| match &mut turn.job {
                    JobType::FellTree { step, .. } => {
                        let path = match step {
                            LumberjackSteps::TravelToAxe { path, .. } => Some(path),
                            LumberjackSteps::TravelToTree { path } => Some(path),
                            _ => None,
                        };
                        if let Some(path) = path {
                            let destination = path[0];
                            path.remove(0);
                            let (x, y, z) = idxmap(destination);
                            MOVER_LIST.lock().insert(*id, (x, y, z));
                        }
                    }
                    JobType::ConstructBuilding {step, ..} => {
                        let path = match step {
                            BuildingSteps::TravelToComponent { path, .. } => Some(path),
                            BuildingSteps::TravelToTBuilding { path, .. } => Some(path),
                            _ => None,
                        };
                        if let Some(path) = path {
                            let destination = path[0];
                            path.remove(0);
                            let (x, y, z) = idxmap(destination);
                            MOVER_LIST.lock().insert(*id, (x, y, z));
                        }
                    }
                    _ => {}
                }
            );
        }
        JobStep::DropItem { id, location } => {
            let idtag = IdentityTag(*id);
            <(Write<Position>, Read<IdentityTag>)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *id)
                .for_each(|(mut pos, _)| {
                    pos.to_ground(*location);
                });
        }
        JobStep::RelinquishClaim { tool_id, tool_pos } => {
            REGION.write().jobs_board.relinquish_claim(*tool_id, *tool_pos);
        }
        JobStep::TreeChop { id: _, tree_id } => {
            println!("Chop tree");
            use nox_planet::TileType;
            let mut rlock = crate::systems::REGION.write();
            let mut tree_idx = std::usize::MAX;

            let tid = tree_id;
            rlock.tile_types.iter_mut().enumerate()
                .filter(|(_, tt)| {
                    match tt {
                        TileType::TreeTrunk{tree_id} => *tree_id == *tid,
                        TileType::TreeFoliage{tree_id} => *tree_id == *tid,
                        _ => false
                    }
                })
                .for_each(|(idx, tt)| {
                    tree_idx = usize::min(tree_idx, idx);
                    *tt = TileType::Empty;
                    super::geometry_changed(idx);
                })
            ;

            for _ in 0..crate::systems::RNG.lock().roll_dice(1, 6) {
                let (tx,ty,tz) = idxmap(tree_idx);
                nox_planet::spawn_item_on_ground(ecs, "wood_log", tx, ty, tz, &mut *rlock);
            }
            super::vox_moved();
        }

        JobStep::EquipItem { id, tool_id } => {
            let itemtag = IdentityTag(*tool_id);
            <(Write<Position>, Read<IdentityTag>)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *id)
                .for_each(|(mut pos, _)| {
                    pos.to_carried(*id);
                });
            super::vox_moved();
        }

        JobStep::DeleteItem { id} => {
            let itemtag = IdentityTag(*id);
            let i = <(Entity, Read<Position>, Read<IdentityTag>)>::query()
                .iter(ecs)
                .filter(|(_, _, idt)| idt.0 == *id)
                .map(|(e, _, _)| e)
                .nth(0);
            if let Some(i) = i {
                ecs.remove(*i);
            }
            super::vox_moved();
        }

        JobStep::DeleteBuilding { building_id } => {
            let itemtag = IdentityTag(*building_id);
            let i = <(Entity, Read<Position>, Read<IdentityTag>)>::query()
                .iter(ecs)
                .filter(|(_, _, idt)| idt.0 == *building_id)
                .map(|(e, _, _)| e)
                .nth(0);
            if let Some(i) = i {
                ecs.remove(*i);
            }
            super::vox_moved();
            super::lights_changed();
        }

        JobStep::FinishBuilding { building_id } => {
            println!("Finish building called for id {}", building_id);
            let idtag = IdentityTag(*building_id);
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
    };

    let movers = MOVER_LIST.lock();
    if !movers.is_empty() {
        super::vox_moved();
        <(Read<IdentityTag>, Write<Position>)>::query()
            .iter_mut(ecs)
            .filter(|(idt, _)| movers.contains_key(&idt.0))
            .for_each(|(id, mut pos)| {
                if let Some(destination) = movers.get(&id.0) {
                    pos.set_tile_loc(destination);
                }
            });
    }
}
