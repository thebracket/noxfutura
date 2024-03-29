use super::super::RunState;
use crate::modes::playgame::DesignMode;
use bengine::gui::*;
use legion::*;
use nox_components::*;

pub fn draw_main_menu(ecs: &World, run_state: &mut RunState, imgui: &Ui) {
    if let Some(menu_bar) = imgui.begin_main_menu_bar() {
        let running_str = match run_state {
            RunState::SlowMo => im_str!("\u{f051} Slow Motion ### RunMenu"),
            RunState::Running => im_str!("\u{f144} Running ### RunMenu"),
            RunState::FullSpeed => im_str!("\u{f04e} Max Speed ### RunMenu"),
            _ => im_str!("\u{f017} Paused ### RunMenu"),
        };

        let mut hud_time = String::new();
        let mut query = <Read<Calendar>>::query();
        for c in query.iter(ecs) {
            hud_time = c.get_date_time();
        }

        MenuItem::new(im_str!("\u{f135} Nox Futura ### NFMain")).build(imgui);

        if let Some(menu) = imgui.begin_menu(running_str, true) {
            if MenuItem::new(im_str!("\u{f017} Pause"))
                .shortcut(im_str!("`"))
                .build(imgui)
            {
                *run_state = RunState::Paused;
            }
            if MenuItem::new(im_str!("\u{f051} Slow Motion"))
                .shortcut(im_str!("1"))
                .build(imgui)
            {
                *run_state = RunState::Running;
            }
            if MenuItem::new(im_str!("\u{f144} Normal Speed"))
                .shortcut(im_str!("2"))
                .build(imgui)
            {
                *run_state = RunState::Running;
            }
            if MenuItem::new(im_str!("\u{f04e} Max Speed"))
                .shortcut(im_str!("3"))
                .build(imgui)
            {
                *run_state = RunState::FullSpeed;
            }
            menu.end(imgui);
        }

        if let Some(menu) = imgui.begin_menu(im_str!("\u{f03a} Info"), true) {
            if MenuItem::new(im_str!("\u{f2c2} Manage Settlers"))
                .shortcut(im_str!("S"))
                .build(imgui)
            {
                *run_state = RunState::Design {
                    mode: DesignMode::SettlerList,
                };
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
            if MenuItem::new(im_str!("\u{f1b3} Mining"))
                .shortcut(im_str!("D"))
                .build(imgui)
            {
                *run_state = RunState::Design {
                    mode: DesignMode::Mining {
                        mode: MiningMode::Dig,
                    },
                };
            }
            if MenuItem::new(im_str!("\u{f015} Buildings"))
                .shortcut(im_str!("B"))
                .build(imgui)
            {
                *run_state = RunState::Design {
                    mode: DesignMode::Buildings { bidx: 0, vox: None },
                };
            }
            if MenuItem::new(im_str!("\u{f009} Construction"))
                .shortcut(im_str!("C"))
                .build(imgui)
            {
                *run_state = RunState::Design {
                    mode: DesignMode::Construction,
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
}
