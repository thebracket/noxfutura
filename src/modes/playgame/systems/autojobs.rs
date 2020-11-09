use legion::*;
use nox_components::*;
use nox_raws::*;
use bengine::geometry::*;
use super::REGION;

pub fn autojobs(ecs: &World) {
    let rlock = RAWS.read();
    <(&Building, &Workshop, &Tag, &IdentityTag, &Position)>::query()
        .iter(ecs)
        .for_each(|(building, workshop, tag, id, pos)| {
            if building.complete && workshop.has_automatic_jobs {
                let bpoint = pos.as_point3();
                rlock.reactions.reactions
                    .iter()
                    .enumerate()
                    .filter(|(_,r)| r.workshop == tag.0 && r.automatic)
                    .filter(|(rid, _)| !REGION.write().jobs_board.autojob_registered(id.0, *rid, pos.effective_location(ecs)) )
                    .for_each(|(rid,reaction)| {

                        let mut components = Vec::new();
                        for input in reaction.inputs.iter() {

                            let mut closest : Vec<(usize, f32, usize, usize)> = <(&Tag, &Position, &IdentityTag, &Material)>::query()
                                .filter(component::<Item>())
                                .iter(ecs)
                                .filter(|(ctag, _, _, _)| ctag.0 == input.tag)
                                .map(|(_ctag, cpos, cid, mat)| {
                                    (
                                        cid.0,
                                        DistanceAlg::Pythagoras.distance3d(cpos.as_point3(), bpoint),
                                        cpos.effective_location(ecs),
                                        mat.0
                                    )
                                })
                                .collect();
                            closest.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                            for i in 0 .. input.qty as usize {
                                components.push((closest[i].0, closest[i].2, closest[i].3));
                            }
                        }
                        println!("Selected Components: {:?}", components);
                        REGION.write().jobs_board.register_autojob(id.0, rid, pos.effective_location(ecs), &components);                            }
                );
            }
        }
    );
}