use nox_planet::{MiningMap, MiningMode};
use crate::modes::playgame::systems::REGION;
use bengine::gui::*;
use legion::*;
use nox_components::*;
use nox_planet::*;
use nox_spatial::mapidx;

pub fn mining_display(
    imgui: &Ui,
    ecs: &mut World,
    mouse_world_pos: &(usize, usize, usize),
    mining_mode: &mut MiningMode,
    mine_state: &mut MiningMap
) {
    let dig_modes = [
        im_str!("Dig"),
        im_str!("Channel"),
        im_str!("Ramp"),
        im_str!("Up Ladder"),
        im_str!("Down Ladder"),
        im_str!("Up/Down Ladder"),
        im_str!("Clear"),
    ];

    let mut dig_mode = match mining_mode {
        MiningMode::Dig => 0,
        MiningMode::Channel => 1,
        MiningMode::Ramp => 2,
        MiningMode::Up => 3,
        MiningMode::Down => 4,
        MiningMode::UpDown => 5,
        _ => 6
    };

    let title = format!("Mining Mode. ### ManicMiner",);
    let title_tmp = ImString::new(title);
    let window = Window::new(&title_tmp);
    window
        .size([420.0, 300.0], Condition::FirstUseEver)
        .movable(true)
        .position([0.0, 20.0], Condition::FirstUseEver)
        .build(imgui, || {
            imgui.text(im_str!("Mining Mode: "));
            imgui.same_line(0.0);
            ComboBox::new(im_str!("##minemode"))
                .build_simple_string(&imgui, &mut dig_mode, &dig_modes);
        }
    );

    *mining_mode = match dig_mode {
        0 => MiningMode::Dig,
        1 => MiningMode::Channel,
        2 => MiningMode::Ramp,
        3 => MiningMode::Up,
        4 => MiningMode::Down,
        5 => MiningMode::UpDown,
        _ => MiningMode::Clear,
    };

    if !imgui.io().want_capture_mouse() && imgui.io().mouse_down[0] {
        let camera_pos = <&Position>::query()
            .filter(component::<CameraOptions>())
            .iter(ecs)
            .nth(0)
            .unwrap()
            .as_point3();

        // Only allow work on the current z-layer
        if mouse_world_pos.2 != camera_pos.z as usize {
            return;
        }

        // Validate types
        let idx = mapidx(mouse_world_pos.0, mouse_world_pos.1, camera_pos.z as usize);
        {
            let rlock = REGION.read();
            match mining_mode {
                MiningMode::Dig => {
                    match rlock.tile_types[idx] {
                        TileType::Empty => return,
                        TileType::Floor => return,
                        _ => {}
                    }
                }
                MiningMode::Channel => {
                    if rlock.tile_types[idx] != TileType::Floor {
                        return;
                    }
                }
                _ => {}
            }
        }

        let mut rlock = REGION.write();
        if *mining_mode == MiningMode::Clear {
            rlock.jobs_board.mining_designations.remove(&idx);
        } else {
            rlock.jobs_board.mining_designations.insert(idx, *mining_mode);
        }
        mine_state.is_dirty = true;
    }
}
