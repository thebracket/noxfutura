use super::super::GameStateResource;
use super::{JobStep, MOVER_LIST};
use crate::components::*;
use crate::modes::playgame::systems::REGION;
use crate::spatial::*;
use legion::*;

pub fn apply_jobs_queue(ecs: &mut World, resources: &mut Resources) {
    let mut vox_moved = false;
    let mut models_moved = false;
    MOVER_LIST.lock().clear();
    loop {
        let js = super::JOBS_QUEUE.lock().pop_front();
        if let Some(mut js) = js {
            match js {
                JobStep::VoxMoved => vox_moved = true,
                JobStep::ModelsMoved => models_moved = true,
                _ => apply(ecs, &mut js),
            }
        } else {
            break;
        }
    }
    movers(ecs, resources);

    if vox_moved || models_moved {
        let mut gs = resources.get_mut::<GameStateResource>();
        let gsr = gs.as_mut().unwrap();
        if vox_moved {
            gsr.vox_moved = true;
        }
        if models_moved {
            gsr.models_moved = true;
        }
    }
}

fn movers(ecs: &mut World, resources: &mut Resources) {
    let movers = MOVER_LIST.lock();
    if !movers.is_empty() {
        if let Some(mut gs) = resources.get_mut::<GameStateResource>() {
            gs.vox_moved = true;
        }
        <(Read<IdentityTag>, Write<Position>)>::query()
            .iter_mut(ecs)
            .filter(|(idt, _)| movers.contains_key(&idt.0))
            .for_each(|(id, pos)| {
                if let Some(destination) = movers.get(&id.0) {
                    pos.set_tile_loc(destination);
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
            <(&mut MyTurn, &IdentityTag)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *id)
                .for_each(|(turn, _)| match &mut turn.job {
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
                    JobType::ConstructBuilding { step, .. } => {
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
                });
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
            REGION
                .write()
                .jobs_board
                .relinquish_claim(*tool_id, *tool_pos);
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
        JobStep::TreeChop { id: _, tree_id } => {
            println!("Chop tree");
            let mut rlock = REGION.write();

            let mut to_remove = Vec::new();
            let mut to_spawn = Vec::new();
            <(Entity, &Position, &IdentityTag)>::query()
                .filter(component::<Tree>())
                .iter(ecs)
                .filter(|(_, _, id)| id.0 == *tree_id)
                .for_each(|(entity, pos, _)| {
                    to_remove.push(*entity);
                    to_spawn.push(pos.get_idx());
                });
            if !to_remove.is_empty() {
                let mut cb = legion::systems::CommandBuffer::new(ecs);
                to_remove.iter().for_each(|e| cb.remove(*e));
                cb.flush(ecs);
                super::models_moved();
            }
            if !to_spawn.is_empty() {
                let wood = crate::raws::get_material_by_tag("Wood").unwrap();
                for idx in to_spawn.iter() {
                    let (tx, ty, tz) = idxmap(*idx);
                    crate::planet::spawn_item_on_ground(
                        ecs,
                        "wood_log",
                        tx,
                        ty,
                        tz,
                        &mut *rlock,
                        wood,
                    );
                }
                super::vox_moved();
            }
        }
        _ => {}
    }
}
