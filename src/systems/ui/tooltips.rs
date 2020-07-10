use nox_components::*;
use legion::prelude::*;
use imgui::*;

pub fn draw_tooltips(ecs: &World, mouse_world_pos: &(usize, usize, usize), imgui: &Ui) {
    let mut toolstring = String::new();
        <(Read<Name>, Read<Position>, Read<Identity>)>::query()
            .iter_entities(&ecs)
            .filter(| (_, (_, pos, _))| pos.x == mouse_world_pos.0 && pos.y == mouse_world_pos.1 && pos.z == mouse_world_pos.2 )
            .for_each(|(entity, (name, _, identity))| {
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