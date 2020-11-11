use bengine::gui::*;
use legion::*;
use nox_components::*;
use nox_planet::LumberMap;
use nox_spatial::mapidx;

pub fn lumberjack_display(imgui: &Ui, ecs: &mut World, mouse_world_pos: &(usize, usize, usize), lumber_map: &mut LumberMap) {
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
        <(&mut Tree, &Position)>::query()
            .iter_mut(ecs)
            .filter(|(_, pos)| pos.get_idx() == idx)
            .for_each(|(tree, _)| {
                tree.chop = true;
            }
        );
        lumber_map.is_dirty = true;
    }

    if imgui.io().mouse_down[1] {
        let idx = mapidx(mouse_world_pos.0, mouse_world_pos.1, mouse_world_pos.2);
        <(&mut Tree, &Position)>::query()
            .iter_mut(ecs)
            .filter(|(_, pos)| pos.get_idx() == idx)
            .for_each(|(tree, _)| {
                tree.chop = false;
            }
        );
        lumber_map.is_dirty = true;
    }
}
