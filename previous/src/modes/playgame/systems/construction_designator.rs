use legion::*;
use legion::{systems::CommandBuffer, world::SubWorld};
use nox_components::*;
use nox_planet::ConstructionMap;
use std::collections::HashSet;

#[system]
#[read_component(Construction)]
#[read_component(Item)]
#[read_component(Position)]
#[read_component(Claimed)]
#[read_component(Blueprint)]
#[read_component(Tag)]
#[read_component(IdentityTag)]
pub fn construction_designator(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] cmap: &mut ConstructionMap,
) {
    let mut available_blocks = <(&Item, &Tag)>::query()
        .filter(!component::<Claimed>())
        .iter(ecs)
        .filter(|(_i, t)| t.0 == "block")
        .count();

    if available_blocks < 1 {
        return;
    }

    let mut used_blocks = HashSet::new();

    <(&Construction, &Position, &IdentityTag, Entity)>::query()
        .filter(!component::<Blueprint>())
        .iter(ecs)
        .for_each(|(_, pos, build_id, e)| {
            println!("Found a matching construction job");
            if available_blocks > 0 {
                let idx = pos.get_idx();
                if cmap.dijkstra[idx] < f32::MAX {
                    // The building site is accessible
                    // Select components
                    let mut blocks: Vec<(usize, f32, Entity)> =
                        <(&Tag, &IdentityTag, &Position, Entity)>::query()
                            .filter(!component::<Claimed>())
                            .iter(ecs)
                            .filter(|(t, block_id, _, _)| {
                                t.0 == "block" && !used_blocks.contains(&block_id.0)
                            })
                            .map(|(_, bid, bpos, be)| (bid.0, cmap.dijkstra[bpos.get_idx()], *be))
                            .collect();

                    println!("Found {} blocks", blocks.len());
                    if !blocks.is_empty() {
                        blocks.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                        let selected_block = blocks[0].0;
                        println!("Enabled a construction job {:?}", build_id);

                        // Add a blueprint and haul order
                        commands.add_component(
                            *e,
                            Blueprint {
                                ready_to_build: false,
                                required_items: vec![selected_block],
                            },
                        );
                        commands.add_component(blocks[0].2, Claimed { by: build_id.0 });
                        commands.add_component(
                            blocks[0].2,
                            RequestHaul {
                                in_progress: None,
                                destination: idx,
                            },
                        );

                        cmap.is_dirty = true;
                        available_blocks -= 1;
                        used_blocks.insert(blocks[0].0);
                    }
                }
            }
        });
}
