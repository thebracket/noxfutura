use crate::modes::playgame::systems::REGION;
use bengine::{geometry::Point3, gui::*};
use legion::*;
use nox_components::*;
use nox_planet::MiningMap;
use nox_planet::*;
use nox_spatial::mapidx;
use parking_lot::RwLock;

lazy_static! {
    static ref CONSTRUCTION_PARAMS: RwLock<ConstructionParams> =
        RwLock::new(ConstructionParams::new());
}

struct ConstructionParams {
    construct_mode: usize,
    allow_overcommit: bool,
}

impl ConstructionParams {
    fn new() -> Self {
        Self {
            construct_mode: 0,
            allow_overcommit: false,
        }
    }
}

pub fn show_construction(
    imgui: &Ui,
    ecs: &mut World,
    mouse_world_pos: &(usize, usize, usize),
    construction_map: &mut ConstructionMap,
) {
    let construct_modes = [
        im_str!("Wall (W)"),
        im_str!("Up Ladder (U)"),
        im_str!("Down Ladder (D)"),
        im_str!("Up/Down Ladder (J)"),
        im_str!("Floor (F)"),
        im_str!("Clear (X)"),
    ];

    let title = format!("Construction Mode. ### WallItIn",);
    let title_tmp = ImString::new(title);
    let window = Window::new(&title_tmp);
    window
        .size([420.0, 75.0], Condition::FirstUseEver)
        .movable(true)
        .position([0.0, 20.0], Condition::FirstUseEver)
        .build(imgui, || {
            let mut cp = CONSTRUCTION_PARAMS.write();
            let n_blocks = available_blocks(ecs);
            imgui.text(&ImString::new(format!("{} blocks available", n_blocks)));
            imgui.same_line(0.0);
            imgui.checkbox(im_str!("Allow Overcommit"), &mut cp.allow_overcommit);

            if n_blocks > 0 || cp.allow_overcommit {
                imgui.text(im_str!("Construction Mode: "));
                imgui.same_line(0.0);
                imgui.set_next_item_width(100.0);
                ComboBox::new(im_str!("##construct_mode")).build_simple_string(
                    &imgui,
                    &mut cp.construct_mode,
                    &construct_modes,
                );

                if !imgui.io().want_capture_mouse && imgui.io().mouse_down[0] {
                    let camera_pos = <&Position>::query()
                        .filter(component::<CameraOptions>())
                        .iter(ecs)
                        .nth(0)
                        .unwrap()
                        .as_point3();

                    if cp.construct_mode == 4 {
                        // Clearance mode
                        let idx =
                            mapidx(mouse_world_pos.0, mouse_world_pos.1, camera_pos.z as usize);
                        let to_remove: Vec<Entity> = <(Entity, &Construction, &Position)>::query()
                            .iter(ecs)
                            .filter(|(_, _, pos)| pos.get_idx() == idx)
                            .map(|(e, _, _)| *e)
                            .collect();
                        for e in to_remove.iter() {
                            ecs.remove(*e);
                        }
                    } else {
                        if can_build_here(ecs, &mouse_world_pos, &cp.construct_mode) {
                            let new_id = IdentityTag::new();
                            println!("New build job: {:?}", new_id);
                            ecs.push((
                                Construction {
                                    mode: cp.construct_mode,
                                    in_progress: None,
                                },
                                Position::with_tile_idx(
                                    mapidx(
                                        mouse_world_pos.0,
                                        mouse_world_pos.1,
                                        camera_pos.z as usize,
                                    ),
                                    REGION.read().world_idx,
                                    (1, 1, 1),
                                ),
                                new_id,
                            ));
                            construction_map.is_dirty = true;
                        } else {
                            println!("Rejecting build order");
                        }
                    }
                }
            }
        });
}

fn available_blocks(ecs: &World) -> usize {
    <(&Item, &Tag)>::query()
        .filter(!component::<Claimed>())
        .iter(ecs)
        .filter(|(_i, t)| t.0 == "block")
        .count()
}

fn can_build_here(
    ecs: &World,
    mouse_world_pos: &(usize, usize, usize),
    construct_mode: &usize,
) -> bool {
    let idx = mapidx(mouse_world_pos.0, mouse_world_pos.1, mouse_world_pos.2);

    // Are there any constructions here already?
    let n_present = <(&Construction, &Position)>::query()
        .iter(ecs)
        .filter(|(_, pos)| pos.get_idx() == idx)
        .count();
    if n_present > 0 {
        return false;
    }

    let rlock = REGION.read();
    let mut can_build = true;
    if !rlock.flag(idx, Region::CAN_STAND_HERE) {
        can_build = false;
    }

    can_build
}
