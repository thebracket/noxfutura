use nox_components::*;
use legion::prelude::*;
use imgui::*;

fn point_in_model(pos: &Position, dims: &Dimensions, point: &(usize, usize, usize)) -> bool {
    if dims.width == 1 && dims.height == 1 && dims.depth == 1 {
        point.0 == pos.x && point.1 == pos.y && point.2 == pos.z
    } else if dims.width == 3 && dims.height == 3 {
        for x in pos.x - 1 .. pos.x + dims.width as usize - 1 {
            for y in pos.y - 1 .. pos.y + dims.height as usize - 1 {
                for z in pos.z .. pos.z + dims.depth as usize {
                    if point.0 == x && point.1 == y && point.2 == z {
                        return true;
                    }
                }
            }
        }
        false
    } else {
        for x in pos.x .. pos.x + dims.width as usize {
            for y in pos.y .. pos.y + dims.height as usize {
                for z in pos.z .. pos.z + dims.depth as usize {
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
    let mut toolstring = String::new();
        <(Read<Name>, Read<Position>, Read<Identity>, Read<Dimensions>)>::query()
            .iter_entities(&ecs)
            .filter(| (_, (_, pos, _, dims))| {
                point_in_model(pos, dims, mouse_world_pos)
            })
            .for_each(|(entity, (name, _, identity, _))| {
                toolstring += &format!("{} #{}\n", name.name, identity.id);

                <Read<Description>>::query()
                    .iter_entities(&ecs)
                    .filter(|(e,_)| *e == entity)
                    .for_each(|(_,d)| {
                        toolstring += &format!("{}\n", d.desc);
                    }
                );

                <(Read<Name>, Read<ItemStored>)>::query()
                    .iter(&ecs)
                    .filter(|(_, store)| store.container == identity.id )
                    .for_each(|(name, _)| {
                        toolstring += &format!(" - {}\n", name.name);
                    }
                );
            }
        );
        if !toolstring.is_empty() {
            let info = ImString::new(toolstring);
            imgui::Window::new(im_str!("### tooltip"))
                .no_decoration()
                .size([300.0, 200.0], Condition::Always)
                .collapsed(false, Condition::Always)
                .position(imgui.io().mouse_pos, Condition::Always)
                .build(imgui, || {
                    imgui.text_wrapped(&info);
                });
        }
}