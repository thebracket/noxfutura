use bracket_lib::prelude::*;
use nox_api::DisplayMode;
mod main_menu;
use main_menu::{ MainMenu, MenuResult };

struct State {
    display_mode : DisplayMode,

    main_menu_handler : MainMenu
}

impl State {
    fn new() -> Self {
        Self {
            display_mode : DisplayMode::MainMenu,
            main_menu_handler : MainMenu::new()
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.display_mode {
            DisplayMode::WorldGen => {

            }
            DisplayMode::MainMenu => {
                let mr = self.main_menu_handler.tick(ctx);
                match mr {
                    MenuResult::MakeWorld => self.display_mode = DisplayMode::WorldGen,
                    MenuResult::Quit => ctx.quit(),
                    _ => {}
                }
            }
        }
        ctx.print(0, 0, format!("FPS: {}", ctx.fps));
    }
}

fn main() {
    let ctx = BTermBuilder::simple(120, 100)
        .unwrap()
        .with_title("Nox Futura")
        .with_automatic_console_resize(true)
        .build()
        .expect("Unable to start a terminal.");

    main_loop(ctx, State::new())
        .expect("Unable to run game loop");
}