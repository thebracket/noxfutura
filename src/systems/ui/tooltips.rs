use imgui::*;
use legion::prelude::*;
use nox_components::*;

pub fn point_in_model(pos: &Position, dims: &Dimensions, point: &(usize, usize, usize)) -> bool {
    if dims.width == 1 && dims.height == 1 && dims.depth == 1 {
        point.0 == pos.x && point.1 == pos.y && point.2 == pos.z
    } else if dims.width == 3 && dims.height == 3 {
        for x in pos.x - 1..pos.x + dims.width as usize - 1 {
            for y in pos.y - 1..pos.y + dims.height as usize - 1 {
                for z in pos.z..pos.z + dims.depth as usize {
                    if point.0 == x && point.1 == y && point.2 == z {
                        return true;
                    }
                }
            }
        }
        false
    } else {
        for x in pos.x..pos.x + dims.width as usize {
            for y in pos.y..pos.y + dims.height as usize {
                for z in pos.z..pos.z + dims.depth as usize {
                    if point.0 == x && point.1 == y && point.2 == z {
                        return true;
                    }
                }
            }
        }
        false
    }
}

pub fn draw_tooltips(ecs: &World, mouse_world_pos: &(usize, usize, usize), imgui: &Ui) {
    if imgui.io().want_capture_mouse {
        return;
    }

    let mut lines: Vec<(bool, String)> = Vec::new();

    <(Read<Name>, Read<Position>, Read<Identity>, Read<Dimensions>)>::query()
        .iter_entities(&ecs)
        .filter(|(_, (_, pos, _, dims))| point_in_model(pos, dims, mouse_world_pos))
        .for_each(|(entity, (name, _, identity, _))| {
            lines.push((true, format!("{}", name.name)));

            <Read<Description>>::query()
                .iter_entities(&ecs)
                .filter(|(e, _)| *e == entity)
                .for_each(|(_, d)| {
                    lines.push((false, format!("{}", d.desc)));
                });

            <(Read<Name>, Read<ItemStored>)>::query()
                .iter(&ecs)
                .filter(|(_, store)| store.container == identity.id)
                .for_each(|(name, _)| {
                    lines.push((false, format!(" - {}", name.name)));
                });
        });

    if !lines.is_empty() {
        let im_lines: Vec<(bool, ImString)> = lines
            .iter()
            .map(|(heading, s)| (*heading, ImString::new(s)))
            .collect();

        let size = crate::engine::DEVICE_CONTEXT.read().as_ref().unwrap().size;
        let mouse_pos = imgui.io().mouse_pos;
        let vsize = im_lines
            .iter()
            .map(|(_, s)| imgui.calc_text_size(s, false, 150.0)[1] + 10.0)
            .sum();

        let tip_pos = [
            f32::min(size.width as f32 - 300.0, mouse_pos[0]),
            f32::min(size.height as f32 - vsize, mouse_pos[1]),
        ];

        imgui::Window::new(im_str!("### tooltip"))
            .no_decoration()
            .size([300.0, vsize], Condition::Always)
            .collapsed(false, Condition::Always)
            .position(tip_pos, Condition::Always)
            .no_inputs()
            .build(imgui, || {
                im_lines.iter().for_each(|(heading, text)| {
                    if *heading {
                        imgui.text_colored([1.0, 1.0, 0.0, 1.0], text);
                    } else {
                        imgui.text_wrapped(text);
                    }
                });
            });
    }
}
