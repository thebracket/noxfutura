use nox_components::*;
use crate::modes::playgame::systems::REGION;
use nox_spatial::mapidx;
use bengine::gui::*;
use legion::*;

pub fn lumberjack_display(imgui: &Ui, ecs: &World, mouse_world_pos: &(usize, usize, usize)) {
    let title = format!("Lumberjack Mode. Click trees to designate for chopping. ### LumberJack",);
    let title_tmp = ImString::new(title);
    let window = Window::new(&title_tmp);
    window
        .collapsed(true, Condition::FirstUseEver)
        .no_inputs()
        .size([420.0, 100.0], Condition::FirstUseEver)
        .movable(false)
        .position([0.0, 20.0], Condition::FirstUseEver)
        .build(imgui, || {});

    if imgui.io().mouse_down[0] {
        let idx = mapidx(mouse_world_pos.0, mouse_world_pos.1, mouse_world_pos.2);
        <(&Tree, &Position, &IdentityTag)>::query()
            .iter(ecs)
            .filter(|(_, pos, _)| pos.get_idx() == idx)
            .for_each(|(_, _, id)| {
                REGION.write().jobs_board.set_tree(id.0, idx);
            });
    }

    if imgui.io().mouse_down[1] {
        let idx = mapidx(mouse_world_pos.0, mouse_world_pos.1, mouse_world_pos.2);
        <(&Tree, &Position, &IdentityTag)>::query()
            .iter(ecs)
            .filter(|(_, pos, _)| pos.get_idx() == idx)
            .for_each(|(_, _, id)| {
                REGION.write().jobs_board.remove_tree(&id.0);
            });
    }
}
