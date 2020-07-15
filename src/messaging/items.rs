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
                let treetag = IdentityTag(*tree_id);
                let tree_pos = <Read<Position>>::query()
                    .filter(tag_value(&treetag))
                    .iter_entities(ecs)
                    .map(|(e, pos)| (e, pos.get_idx()))
                    .nth(0)
                    .unwrap();
                let (tx, ty, tz) = idxmap(tree_pos.1);

                ecs.delete(tree_pos.0);

                let mut rlock = crate::systems::REGION.write();
                for _ in 0..crate::systems::RNG.lock().roll_dice(1, 6) {
                    nox_planet::spawn_item_on_ground(ecs, "wood_log", tx, ty, tz, &mut *rlock);
                }
                super::vox_moved();
            }

            WorldChange::EquipItem { id, tool_id } => {
                let itemtag = IdentityTag(*tool_id);
                <Write<Position>>::query()
                    .filter(tag_value(&itemtag))
                    .iter_mut(ecs)
                    .for_each(|mut pos| {
                        pos.to_carried(*id);
                    });
                super::vox_moved();
            }
        }
    }

    // Clean up
    wlock.clear();
}
