use bracket_lib::prelude::*;
use nox_api::new_menu_tagline;
use bracket_ui::{ColorMenu, ColorMenuItem};

pub enum MenuResult {
    Continue,
    MakeWorld,
    ResumeGame,
    Quit
}

pub struct MainMenu {
    tagline: String,
    map: WorldMap,
    last_size : (u32, u32),
    seed: u64,
    elapsed: f32,
    main_menu : ColorMenu
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            tagline : new_menu_tagline(),
            map: WorldMap::new(200.0, 200.0, 42),
            last_size : (0, 0),
            seed: 42,
            elapsed : 0.0,
            main_menu : ColorMenu::new(
                vec![
                    ColorMenuItem{ order: 0, label: "Create New World".to_string() },
                    ColorMenuItem{ order: 1, label: "Continue Game".to_string() },
                    ColorMenuItem{ order: 2, label: "Quit to Desktop".to_string() },
                ]
            )
        }
    }

    pub fn tick(&mut self, ctx: &mut BTerm) -> MenuResult {
        self.elapsed += ctx.frame_time_ms;
        let console_size = ctx.get_char_size();
        if console_size != self.last_size || self.elapsed > 5000.0 {
            self.seed += 1;
            self.last_size = console_size;
            self.map = WorldMap::new(console_size.0 as f32, console_size.1 as f32, self.seed);
            self.elapsed = 0.0;
        }

        ctx.cls();

        self.map.render(ctx);

        let left = console_size.0 /2 - 25;
        let right = console_size.0/2 + 25;
        let top = console_size.1 / 2 - 10;
        let bottom = console_size.1 / 2 + 10;
        ctx.draw_box_double(left, top, 50, 20, WHITE, BLACK);

        ctx.print_color_centered(top+1, YELLOW, BLACK, "Nox Futura");
        ctx.print_color_centered(top+2, RED, BLACK, &self.tagline);
        ctx.print_color_centered(bottom - 2, GREEN, BLACK,  "Copyright (c) 2015-2021 Bracket Producutions.");

        self.main_menu.render(ctx, left + 2, right - 2, top + 5);
        self.main_menu.handle_key(&ctx.key);
        self.main_menu.handle_mouse(ctx, left + 2, right - 2, top + 5);

        match self.main_menu.get_selection() {
            Some(0) => return MenuResult::MakeWorld,
            Some(1) => return MenuResult::ResumeGame,
            Some(2) => return MenuResult::Quit,
            _ => {}
        }

        MenuResult::Continue
    }
}

struct WorldMap {
    noise : FastNoise,
    width: f32,
    height: f32
}

impl WorldMap {
    pub fn new(width: f32, height: f32, seed: u64) -> Self {
        let mut noise = FastNoise::seeded(seed);
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(10);
        noise.set_fractal_gain(0.5);
        noise.set_fractal_lacunarity(4.0);
        noise.set_frequency(0.01);

        Self{
            noise,
            width,
            height
        }
    }

    fn sphere_vertex(&self, altitude: f32, lat: f32, lon: f32) -> (f32, f32, f32) {
        (
            altitude * f32::cos(lat) * f32::cos(lon),
            altitude * f32::cos(lat) * f32::sin(lon),
            altitude * f32::sin(lat)
        )
    }

    fn tile_display(&self, x: i32, y:i32) -> (FontCharType, RGB) {
        let lat = (((y as f32 / self.height) * 180.0) - 90.0) * 0.017_453_3;
        let lon = (((x as f32 / self.width) * 360.0) - 180.0) * 0.017_453_3;
        let coords = self.sphere_vertex(100.0, lat, lon);
        let altitude = self.noise.get_noise3d(coords.0, coords.1, coords.2);
        if altitude < 0.0 {
            ( to_cp437('▒'), RGB::from_f32(0.0, 0.0, 1.0 + altitude) )
        } else if altitude < 0.5 {
            let greenness = 0.5 + (altitude / 1.0);
            ( to_cp437('█'), RGB::from_f32(0.0, greenness, 0.0) )
        } else {
            let greenness = 0.2 + (altitude / 1.0);
            ( to_cp437('▒'), RGB::from_f32(greenness, greenness, greenness) )
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let render = self.tile_display(x, y);
                ctx.set(x, y, render.1, RGB::from_f32(0.0, 0.0, 0.0), render.0);
            }
        }
    }
}