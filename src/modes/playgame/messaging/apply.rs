use super::{ MOVER_LIST, JobStep };
use legion::*;
use crate::components::*;
use super::super::GameStateResource;

pub fn apply_jobs_queue(ecs: &mut World, resources: &mut Resources) {
    MOVER_LIST.lock().clear();
    loop {
        let js = super::JOBS_QUEUE.lock().pop_front();
        if let Some(mut js) = js {
            apply(ecs, &mut js);
        } else {
            break;
        }
    }
    movers(ecs, resources);
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
        _ => {}
    }
}