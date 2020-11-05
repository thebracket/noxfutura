use crate::modes::playgame::systems::REGION;
use bengine::{geometry::Point3, gui::*};
use legion::*;
use nox_components::*;
use nox_planet::*;
use nox_planet::{MiningMap, MiningMode};
use nox_spatial::mapidx;
use parking_lot::RwLock;

pub struct MiningParams {
    pub brush_width: i32,
    pub brush_height: i32,
    pub stairs_depth: i32,
}

impl MiningParams {
    fn new() -> Self {
        Self {
            brush_height: 1,
            brush_width: 1,
            stairs_depth: 1,
        }
    }
}

lazy_static! {
    pub static ref MINING_PARAMS: RwLock<MiningParams> = RwLock::new(MiningParams::new());
}

pub fn mining_display(
    imgui: &Ui,
    ecs: &mut World,
    mouse_world_pos: &(usize, usize, usize),
    mining_mode: &mut MiningMode,
    mine_state: &mut MiningMap,
) {
    let dig_modes = [
        im_str!("Dig (D)"),
        im_str!("Channel (H)"),
        im_str!("Ramp (R)"),
        im_str!("Up Ladder (U)"),
        im_str!("Down Ladder (J)"),
        im_str!("Up/Down Ladder (K)"),
        im_str!("Clear (X)"),
    ];

    let mut dig_mode = match mining_mode {
        MiningMode::Dig => 0,
        MiningMode::Channel => 1,
        MiningMode::Ramp => 2,
        MiningMode::Up => 3,
        MiningMode::Down => 4,
        MiningMode::UpDown => 5,
        _ => 6,
    };

    let title = format!("Mining Mode. ### ManicMiner",);
    let title_tmp = ImString::new(title);
    let window = Window::new(&title_tmp);
    window
        .size([420.0, 100.0], Condition::FirstUseEver)
        .movable(true)
        .position([0.0, 20.0], Condition::FirstUseEver)
        .build(imgui, || {
            imgui.text(im_str!("Mining Mode: "));
            imgui.same_line(0.0);
            ComboBox::new(im_str!("##minemode")).build_simple_string(
                &imgui,
                &mut dig_mode,
                &dig_modes,
            );

            let mut mp = MINING_PARAMS.write();
            match mining_mode {
                MiningMode::Dig => {
                    imgui
                        .input_int(im_str!("Width"), &mut mp.brush_width)
                        .step(1)
                        .step_fast(1)
                        .build();
                    imgui
                        .input_int(im_str!("Height"), &mut mp.brush_height)
                        .step(1)
                        .step_fast(1)
                        .build();
                }
                MiningMode::Down => {
                    imgui
                        .input_int(im_str!("Depth"), &mut mp.stairs_depth)
                        .step(1)
                        .step_fast(1)
                        .build();
                }
                MiningMode::Up => {
                    imgui
                        .input_int(im_str!("Depth"), &mut mp.stairs_depth)
                        .step(1)
                        .step_fast(1)
                        .build();
                }
                _ => {}
            }
            std::mem::drop(mp);
        });

    *mining_mode = match dig_mode {
        0 => MiningMode::Dig,
        1 => MiningMode::Channel,
        2 => MiningMode::Ramp,
        3 => MiningMode::Up,
        4 => MiningMode::Down,
        5 => MiningMode::UpDown,
        _ => MiningMode::Clear,
    };

    if !imgui.io().want_capture_mouse && imgui.io().mouse_down[0] {
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

        // Apply the mining designations
        let idx = mapidx(mouse_world_pos.0, mouse_world_pos.1, camera_pos.z as usize);
        match mining_mode {
            MiningMode::Clear => {
                REGION.write().jobs_board.mining_designations.remove(&idx);
            }
            MiningMode::Dig => {
                let mp = MINING_PARAMS.read();
                for iy in 0..mp.brush_height as usize {
                    for ix in 0..mp.brush_width as usize {
                        let idx = mapidx(
                            mouse_world_pos.0 + ix,
                            mouse_world_pos.1 + iy,
                            camera_pos.z as usize,
                        );
                        let mut mwp = *mouse_world_pos;
                        mwp.0 += ix;
                        mwp.1 += iy;
                        if validate_mining(&mwp, mining_mode, &camera_pos) {
                            REGION
                                .write()
                                .jobs_board
                                .mining_designations
                                .insert(idx, *mining_mode);
                        }
                    }
                }
            }
            MiningMode::Down => {
                let mp = MINING_PARAMS.read();
                // Add down stairs at the start
                if validate_mining(mouse_world_pos, mining_mode, &camera_pos) {
                    REGION
                        .write()
                        .jobs_board
                        .mining_designations
                        .insert(idx, *mining_mode);
                }

                // If there are more add up at the bottom
                if mp.stairs_depth > 1 {
                    let idx = mapidx(
                        mouse_world_pos.0,
                        mouse_world_pos.1,
                        camera_pos.z as usize - (mp.stairs_depth - 1) as usize,
                    );
                    let mut mwp = *mouse_world_pos;
                    mwp.2 -= (mp.stairs_depth - 1) as usize;
                    if validate_mining(&mwp, mining_mode, &camera_pos) {
                        REGION
                            .write()
                            .jobs_board
                            .mining_designations
                            .insert(idx, MiningMode::Up);
                    }
                }

                // Fill in intermediate with up/down
                if mp.stairs_depth > 2 {
                    let mut sy = 1;
                    while sy < mp.stairs_depth {
                        let idx = mapidx(
                            mouse_world_pos.0,
                            mouse_world_pos.1,
                            camera_pos.z as usize - sy as usize,
                        );
                        let mut mwp = *mouse_world_pos;
                        mwp.2 -= sy as usize;
                        if validate_mining(&mwp, mining_mode, &camera_pos) {
                            REGION
                                .write()
                                .jobs_board
                                .mining_designations
                                .insert(idx, MiningMode::UpDown);
                        }

                        sy += 1;
                    }
                }
            }
            _ => {
                if validate_mining(mouse_world_pos, mining_mode, &camera_pos) {
                    REGION
                        .write()
                        .jobs_board
                        .mining_designations
                        .insert(idx, *mining_mode);
                }
            }
        }

        // Mark as dirty
        mine_state.is_dirty = true;
    }
}

pub fn validate_mining(
    mouse_world_pos: &(usize, usize, usize),
    mining_mode: &MiningMode,
    camera_pos: &Point3,
) -> bool {
    let idx = mapidx(mouse_world_pos.0, mouse_world_pos.1, camera_pos.z as usize);
    let rlock = REGION.read();
    match mining_mode {
        MiningMode::Dig => match rlock.tile_types[idx] {
            TileType::Empty => false,
            TileType::Floor => false,
            _ => true,
        },
        MiningMode::Down | MiningMode::UpDown | MiningMode::Up => match rlock.tile_types[idx] {
            TileType::Empty => false,
            _ => true,
        },
        MiningMode::Channel => rlock.tile_types[idx] == TileType::Floor,
        _ => false,
    }
}
