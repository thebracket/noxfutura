use crate::planet::planet_render::*;
use crate::planet::Planet;
use bengine::FloatBuffer;
use parking_lot::Mutex;

const ALTITUDE_DIVISOR: f32 = 4096.0;

lazy_static! {
    pub static ref WORLDGEN_RENDER: Mutex<WorldGenPlanetRender> =
        Mutex::new(WorldGenPlanetRender::new());
}

pub struct WorldGenPlanetRender {
    pub vertex_buffer: FloatBuffer<f32>,
    pub needs_update: bool,
}

impl WorldGenPlanetRender {
    fn new() -> Self {
        let mut wgpr = Self {
            vertex_buffer: FloatBuffer::new(&[3, 3, 4], 1000, bengine::gpu::BufferUsage::VERTEX),
            needs_update: false,
        };
        build_blank_planet(&mut wgpr.vertex_buffer);
        wgpr
    }

    pub fn planet_with_altitude(&mut self, planet: Planet) {
        self.vertex_buffer.clear();
        all_planet_points(|l| {
            add_point(
                &mut self.vertex_buffer,
                l.0,
                l.1,
                planet.landblocks[l.2].height as f32 / ALTITUDE_DIVISOR,
                &altitude_to_color(planet.landblocks[l.2].height),
            );
        });
        self.needs_update = true;
    }

    pub fn planet_with_category(&mut self, planet: &Planet) {
        self.vertex_buffer.clear();

        all_planet_points(|l| {
            add_point(
                &mut self.vertex_buffer,
                l.0,
                l.1,
                planet.landblocks[l.2].height as f32 / ALTITUDE_DIVISOR,
                &landblock_to_color(&planet.landblocks[l.2]),
            );
        });
        self.needs_update = true;
    }
}
