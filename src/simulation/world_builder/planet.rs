use crate::types::{Degrees, Radians};

const WORLD_WIDTH: usize = 4096;
const WORLD_HEIGHT: usize = 4096;

pub struct Planet {
    pub bounds: PlanetBounds,
    pub landblock_dimensions: (usize, usize),
    pub noise_seed: u64,
}

impl Planet {
    pub fn whole_world(noise_seed: u64) -> Self {
        Self {
            bounds: PlanetBounds::whole_world(),
            landblock_dimensions: (WORLD_WIDTH, WORLD_HEIGHT),
            noise_seed,
        }
    }

    #[allow(dead_code)]
    pub fn partial_world(
        noise_seed: u64,
        north: Degrees,
        south: Degrees,
        east: Degrees,
        west: Degrees,
    ) -> Self {
        Self {
            bounds: PlanetBounds::partial_world(north, south, east, west),
            landblock_dimensions: (WORLD_WIDTH, WORLD_HEIGHT),
            noise_seed,
        }
    }
}

pub struct PlanetBounds {
    pub west: Degrees,
    pub east: Degrees,
    pub north: Degrees,
    pub south: Degrees,
}

impl PlanetBounds {
    pub fn whole_world() -> Self {
        Self {
            west: Degrees::new(-180.0),
            east: Degrees::new(180.0),
            north: Degrees::new(-90.0),
            south: Degrees::new(90.0),
        }
    }

    pub fn partial_world(north: Degrees, south: Degrees, east: Degrees, west: Degrees) -> Self {
        Self {
            north,
            south,
            east,
            west,
        }
    }
}

#[allow(dead_code)]
pub fn average_temperature_by_latitude(lat: Degrees) -> f32 {
    // Source: https://davidwaltham.com/global-warming-model/
    const AVERAGE_EQUATORIAL_C: f32 = 30.0;
    const A: f32 = 5.0; // Based on current data
    let lat_rad: Radians = lat.into();
    let lat_sin_squared = lat_rad.0.sin() * lat_rad.0.sin();
    AVERAGE_EQUATORIAL_C - (A * lat_sin_squared)
}

#[allow(dead_code)]
pub fn average_precipitation_mm_by_latitude(lat: Degrees) -> f32 {
    // Mangled from https://i.stack.imgur.com/YBgot.png
    const PEAK: f32 = 2000.0;
    let fudge = if (lat.0 > -50.0 && lat.0 < -5.0) || (lat.0 < 50.0 && lat.0 > 5.0) {
        400.0
    } else {
        0.0
    };
    let lat_rad: Radians = lat.into();
    let lat_sin_squared = lat_rad.0.sin() * lat_rad.0.sin();
    PEAK - (lat_sin_squared * PEAK) - fudge
}

#[allow(dead_code)]
pub fn temperature_decrease_by_altitude(altitude_meters: f32) -> f32 {
    altitude_meters * 6.5
}
