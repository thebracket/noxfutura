use crate::modes::playgame::systems::REGION;
use bengine::geometry::*;
use bengine::gui::*;
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
                .filter(!component::<Claimed>())
                .iter(ecs)
                .filter(|(_, _, t, _)| t.0 == c.item.to_string())
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
    let window = Window::new(&title_tmp);
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

    super::super::messaging::vox_moved();

    if available_buildings.is_empty() || bid as usize >= available_buildings.len() {
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
                let chosen_components = select_components(ecs, &raws, rtag, mouse_world_pos);
                let component_ids = chosen_components
                    .iter()
                    .map(|(_, id, _)| *id)
                    .collect::<Vec<usize>>();

                // Issue build order
                let new_building_id = nox_planet::spawn_building(
                    ecs,
                    &rtag,
                    mouse_world_pos.0,
                    mouse_world_pos.1,
                    mouse_world_pos.2,
                    world_idx,
                    false,
                    &component_ids,
                );

                // Claim the components
                let building_idx = mapidx(mouse_world_pos.0, mouse_world_pos.1, mouse_world_pos.2);
                chosen_components
                    .iter()
                    .for_each(|(_distance, _comp_id, comp_e)| {
                        if let Some(mut ce) = ecs.entry(*comp_e) {
                            ce.add_component(Claimed {
                                by: new_building_id,
                            });
                            ce.add_component(RequestHaul {
                                destination: building_idx,
                                in_progress: None,
                            });
                        }
                    });
            }

            (bid, Some(btag))
        } else {
            (bid, None)
        }
    }
}

fn select_components(
    ecs: &World,
    raws: &Raws,
    rtag: &String,
    mouse_world_pos: &(usize, usize, usize),
) -> Vec<(f32, usize, Entity)> {
    let mut result = Vec::new();
    let building_pos = Point3::new(mouse_world_pos.0, mouse_world_pos.1, mouse_world_pos.2);
    let binfo = raws.buildings.building_by_tag(rtag).unwrap();
    binfo.components.iter().for_each(|req_comp| {
        let mut available_components: Vec<(f32, usize, Entity)> =
            <(Entity, Read<Position>, Read<Tag>, Read<IdentityTag>)>::query()
                .filter(component::<Item>() & !component::<Claimed>())
                .iter(ecs)
                .filter(|(_, _, tag, _)| tag.0 == req_comp.item)
                .map(|(e, pos, _tag, id)| {
                    (
                        DistanceAlg::Pythagoras.distance3d(building_pos, pos.as_point3()),
                        id.0,
                        *e,
                    )
                })
                .collect();

        // Sort by closest
        available_components.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Take the first n
        available_components
            .iter()
            .take(req_comp.qty as usize)
            .for_each(|cc| result.push(cc.clone()));
    });

    result
}
