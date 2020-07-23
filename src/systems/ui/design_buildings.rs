use imgui::*;
use legion::prelude::*;
use nox_spatial::mapidx;
use crate::systems::REGION;
use nox_planet::*;
use nox_components::*;
use nox_raws::*;
use bracket_geometry::prelude::Point3;

pub fn building_display(imgui: &Ui, ecs: &mut World, mouse_world_pos: &(usize, usize, usize), bidx: i32) -> (i32, Option<usize>) {
    let mut available_buildings = Vec::new();

    // Temporary code to figure out what buildings are possible
    let raws = RAWS.read();
    let region = REGION.read();
    raws.buildings.buildings.iter().for_each(|b| {
        let mut has_all = true;
        b.components.iter().for_each(|c| {
            let t = Tag(c.item.to_string());
            let n = <Read<Position>>::query()
                .filter(tag::<Item>())
                .filter(tag_value(&t))
                .iter_entities(ecs)
                .filter(|(entity, _)| region.jobs_board.is_component_claimed(
                    ecs.get_tag::<IdentityTag>(*entity).unwrap().0) == false
                )
                .count();
            if n < c.qty as usize {
                has_all = false;
            }
        });
        if has_all {
            available_buildings.push(
                (
                    b.tag.to_string(), 
                    ImString::new(b.name.to_string()),
                    ImString::new(b.description.to_string()),
                    raws.vox.get_model_idx(&b.vox),
                    b.dimensions
                )
            );
        }
    });

    let mut bid = bidx;
    let blist: Vec<&ImString> = available_buildings.iter().map(|b| &b.1).collect();

    let title = format!("Building Mode. ### BobTheBuilder",);
    let title_tmp = ImString::new(title);
    let window = imgui::Window::new(&title_tmp);
    window
        .size([420.0, 300.0], Condition::FirstUseEver)
        .movable(true)
        .position([0.0, 20.0], Condition::FirstUseEver)
        .build(imgui, || {
            imgui.text(im_str!("Building list goes here"));
            imgui.list_box(
                im_str!("### build_list"),
                &mut bid,
                &blist,
                10
            );
            if (bid as usize) < available_buildings.len() {
                imgui.text_wrapped(&available_buildings[bid as usize].2);
            }
        }
    );

    crate::messaging::vox_moved();

    if available_buildings.is_empty() {
        (bid, None)
    } else {
        let building_info = &available_buildings[bid as usize];
        let btag = building_info.3;
        let rtag = &building_info.0;
        let dims = &building_info.4;

        // Determine build validity here
        let mut occupied_tiles = Vec::new();
        let (width, height) = if let Some(dims) = dims {
            (dims.0, dims.1)
        } else {
            (1, 1)
        };

        if width==1 && height==1 {
            occupied_tiles.push(mapidx(mouse_world_pos.0, mouse_world_pos.1, mouse_world_pos.2));
        }
        // TODO: Other sizes

        let mut can_build = true;
        occupied_tiles.iter().for_each(|idx| {
            if !region.flag(*idx, Region::CAN_STAND_HERE) {
                can_build = false;
            }
            match &region.tile_types[*idx] {
                TileType::Stairs{..} => { can_build = false; }
                TileType::Ramp{..} => { can_build = false; }
                _ => {}
            }

            // Check to see if the space if occupied
            <Read<Position>>::query().filter(tag::<Building>()).iter(ecs).for_each(|p| {
                if p.contains_point(&mouse_world_pos) {
                    can_build = false;
                }
            });
        });
        let world_idx = region.world_idx;
        std::mem::drop(region);

        if can_build {
            if imgui.io().mouse_down[0] {
                // Issue build order
                let new_building_id = nox_planet::spawn_building(
                    ecs,
                    &rtag,
                    mouse_world_pos.0,
                    mouse_world_pos.1,
                    mouse_world_pos.2,
                    world_idx,
                    false
                );

                // Claim the components
                let binfo = raws.buildings.building_by_tag(rtag).unwrap();
                let mut chosen_components = Vec::new();
                for c in binfo.components.iter() {
                    let t = Tag(c.item.to_string());
                    let mut available_components : Vec<(usize, usize)> = <Read<Position>>::query()
                        .filter(tag::<Item>())
                        .filter(tag_value(&t))
                        .iter_entities(ecs)
                        .map(|(entity, pos)| {
                            (pos.effective_location(ecs), ecs.get_tag::<IdentityTag>(entity).unwrap().0)
                        })
                        .collect()
                    ;
                    available_components.sort_by(|a,b| a.0.cmp(&b.0));
                    available_components.iter().take(c.qty as usize).for_each(|cc| {
                        chosen_components.push(cc.clone());
                    });
                }

                let mut rwlock = REGION.write();
                chosen_components.iter().for_each(|c| {
                    rwlock.jobs_board.claim_component_for_building(
                        new_building_id, 
                        c.1, 
                        c.0
                    );
                });
                rwlock.jobs_board.add_building_job(
                    new_building_id, 
                    mapidx(mouse_world_pos.0, mouse_world_pos.1, mouse_world_pos.2),
                    &chosen_components
                );
            }

            (bid, Some(btag))
        } else {
            (bid, None)
        }
    }
}