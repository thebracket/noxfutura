use super::tables::*;
use bengine::gui::*;
use legion::*;
use nox_components::*;
use nox_raws::RAWS;

pub fn show_building_info(imgui: &Ui, ecs: &World, id: &usize) {
    let (name, description, building, entity, btag) = <(&IdentityTag, &Name, &Description, &Building, Entity, &Tag)>::query()
        .iter(ecs)
        .filter(|(bid, _, _, _, _, _)| bid.0 == *id)
        .map(|(_, n, d, b, e, tag)| (ImString::new(&n.name), ImString::new(&d.desc), b, *e, tag.0.clone()))
        .nth(0)
        .unwrap();
    let window = Window::new(&name);
    window
        .size(
            [400.0, 400.0],
            Condition::FirstUseEver,
        )
        .movable(true)
        .position([20.0, 20.0], Condition::FirstUseEver)
        .build(imgui, || {
            imgui.text_wrapped(&description);
            if !building.complete {
                imgui.text_colored([1.0, 0.0, 0.0, 1.0], im_str!("(Incomplete)"));
            }

            // Check container contents
            let mut has_contents = false;
            <(Read<Name>, Read<Position>)>::query()
                .iter(ecs)
                .filter(|(_, store)| store.is_in_container(*id))
                .for_each(|(name, _)| {
                    if !has_contents {
                        has_contents = true;
                        imgui.text_colored([1.0, 1.0, 0.0, 1.0], im_str!("Contains the following items:"));
                    }
                    imgui.text(ImString::new(&name.name));
                }
            );

            // Check for reactions
            if let Some(er) = ecs.entry_ref(entity) {
                if let Ok(_ws) = er.get_component::<Workshop>() {
                    imgui.text_colored([1.0, 1.0, 0.0, 1.0], im_str!("Available Commands:"));
                    RAWS.read().reactions.reactions.iter()
                        .filter(|r| r.workshop == btag)
                        .for_each(|r| {
                            imgui.text(&ImString::new(&r.name));
                            if r.automatic {
                                imgui.same_line(0.0);
                                imgui.text(im_str!("(auto)"));
                            }
                        }
                    );
                }
            }
        }
    );
}