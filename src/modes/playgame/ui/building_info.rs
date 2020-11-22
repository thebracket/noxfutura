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
            [600.0, 400.0],
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
            let reaction_modes = [
                im_str!("Make"),
                im_str!("Until You Have"),
            ];
            let mut current_mode = 0;
            let mut qty = 0;
            if let Ok(er) = ecs.entry_ref(entity) {
                if let Ok(_ws) = er.get_component::<Workshop>() {
                    imgui.text_colored([1.0, 1.0, 0.0, 1.0], im_str!("Available Commands:"));
                    RAWS.read().reactions.reactions.iter()
                        .filter(|r| r.workshop == btag)
                        .for_each(|r| {
                            imgui.text(&ImString::new(&r.name));
                            imgui.set_next_item_width(250.0);
                            if r.automatic {
                                imgui.same_line(0.0);
                                imgui.text(im_str!("(auto)"));
                            }
                            imgui.same_line(260.0);

                            imgui.set_next_item_width(100.0);
                            ComboBox::new(im_str!("##minemode")).build_simple_string(
                                &imgui,
                                &mut current_mode,
                                &reaction_modes,
                            );
                            imgui.same_line(0.0);
                            imgui.set_next_item_width(75.0);
                            imgui
                                .input_int(im_str!(""), &mut qty)
                                .step(1)
                                .step_fast(1)
                                .build();
                            imgui.same_line(0.0);
                            imgui.button(im_str!("Queue"), [50.0, 20.0]);
                        }
                    );
                }
            }
        }
    );
}