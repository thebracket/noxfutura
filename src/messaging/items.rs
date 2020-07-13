use legion::prelude::*;
use nox_components::*;

pub enum WorldChange{
    EquipItem{id: usize, tool_id: usize},
    TreeChop{id: usize, tree_id: usize}
}

pub fn apply_world_queue(ecs: &mut World) {
    let mut wlock = super::WORLD_QUEUE.lock();

    for w in wlock.iter() {
        match w {
            WorldChange::TreeChop {id, tree_id} => {
                /*let mut commands = legion::prelude::CommandBuffer::new(ecs);
                let mut tree_pos = Position{x:0, y:0, z:0};
                <(Read<Identity>,Read<Position>)>::query()
                    .iter_entities(ecs)
                    .filter(|(_entity, (tid, _))| tid.id == *tree_id)
                    .for_each(|(entity, (_, pos))| {
                        tree_pos = *pos;
                        commands.delete(entity);
                    }
                );
                commands.write(ecs);
                let mut rlock = crate::systems::REGION.write();
                for _ in 0 .. crate::systems::RNG.lock().roll_dice(1, 6) {
                    nox_planet::spawn_item_on_ground(ecs, "wood_log", tree_pos.x, tree_pos.y, tree_pos.z, &mut *rlock);
                }*/
    }
            WorldChange::EquipItem {id, tool_id} => {
                /*let mut commands = legion::prelude::CommandBuffer::new(ecs);
                <Read<Identity>>::query()
                    .iter_entities(ecs)
                    .filter(|(_entity, tid)| tid.id == *tool_id)
                    .for_each(|(entity, _)| {
                        commands.add_component(entity, ItemCarried{ wearer: *id });
                        commands.remove_component::<Position>(entity);
                    }
                );
                commands.write(ecs);*/
            }
        }
    }

    // Clean up
    wlock.clear();
}