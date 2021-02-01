use bracket_lib::prelude::*;
use nox_api::DisplayMode;
mod main_menu;
use main_menu::{MainMenu, MenuResult};
mod worldgen;
use worldgen::WorldGen;

struct State {
    display_mode: DisplayMode,

    main_menu_handler: MainMenu,
    worldgen_handler: WorldGen,
}

impl State {
    fn new() -> Self {
        Self {
            display_mode: DisplayMode::MainMenu,
            main_menu_handler: MainMenu::new(),
            worldgen_handler: WorldGen::new(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut new_mode = self.display_mode;
        match self.display_mode {
            DisplayMode::WorldGen => match self.worldgen_handler.tick(ctx) {
                worldgen::WorldGenResult::MainMenu => new_mode = DisplayMode::MainMenu,
                _ => {}
            },
            DisplayMode::MainMenu => {
                let mr = self.main_menu_handler.tick(ctx);
                match mr {
                    MenuResult::MakeWorld => new_mode = DisplayMode::WorldGen,
                    MenuResult::Quit => ctx.quit(),
                    _ => {}
                }
            }
        }
        self.display_mode = new_mode;

        let console_size = ctx.get_char_size();
        ctx.print_color(
            0,
            console_size.1 - 1,
            GRAY,
            BLACK,
            format!("FPS: {}", ctx.fps),
        );
    }
}

fn main() {
    let ctx = BTermBuilder::simple(120, 100)
        .unwrap()
        .with_title("Nox Futura")
        .with_automatic_console_resize(true)
        .build()
        .expect("Unable to start a terminal.");

    main_loop(ctx, State::new()).expect("Unable to run game loop");
}
