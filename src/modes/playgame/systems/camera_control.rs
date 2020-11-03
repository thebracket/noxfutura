use super::super::GameStateResource;
use bengine::VirtualKeyCode;
use legion::world::SubWorld;
use legion::*;
use nox_components::{CameraMode, CameraOptions, Position};

#[system]
#[write_component(Position)]
#[write_component(CameraOptions)]
pub fn camera_control(ecs: &mut SubWorld, #[resource] state: &mut GameStateResource) {
    if let Some(key) = state.keycode {
        let mut query = <(&mut Position, &mut CameraOptions)>::query();
        for (pos, camopts) in query.iter_mut(ecs) {
            let mut cc = true;
            match key {
                VirtualKeyCode::Left => pos.apply_delta(-1, 0, 0),
                VirtualKeyCode::Right => pos.apply_delta(1, 0, 0),
                VirtualKeyCode::Up => pos.apply_delta(0, -1, 0),
                VirtualKeyCode::Down => pos.apply_delta(0, 1, 0),
                VirtualKeyCode::Comma => pos.apply_delta(0, 0, 1),
                VirtualKeyCode::Period => pos.apply_delta(0, 0, -1),
                VirtualKeyCode::Minus => camopts.zoom_level -= 1,
                VirtualKeyCode::Add => camopts.zoom_level += 1,
                VirtualKeyCode::Tab => match camopts.mode {
                    CameraMode::TopDown => camopts.mode = CameraMode::Front,
                    CameraMode::Front => camopts.mode = CameraMode::DiagonalNW,
                    CameraMode::DiagonalNW => camopts.mode = CameraMode::DiagonalNE,
                    CameraMode::DiagonalNE => camopts.mode = CameraMode::DiagonalSW,
                    CameraMode::DiagonalSW => camopts.mode = CameraMode::DiagonalSE,
                    CameraMode::DiagonalSE => camopts.mode = CameraMode::TopDown,
                },
                _ => cc = false,
            }
            state.camera_changed = cc;
        }
    }
}
