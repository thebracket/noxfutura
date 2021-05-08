use super::messaging;
use bengine::geometry::*;
use legion::world::SubWorld;
use legion::*;
use nox_components::*;
use nox_raws::*;
use nox_spatial::idxmap;

#[system]
#[read_component(Workshop)]
#[read_component(Tag)]
#[read_component(Building)]
#[read_component(ReactionJob)]
#[read_component(Position)]
#[read_component(IdentityTag)]
pub fn automatic_reactions(ecs: &SubWorld) {
    <(&Workshop, &Tag, &Building, &Position, &IdentityTag)>::query()
        .filter(!component::<Claimed>())
        .iter(ecs)
        .filter(|(ws, _tag, building, _pos, id)| {
            building.complete && ws.has_automatic_jobs && !has_autojobs(ecs, id.0)
        })
        .for_each(|(_ws, tag, _building, pos, id)| {
            let mut done = false;
            let rlock = RAWS.read();
            rlock
                .reactions
                .reactions
                .iter()
                .filter(|r| r.workshop == tag.0 && r.automatic)
                .for_each(|r| {
                    if done {
                        return;
                    }

                    // Are the inputs available?
                    if let Some(components) = select_components(ecs, &r.inputs, pos.as_point3()) {
                        done = true;
                        messaging::create_reaction_job(id.0, &r.name, &components);
                    }
                });
        });
}

fn select_components(
    ecs: &SubWorld,
    requires: &[ReactionItem],
    workshop_pos: Point3,
) -> Option<Vec<usize>> {
    let mut selected_components = Vec::new();
    for ri in requires.iter() {
        let mut available: Vec<(usize, f32)> = <(&Tag, &Position, &IdentityTag)>::query()
            .filter(!component::<Claimed>())
            .iter(ecs)
            .filter(|(tag, _pos, _id)| tag.0 == ri.tag)
            .map(|(_tag, pos, id)| (id.0, pos.effective_location_sw(ecs)))
            .map(|(id, pos)| {
                let (x, y, z) = idxmap(pos);
                (
                    id,
                    DistanceAlg::Pythagoras.distance3d(workshop_pos, Point3::new(x, y, z)),
                )
            })
            .collect();

        if available.len() < ri.qty as usize {
            // return None;
        } else {
            available.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            available
                .iter()
                .map(|(id, _)| *id)
                .take(ri.qty as usize)
                .for_each(|id| selected_components.push(id));
        }
    }
    Some(selected_components)
}

pub fn has_autojobs(ecs: &SubWorld, workshop_id: usize) -> bool {
    <&ReactionJob>::query()
        .iter(ecs)
        .filter(|rj| rj.workshop_id == workshop_id)
        .count()
        > 0
}
