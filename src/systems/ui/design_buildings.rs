use imgui::*;
use legion::prelude::*;
use nox_spatial::mapidx;
use crate::systems::REGION;
use nox_planet::TileType;
use nox_components::*;
use nox_raws::*;

pub fn building_display(imgui: &Ui, ecs: &World, mouse_world_pos: &(usize, usize, usize), bidx: i32) -> (i32, Option<usize>) {
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
                    raws.vox.get_model_idx(&b.vox)
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
        let btag = available_buildings[bid as usize].3;

        // Determine build validity here

        (bid, Some(btag))
    }
}