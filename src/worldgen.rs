use bracket_lib::prelude::*;
use bracket_ui::{RetainedGui, RetainedGuiEvent};

pub enum WorldGenResult {
    Continue,
    MainMenu,
    MakeWorld,
}

pub struct WorldGen {
    last_size: (u32, u32),
    gui: RetainedGui,
}

impl WorldGen {
    pub fn new() -> Self {
        let mut gui = RetainedGui::new(Rect::with_size(0, 0, 10, 10));
        gui.add_double_box(RGB::named(WHITE), RGB::named(BLACK), 0);
        gui.add_label_centered(
            RGB::named(YELLOW),
            RGB::named(BLACK),
            "World Generation Parameters",
            1,
            1,
        );
        gui.add_label(RGB::named(GRAY), RGB::named(BLACK), "Seed", 3, 3, 2);
        gui.add_label(RGB::named(GRAY), RGB::named(BLACK), "Water Level", 3, 4, 3);
        gui.add_label(RGB::named(GRAY), RGB::named(BLACK), "Plains Level", 3, 5, 4);
        gui.add_label(
            RGB::named(GRAY),
            RGB::named(BLACK),
            "Starting Settlers",
            3,
            6,
            5,
        );
        gui.add_label(
            RGB::named(GRAY),
            RGB::named(BLACK),
            "Strict Beamdown",
            3,
            7,
            6,
        );
        gui.add_checkbox(22, 7, true, 7);
        gui.add_label(RGB::named(GRAY), RGB::named(BLACK), "Extra Noise", 3, 8, 8);
        gui.add_checkbox(22, 8, true, 9);
        gui.add_label(RGB::named(GRAY), RGB::named(BLACK), "Bumpiness", 3, 9, 10);
        gui.add_checkbox(22, 9, true, 11);

        gui.add_button(
            "GENERATE WORLD",
            3,
            13,
            RGB::named(WHITE),
            RGB::named(BLUE),
            12,
        );
        gui.add_button(
            "RETURN TO MENU",
            30,
            13,
            RGB::named(WHITE),
            RGB::named(RED),
            13,
        );

        gui.build();
        Self {
            last_size: (0, 0),
            gui,
        }
    }

    pub fn tick(&mut self, ctx: &mut BTerm) -> WorldGenResult {
        let console_size = ctx.get_char_size();

        let left = console_size.0 / 2 - 25;
        let right = console_size.0 / 2 + 25;
        let top = console_size.1 / 2 - 10;
        let bottom = console_size.1 / 2 + 10;

        if console_size != self.last_size {
            self.last_size = console_size;
            self.gui
                .resize(Rect::with_exact(left + 1, top + 1, right - 1, bottom - 1));
        }

        ctx.cls();
        if let Some(result) = self.gui.tick(ctx) {
            match result {
                RetainedGuiEvent::Click(btn) => {
                    if btn == 12 {
                        return WorldGenResult::MainMenu;
                    } else if btn == 13 {
                        return WorldGenResult::MakeWorld;
                    }
                }
                _ => {}
            }
        }

        WorldGenResult::Continue
    }
}
