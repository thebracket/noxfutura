use imgui::*;
use legion::prelude::*;
use nox_components::*;

pub fn lumberjack_display(imgui: &Ui, ecs: &World, mouse_world_pos: &(usize, usize, usize)) {
    let title = format!("Lumberjack Mode. Click trees to designate for chopping. ### LumberJack",);
    let title_tmp = ImString::new(title);
    let window = imgui::Window::new(&title_tmp);
    window
        .collapsed(true, Condition::FirstUseEver)
        .no_inputs()
        .size([420.0, 100.0], Condition::FirstUseEver)
        .movable(false)
        .position([0.0, 20.0], Condition::FirstUseEver)
        .build(imgui, || {});

    if imgui.io().mouse_down[0] {
        <(Read<Position>, Tagged<IdentityTag>)>::query()
            .filter(tag::<Tree>())
            .iter(ecs)
            .filter(|(pos, _)| pos.contains_point(mouse_world_pos))
            .for_each(|(pos, id)| {
                let mut rlock = crate::systems::shared_state::REGION.write();
                rlock.jobs_board.set_tree(id.0, pos.get_idx());
                //println!("Designated tree #{}", id.id);
            });
    }

    if imgui.io().mouse_down[1] {
        <(Read<Position>, Tagged<IdentityTag>)>::query()
            .filter(tag::<Tree>())
            .iter(ecs)
            .filter(|(pos, _)| pos.contains_point(mouse_world_pos))
            .for_each(|(_, id)| {
                let mut rlock = crate::systems::shared_state::REGION.write();
                rlock.jobs_board.remove_tree(&id.0);
                //println!("UN-Designated tree #{}", id.id);
            });
    }
}
