use super::point_in_model;
use imgui::*;
use legion::prelude::*;
use nox_components::*;
use nox_planet::mapidx;

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
        <(Read<Position>, Read<Dimensions>, Read<Identity>)>::query()
            .filter(tag::<Tree>())
            .iter(ecs)
            .filter(|(pos, dims, _)| point_in_model(pos, dims, mouse_world_pos))
            .for_each(|(pos, _, id)| {
                let mut rlock = crate::systems::shared_state::REGION.write();
                rlock.jobs_board.set_tree(id.id, mapidx(pos.x, pos.y, pos.z));
                //println!("Designated tree #{}", id.id);
            });
    }

    if imgui.io().mouse_down[1] {
        <(Read<Position>, Read<Dimensions>, Read<Identity>)>::query()
            .filter(tag::<Tree>())
            .iter(ecs)
            .filter(|(pos, dims, _)| point_in_model(pos, dims, mouse_world_pos))
            .for_each(|(_, _, id)| {
                let mut rlock = crate::systems::shared_state::REGION.write();
                rlock.jobs_board.remove_tree(&id.id);
                //println!("UN-Designated tree #{}", id.id);
            });
    }
}
