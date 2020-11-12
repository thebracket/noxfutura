use legion::*;
use nox_components::*;
use nox_spatial::idxmap;
use super::super::{ models_moved, vox_moved };
use bengine::geometry::*;
use super::{ skill_check, REGION };

pub(crate) fn chop_tree(ecs: &mut World, actor_id: usize, tree_pos: usize) {
    println!("Chop tree");
    let mut to_remove = Vec::new();
    let mut to_spawn = Vec::new();

    // Identify a neighboring tree
    if let Some((tree_entity, tree_pos)) = locate_target(ecs, tree_pos) {
        let skill_check_result = skill_check(ecs, actor_id, Skill::Lumberjack, 12);
        if skill_check_result > 0 {
            // Damage the tree
            if let Some(mut te) = ecs.entry_mut(tree_entity) {
                if let Ok(health) = te.get_component_mut::<Health>() {
                    health.current -= skill_check_result;
                    // Destroy it if it went down
                    if health.current < 1 {
                        to_remove.push(tree_entity);
                        to_spawn.push(tree_pos);
                    }
                }
            }
        }
    }

    if !to_remove.is_empty() {
        let mut cb = legion::systems::CommandBuffer::new(ecs);
        to_remove.iter().for_each(|e| cb.remove(*e));
        cb.flush(ecs);
        models_moved();
    }
    if !to_spawn.is_empty() {
        let wood = nox_raws::get_material_by_tag("Wood").unwrap();
        for idx in to_spawn.iter() {
            let mut rlock = REGION.write();
            let (tx, ty, tz) = idxmap(*idx);
            nox_planet::spawn_item_on_ground(
                ecs,
                "wood_log",
                tx,
                ty,
                tz,
                &mut *rlock,
                wood,
            );
        }
        vox_moved();
    }
}

fn locate_target(ecs: &World, chop_pos: usize) -> Option<(Entity, usize)> {
    let (x, y, z) = idxmap(chop_pos);
    let chopper_pt = Point3::new(x, y, z);
    let mut trees : Vec<(Entity, f32, usize)> = <(Entity, &Tree, &Position)>::query()
        .iter(ecs)
        .filter(|(_entity, tree, _pos)| tree.chop)
        .map(|(entity, _tree, pos)| 
            (
                *entity,
                DistanceAlg::Pythagoras.distance3d(chopper_pt, pos.as_point3()),
                pos.get_idx()
            )
        )
        .collect();
    if trees.is_empty() {
        None
    } else {
        trees.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
        Some((trees[0].0, trees[0].2))
    }
}

