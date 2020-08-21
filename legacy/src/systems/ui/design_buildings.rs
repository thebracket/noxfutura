use crate::systems::REGION;
use imgui::*;
use legion::*;
use nox_components::*;
use nox_planet::*;
use nox_raws::*;
use nox_spatial::mapidx;

struct AvailableBuilding {
    tag: String,
    name: ImString,
    description: ImString,
    model_idx: usize,
    dimensions: Option<(i32, i32, i32)>,
}

pub fn building_display(
    imgui: &Ui,
    ecs: &mut World,
    mouse_world_pos: &(usize, usize, usize),
    bidx: i32,
) -> (i32, Option<usize>) {
    let mut available_buildings = Vec::<AvailableBuilding>::new();

    // Temporary code to figure out what buildings are possible
    let raws = RAWS.read();
    let region = REGION.read();
    raws.buildings.buildings.iter().for_each(|b| {
        let mut has_all = true;
        b.components.iter().for_each(|c| {
            let n = <(Entity, Read<Position>, Read<Tag>, Read<IdentityTag>)>::query()
                .iter(ecs)
                .filter(|(_, _, t, _)| t.0 == c.item.to_string())
                .filter(|(_, _, _, idt)| region.jobs_board.is_component_claimed(idt.0) == false)
                .count();
            if n < c.qty as usize {
                has_all = false;
            }
        });
        if has_all {
            available_buildings.push(AvailableBuilding {
                tag: b.tag.to_string(),
                name: ImString::new(b.name.to_string()),
                description: ImString::new(b.description.to_string()),
                model_idx: raws.vox.get_model_idx(&b.vox),
                dimensions: b.dimensions,
            });
        }
    });

    let mut bid = bidx;
    let blist: Vec<&ImString> = available_buildings.iter().map(|b| &b.name).collect();

    let title = format!("Building Mode. ### BobTheBuilder",);
    let title_tmp = ImString::new(title);
    let window = imgui::Window::new(&title_tmp);
    window
        .size([420.0, 300.0], Condition::FirstUseEver)
        .movable(true)
        .position([0.0, 20.0], Condition::FirstUseEver)
        .build(imgui, || {
            imgui.text(im_str!("Available buildings:"));
            imgui.list_box(im_str!("### build_list"), &mut bid, &blist, 10);
            if (bid as usize) < available_buildings.len() {
                imgui.text_wrapped(&available_buildings[bid as usize].description);
            }
        });

    crate::messaging::vox_moved();

    if available_buildings.is_empty() {
        (bid, None)
    } else {
        let building_info = &available_buildings[bid as usize];
        let btag = building_info.model_idx;
        let rtag = &building_info.tag;
        let dims = &building_info.dimensions;

        // Determine build validity here
        let mut occupied_tiles = Vec::new();
        let (width, height) = if let Some(dims) = dims {
            (dims.0, dims.1)
        } else {
            (1, 1)
        };

        if width == 1 && height == 1 {
            occupied_tiles.push(mapidx(
                mouse_world_pos.0,
                mouse_world_pos.1,
                mouse_world_pos.2,
            ));
        }
        // TODO: Other sizes

        let mut can_build = true;
        occupied_tiles.iter().for_each(|idx| {
            if !region.flag(*idx, Region::CAN_STAND_HERE) {
                can_build = false;
            }
            match &region.tile_types[*idx] {
                TileType::Stairs { .. } => {
                    can_build = false;
                }
                TileType::Ramp { .. } => {
                    can_build = false;
                }
                _ => {}
            }

            // Check to see if the space if occupied
            <Read<Position>>::query()
                .filter(component::<Building>())
                .iter(ecs)
                .for_each(|p| {
                    if p.contains_point(&mouse_world_pos) {
                        can_build = false;
                    }
                });
        });
        let world_idx = region.world_idx;
        std::mem::drop(region);

        if can_build && !imgui.io().want_capture_mouse {
            if imgui.io().mouse_down[0] {
                // Issue build order
                let new_building_id = nox_planet::spawn_building(
                    ecs,
                    &rtag,
                    mouse_world_pos.0,
                    mouse_world_pos.1,
                    mouse_world_pos.2,
                    world_idx,
                    false,
                );

                // Claim the components
                let binfo = raws.buildings.building_by_tag(rtag).unwrap();
                let mut chosen_components = Vec::new();
                for c in binfo.components.iter() {
                    let t = Tag(c.item.to_string());
                    let mut available_components: Vec<(usize, usize)> =
                        <(Entity, Read<Position>, Read<Tag>, Read<IdentityTag>)>::query()
                            .filter(component::<Item>())
                            .iter(ecs)
                            .filter(|(_, _, tag, _)| tag.0 == t.0)
                            .map(|(_, pos, _, idt)| (pos.effective_location(ecs), idt.0))
                            .collect();
                    available_components.sort_by(|a, b| a.0.cmp(&b.0));
                    available_components
                        .iter()
                        .take(c.qty as usize)
                        .for_each(|cc| {
                            chosen_components.push(cc.clone());
                        });
                }

                let mut rwlock = REGION.write();
                chosen_components.iter().for_each(|c| {
                    rwlock
                        .jobs_board
                        .claim_component_for_building(new_building_id, c.1, c.0);
                });
                rwlock.jobs_board.add_building_job(
                    new_building_id,
                    mapidx(mouse_world_pos.0, mouse_world_pos.1, mouse_world_pos.2),
                    &chosen_components,
                );
            }

            (bid, Some(btag))
        } else {
            (bid, None)
        }
    }
}
