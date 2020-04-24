use bracket_lib::prelude::*;

pub struct WorldMap {
    noise : FastNoise,
    width: f32,
    height: f32
}

impl WorldMap {
    pub fn new(width: f32, height: f32) -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mut noise = FastNoise::seeded(rng.next_u64());
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(10);
        noise.set_fractal_gain(0.5);
        noise.set_fractal_lacunarity(3.0);
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
            ( to_cp437('▲'), RGB::from_f32(greenness, greenness, greenness) )
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