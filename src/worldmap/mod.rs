use bracket_random::prelude::*;
use bracket_noise::prelude::*;
use crate::engine::VertexBuffer;

pub struct WorldMap {
    noise : FastNoise
}

impl WorldMap {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mut noise = FastNoise::seeded(rng.next_u64());
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(10);
        noise.set_fractal_gain(0.5);
        noise.set_fractal_lacunarity(3.0);
        noise.set_frequency(0.01);

        Self{
            noise
        }
    }

    fn sphere_vertex(&self, altitude: f32, lat: f32, lon: f32) -> (f32, f32, f32) {
        (
            altitude * f32::cos(lat) * f32::cos(lon),
            altitude * f32::cos(lat) * f32::sin(lon),
            altitude * f32::sin(lat)
        )
    }

    fn add_point(&self, lat: f32, lon:f32, buffer: &mut VertexBuffer<f32>) {
        let base_coords = self.sphere_vertex(100.0, lat as f32, lon as f32);
        let altitude = self.noise.get_noise3d(base_coords.0, base_coords.1, base_coords.2);
        let sphere_coords = self.sphere_vertex(0.5 + (altitude / 50.0), lat as f32, lon as f32);
        buffer.add3(sphere_coords.0, sphere_coords.1, sphere_coords.2);

        if altitude < 0.0 {
            buffer.add3(0.0, 0.0, 1.0 + altitude);
        } else {
            buffer.add3(0.0, 0.2 + altitude, 0.0);
        }
    }

    pub fn build_vertex_buffer(&self) -> VertexBuffer<f32> {
        let mut buffer = VertexBuffer::new(&[3, 3]);
        const STEP : f32 = 0.25;

        let mut lat = -180.0;
        let mut lon = -90.0;

        while lat < 180.0 {
            while lon < 90.0 {
                self.add_point(lat, lon, &mut buffer);
                self.add_point(lat + STEP, lon, &mut buffer);
                self.add_point(lat + STEP, lon + STEP, &mut buffer);
                lon += STEP;
            }
            lon = -90.0;
            lat += STEP;
        }

        buffer
    }
}