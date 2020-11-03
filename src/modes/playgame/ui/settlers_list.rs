use super::tables::*;
use crate::modes::playgame::systems::REGION;
use bengine::gui::*;
use legion::*;
use nox_components::*;
use nox_spatial::mapidx;

pub fn settler_list_display(imgui: &Ui, ecs: &World) {
    let headings = im_str!("Name\0Profession");

    let size = bengine::get_window_size();
    let title = format!("All Settlers. ### SettlerList",);
    let title_tmp = ImString::new(title);
    let window = Window::new(&title_tmp);
    window
        .size(
            [size.width as f32 - 40.0, size.height as f32 - 40.0],
            Condition::FirstUseEver,
        )
        .movable(true)
        .position([20.0, 20.0], Condition::FirstUseEver)
        .build(imgui, || {
            begin_table(
                &["Name", "Profession", "Mining", "Lumber", "Options"],
                imgui,
                "settler_list",
                true,
            );

            // Make something here
            <(Entity, &Name, &Tagline, &Settler, &IdentityTag)>::query()
                .iter(ecs)
                .for_each(|(e, n, t, settler, id)| {
                    imgui.text(ImString::new(&n.name));
                    imgui.next_column();

                    imgui.text(ImString::new(&t.name));
                    imgui.next_column();

                    if settler.miner {
                        let label = format!("\u{f05e} Mining");
                        if imgui.button(&ImString::new(label), [100.0, 20.0]) {
                            crate::modes::playgame::fire_miner(id.0);
                        }
                    } else {
                        if REGION
                            .read()
                            .jobs_board
                            .available_tool_count(ToolType::Digging)
                        {
                            let label = format!("\u{f1b3} Miner##{}", id.0);
                            if imgui.button(&ImString::new(label), [100.0, 20.0]) {
                                crate::modes::playgame::become_miner(id.0);
                            }
                        } else {
                            imgui.text(im_str!("Not Miner"));
                        }
                    }
                    imgui.next_column();
                    if settler.lumberjack {
                        let label = format!("\u{f05e} Lumberjack");
                        if imgui.button(&ImString::new(label), [100.0, 20.0]) {
                            crate::modes::playgame::fire_miner(id.0);
                        }
                    } else {
                        if REGION
                            .read()
                            .jobs_board
                            .available_tool_count(ToolType::Chopping)
                        {
                            let label = format!("\u{f1bb} Lumberjack##{}", id.0);
                            if imgui.button(&ImString::new(label), [100.0, 20.0]) {
                                crate::modes::playgame::become_lumberjack(id.0);
                            }
                        } else {
                            imgui.text(im_str!("Not Lumberjack"));
                        }
                    }
                    imgui.next_column();

                    let label = format!("\u{f00e} View##{}", id.0);
                    imgui.button(&ImString::new(label), [100.0, 20.0]);
                    imgui.next_column();
                });
            end_table(imgui, im_str!("..."));
        });
}
