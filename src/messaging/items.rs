use legion::prelude::*;
use nox_components::*;
use nox_spatial::idxmap;

pub enum WorldChange {
    EquipItem { id: usize, tool_id: usize },
    TreeChop { id: usize, tree_id: usize },
}

pub fn apply_world_queue(ecs: &mut World) {
    let mut wlock = super::WORLD_QUEUE.lock();

    for w in wlock.iter() {
        match w {
            WorldChange::TreeChop { id: _, tree_id } => {
                println!("Chop tree");
                // TODO: Remove the tree from the region data
                let tree_entity = <(Read<Identity>,Read<Position>)>::query().filter(tag::<Tree>())
                    .iter_entities(ecs)
                    .filter(|(_entity, (tid, _))| tid.id == *tree_id)
                    .map(|(e, (_,pos))| (e, pos.get_idx()))
                    .nth(0)
                    .unwrap()
                ;
                //ecs.delete(tree_entity.0); // Crashes

                let mut rlock = crate::systems::REGION.write();
                let (tx, ty, tz) = idxmap(tree_entity.1);
                for _ in 0 .. crate::systems::RNG.lock().roll_dice(1, 6) {
                    nox_planet::spawn_item_on_ground(ecs, "wood_log", tx, ty, tz, &mut *rlock);
                }
                super::vox_moved();
            }

            WorldChange::EquipItem { id, tool_id } => {
                <(Read<Identity>, Write<Position>)>::query()
                    .iter_mut(ecs)
                    .filter(|(tid, _)| tid.id == *tool_id)
                    .for_each(|(_, mut pos)| {
                        pos.to_carried(*id);
                    }
                );
                super::vox_moved();
            }
        }
    }

    // Clean up
    wlock.clear();
}
