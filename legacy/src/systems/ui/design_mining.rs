use imgui::*;
use legion::*;
use crate::modes::MiningMode;

pub fn mining_display(imgui: &Ui, _ecs: &World, _mouse_world_pos: &(usize, usize, usize), mode: &MiningMode) -> MiningMode {
    let mut result = mode.clone();

    let dig;
    let channel; 
    let ramp;
    let up;
    let down; 
    let updown;

    match mode {
        MiningMode::Dig => {
            dig = true; channel = false; ramp = false; up = false; down = false; updown = false;
        }
        MiningMode::Ramp => {
            dig = false; channel = false; ramp = true; up = false; down = false; updown = false;
        }
        MiningMode::Channel => {
            dig = false; channel = true; ramp = false; up = false; down = false; updown = false;
        }
        MiningMode::Up => {
            dig = false; channel = false; ramp = false; up = true; down = false; updown = false;
        }
        MiningMode::Down => {
            dig = false; channel = false; ramp = false; up = false; down = true; updown = false;
        }
        MiningMode::UpDown => {
            dig = false; channel = false; ramp = false; up = false; down = false; updown = true;
        }
    }

    let title = format!("Mining Mode. ### ManicMiner",);
    let title_tmp = ImString::new(title);
    let window = imgui::Window::new(&title_tmp);
    window
        .size([420.0, 200.0], Condition::FirstUseEver)
        .movable(true)
        .position([0.0, 20.0], Condition::FirstUseEver)
        .build(imgui, || {
            imgui.text(im_str!("Choose a mining operation:"));

            if imgui.radio_button_bool(im_str!("Dig (remove solid tile)"), dig) {
                result = MiningMode::Dig;
            }
            if imgui.radio_button_bool(im_str!("Channel (remove floor, leaving ramp below)"), channel) {
                result = MiningMode::Channel;
            }
            if imgui.radio_button_bool(im_str!("Ramp (construct upwards ramp)"), ramp) {
                result = MiningMode::Ramp;
            }
            if imgui.radio_button_bool(im_str!("Stairs Up"), up) {
                result = MiningMode::Up
            }
            if imgui.radio_button_bool(im_str!("Stairs Down"), down) {
                result = MiningMode::Down;
            }
            if imgui.radio_button_bool(im_str!("Stairs Up-Down"), updown) {
                result = MiningMode::UpDown;
            }
        }
    );

    result
}
