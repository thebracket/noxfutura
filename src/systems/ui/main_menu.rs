use crate::modes::{DesignMode, RunState};
use imgui::*;
use legion::prelude::*;
use nox_components::*;
use ultraviolet::Vec3;

// Returns the sun position/color, since we look there anyway
pub fn draw_main_menu(ecs: &World, run_state: &mut RunState, imgui: &Ui) -> (Vec3, Vec3) {
    let mut sun_pos = (Vec3::zero(), Vec3::zero());
    // Obtain info to display
    let mut hud_time = String::new();
    let query = <Read<Calendar>>::query();
    for c in query.iter(ecs) {
        hud_time = c.get_date_time();
        sun_pos = c.calculate_sun_moon();
    }

    let running_str = match run_state {
        RunState::OneStep => im_str!("\u{f051} Single-Step ### RunMenu"),
        RunState::Running => im_str!("\u{f144} Running ### RunMenu"),
        RunState::FullSpeed => im_str!("\u{f04e} Max Speed ### RunMenu"),
        _ => im_str!("\u{f017} Paused ### RunMenu"),
    };

    if let Some(menu_bar) = imgui.begin_main_menu_bar() {
        MenuItem::new(im_str!("\u{f135} Nox Futura ### NFMain")).build(imgui);

        if let Some(menu) = imgui.begin_menu(running_str, true) {
            if MenuItem::new(im_str!("\u{f017} Pause"))
                .shortcut(im_str!("SPACE"))
                .build(imgui)
            {
                *run_state = RunState::Paused;
            }
            if MenuItem::new(im_str!("\u{f051} Single_Step"))
                .shortcut(im_str!("`"))
                .build(imgui)
            {
                *run_state = RunState::OneStep;
            }
            if MenuItem::new(im_str!("\u{f144} Normal Speed"))
                .shortcut(im_str!("1"))
                .build(imgui)
            {
                *run_state = RunState::Running;
            }
            if MenuItem::new(im_str!("\u{f04e} Max Speed"))
                .shortcut(im_str!("2"))
                .build(imgui)
            {
                *run_state = RunState::FullSpeed;
            }
            menu.end(imgui);
        }

        if let Some(menu) = imgui.begin_menu(im_str!("\u{f1b3} Design"), true) {
            if MenuItem::new(im_str!("\u{f1bb} Lumberjack"))
                .shortcut(im_str!("T"))
                .build(imgui)
            {
                *run_state = RunState::Design {
                    mode: DesignMode::Lumberjack,
                };
            }
            menu.end(imgui);
        }

        let hud_time_im = ImString::new(hud_time);
        let status_size = imgui.calc_text_size(&hud_time_im, false, 0.0);
        imgui.same_line(imgui.window_content_region_width() - (status_size[0] + 10.0));
        imgui.text(hud_time_im);

        menu_bar.end(imgui);
    }

    sun_pos
}
