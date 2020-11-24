use super::super::GameStateResource;
use super::{JobStep, MOVER_LIST};
use crate::modes::playgame::systems::REGION;
use bengine::{geometry::DistanceAlg, Palette};
use legion::{systems::CommandBuffer, *};
use nox_components::*;
use nox_spatial::*;
mod job_designations;
use job_designations::*;
mod pathing;
use pathing::*;
mod lumber;
use lumber::*;
mod gamesystem;
use gamesystem::*;
mod mining;
use mining::*;

pub fn apply_jobs_queue(ecs: &mut World, resources: &mut Resources, palette: &Palette) {
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
                _ => apply(ecs, &mut js, palette),
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

fn apply(ecs: &mut World, js: &mut JobStep, palette: &Palette) {
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
                    //REGION.write().jobs_board.restore_job(&turn.job);
                    turn.job = JobType::None;
                });
        }
        JobStep::JobConcluded { id } => {
            println!("Job finished");
            <(&mut MyTurn, &IdentityTag)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *id)
                .for_each(|(mut turn, _)| {
                    match turn.job {
                        JobType::Haul { item_id, .. } => {
                            super::remove_haul_tag(item_id);
                        }
                        _ => {}
                    }
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
                    println!("{:?}", pos);
                    println!("Item dropped");
                    println!("{:?}", pos);
                    pos.to_ground(*location);
                    println!("{:?}", pos);
                });
            super::vox_moved();
        }
        JobStep::RelinquishClaim { .. } => {
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
        JobStep::GetItem { id, item_id } => {
            <(&mut Position, &IdentityTag)>::query()
                .iter_mut(ecs)
                .filter(|(_, idt)| idt.0 == *item_id)
                .for_each(|(pos, _)| {
                    println!("Getting item {:?}", pos);
                    pos.to_carried(*id);
                    println!("Got item {:?}", pos);
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
            chop_tree(ecs, *id, *tree_pos, palette);
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
            let mut cmds = CommandBuffer::new(ecs);
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
                    if let Ok(bp) = en.get_component::<Blueprint>() {
                        for cid in bp.required_items.iter() {
                            super::super::delete_item(*cid);
                        }
                        cmds.remove_component::<Blueprint>(e);
                    }
                }
            }
            cmds.flush(ecs);
        }
        JobStep::DigAt { pos, id } => {
            dig_at(ecs, *id, *pos, palette);
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
        JobStep::SpawnItem {
            pos,
            tag,
            qty,
            material,
        } => {
            let (x, y, z) = idxmap(*pos);
            for _ in 0..*qty {
                nox_planet::spawn_item_on_ground(
                    ecs,
                    tag,
                    x,
                    y,
                    z,
                    &mut REGION.write(),
                    *material,
                    Some(palette),
                );
            }
        }
        JobStep::HaulInProgress { id, by } => {
            <(&IdentityTag, &mut RequestHaul)>::query()
                .iter_mut(ecs)
                .filter(|(hid, _rh)| hid.0 == *id)
                .for_each(|(_id, rh)| {
                    rh.in_progress = Some(*by);
                });
        }
        JobStep::ReactionInProgress { id, by } => {
            <(&IdentityTag, &mut ReactionJob)>::query()
                .iter_mut(ecs)
                .filter(|(hid, _rh)| hid.0 == *id)
                .for_each(|(_id, rh)| {
                    rh.in_progress = Some(*by);
                });
        }
        JobStep::RemoveHaulTag { id } => {
            let mut cmds = CommandBuffer::new(ecs);
            <(Entity, &IdentityTag)>::query()
                .iter(ecs)
                .filter(|(_, hid)| hid.0 == *id)
                .for_each(|(e, _)| {
                    cmds.remove_component::<RequestHaul>(*e);
                });
            cmds.flush(ecs);
        }
        JobStep::UpdateBlueprint { item_id } => {
            let mut ready_blueprints = Vec::new();
            <(&IdentityTag, &Claimed)>::query()
                .iter(ecs)
                .filter(|(id, _)| id.0 == *item_id)
                .for_each(|(_id, claim)| {
                    // claim.by = what claimed it
                    <(Entity, &Blueprint, &IdentityTag, &Position)>::query()
                        .iter(ecs)
                        .filter(|(_, _, cid, _)| cid.0 == claim.by)
                        .for_each(|(e, bp, _, bpos)| {
                            let blueprint_point = bpos.as_point3();
                            // Check all components
                            let mut is_ready = true;
                            for comp_id in bp.required_items.iter() {
                                let comp_pos = <(&IdentityTag, &Position)>::query()
                                    .iter(ecs)
                                    .filter(|(comp_id_tag, _)| comp_id_tag.0 == *comp_id)
                                    .map(|(_, comp_pos)| comp_pos.as_point3())
                                    .nth(0)
                                    .unwrap();
                                let distance =
                                    DistanceAlg::Pythagoras.distance3d(blueprint_point, comp_pos);
                                if distance > 1.4 {
                                    is_ready = false;
                                }
                            }
                            if is_ready {
                                ready_blueprints.push(*e);
                            }
                        });
                });
            for rb in ready_blueprints.iter() {
                if let Ok(mut er) = ecs.entry_mut(*rb) {
                    if let Ok(bp) = er.get_component_mut::<Blueprint>() {
                        println!("Blueprint is ready");
                        bp.ready_to_build = true;
                    }
                }
            }
        }
        JobStep::CreateReactionJob {
            workshop_id,
            reaction_tag,
            components,
        } => {
            let mut cmds = CommandBuffer::new(ecs);
            println!("Made a reaction job of type: {}", reaction_tag);

            let building_pos = <(&Position, &IdentityTag)>::query()
                .iter(ecs)
                .filter(|(_, wid)| wid.0 == *workshop_id)
                .map(|(bpos, _)| bpos.get_idx())
                .nth(0)
                .unwrap();

            println!("Reaction located at {:?}", idxmap(building_pos));

            let new_id = IdentityTag::new();
            let job_id = new_id.0;

            let ready_to_build = components.is_empty();
            ecs.push((
                new_id,
                ReactionJob {
                    workshop_id: *workshop_id,
                    reaction_tag: reaction_tag.to_string(),
                    in_progress: None,
                },
                Blueprint {
                    ready_to_build,
                    required_items: components.clone(),
                },
                Position::with_tile_idx(building_pos, REGION.read().world_idx, (1, 1, 1)),
            ));

            components.iter().for_each(|cid| {
                <(Entity, &IdentityTag)>::query()
                    .iter(ecs)
                    .filter(|(_, id)| id.0 == *cid)
                    .for_each(|(e, _)| {
                        cmds.add_component(*e, Claimed { by: job_id });
                        cmds.add_component(
                            *e,
                            RequestHaul {
                                in_progress: None,
                                destination: building_pos,
                            },
                        );
                    });
            });

            cmds.flush(ecs);
        }
        JobStep::PerformReaction { reaction_id } => {
            // Find the reaction
            let (reaction_entity, reaction_job, blueprint, rpos) =
                <(Entity, &ReactionJob, &IdentityTag, &Blueprint, &Position)>::query()
                    .iter(ecs)
                    .filter(|(_e, _rj, id, _bp, _pos)| id.0 == *reaction_id)
                    .map(|(e, rj, _id, bp, pos)| (*e, rj, bp, pos.get_idx()))
                    .nth(0)
                    .unwrap();

            // Find material for the first component
            let material = if blueprint.required_items.is_empty() {
                0
            } else {
                <(&Material, &IdentityTag)>::query()
                    .iter(ecs)
                    .filter(|(_, mid)| mid.0 == blueprint.required_items[0])
                    .map(|(m, _)| m.0)
                    .nth(0)
                    .unwrap_or(0)
            };

            // Delete all components
            for c in blueprint.required_items.iter() {
                super::delete_item(*c);
            }

            // Spawn the result
            use nox_raws::RAWS;
            if let Some(raw) = RAWS
                .read()
                .reactions
                .reaction_by_tag(&reaction_job.reaction_tag)
            {
                println!("Spawning {:?}", raw.outputs);
                for o in raw.outputs.iter() {
                    super::spawn_item(&rpos, &o.tag, &o.qty, material);
                }
            } else {
                println!("Reaction {} not found", reaction_job.reaction_tag);
            }

            // Delete the reaction entity
            ecs.remove(reaction_entity);

            // Notify of changes
            super::vox_moved();
        }
        _ => {}
    }
}
