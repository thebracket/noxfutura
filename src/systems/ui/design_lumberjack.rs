use imgui::*;
use legion::*;
use nox_spatial::mapidx;
use crate::systems::REGION;
use nox_planet::TileType;

fn find_tree_base(target: usize) -> usize {
    REGION.read().tree_bases[&target]
}

pub fn lumberjack_display(imgui: &Ui, _ecs: &World, mouse_world_pos: &(usize, usize, usize)) {
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
        let idx = mapidx(mouse_world_pos.0, mouse_world_pos.1, mouse_world_pos.2);
        let tree_id = match REGION.read().tile_types[idx] {
            TileType::TreeFoliage{ tree_id } => Some(tree_id),
            TileType::TreeTrunk{ tree_id } => Some(tree_id),
            _ => None
        };

        if let Some(tree_id) = tree_id {
            let tree_pos = find_tree_base(tree_id);
            crate::systems::shared_state::REGION.write().jobs_board.set_tree(tree_id, tree_pos);
            println!("Designated tree");
        }
    }

    if imgui.io().mouse_down[1] {
        let idx = mapidx(mouse_world_pos.0, mouse_world_pos.1, mouse_world_pos.2);
        let tree_id = match REGION.read().tile_types[idx] {
            TileType::TreeFoliage{ tree_id } => Some(tree_id),
            TileType::TreeTrunk{ tree_id } => Some(tree_id),
            _ => None
        };

        if let Some(tree_id) = tree_id {
            crate::systems::shared_state::REGION.write().jobs_board.remove_tree(&tree_id);
            println!("Undesignated tree");
        }
    }
}
