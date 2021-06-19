use crate::types::{Degrees, Radians};

const WORLD_WIDTH : usize = 256;
const WORLD_HEIGHT: usize = 256;

pub struct Planet {
    bounds: PlanetBounds
}

impl Planet {
    pub fn whole_world() -> Self {
        Self {
            bounds: PlanetBounds::whole_world(),
        }
    }

    pub fn partial_world(north: Degrees, south: Degrees, east: Degrees, west: Degrees) -> Self {
        Self {
            bounds: PlanetBounds::partial_world(north, south, east, west),
        }
    }

    pub fn landblock_idx(&self, lat: Degrees, lon: Degrees) -> usize {
        let lat_span = f32::abs(self.bounds.south.0 - self.bounds.north.0);
        let lon_span = f32::abs(self.bounds.east.0 - self.bounds.west.0);
        let y_extent = (lat.0 - f32::min(self.bounds.north.0, self.bounds.south.0)) / lat_span;
        let x_extent = (lon.0 - f32::min(self.bounds.west.0, self.bounds.east.0)) / lon_span;
        let y = y_extent as usize * WORLD_HEIGHT;
        let x = x_extent as usize * WORLD_WIDTH;
        (y * WORLD_WIDTH) + x
    }
}

pub struct PlanetBounds {
    pub west: Degrees,
    pub east: Degrees,
    pub north: Degrees,
    pub south: Degrees
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
        Self { north, south, east, west }
    }
}

pub fn average_temperature_by_latitude(lat: Degrees) -> f32 {
    // Source: https://davidwaltham.com/global-warming-model/
    const AVERAGE_EQUATORIAL_C : f32 = 30.0;
    const A : f32 = 5.0; // Based on current data
    let lat_rad : Radians = lat.into();
    let lat_sin_squared = lat_rad.0.sin() * lat_rad.0.sin();
    AVERAGE_EQUATORIAL_C - (A * lat_sin_squared)
}

pub fn average_precipitation_mm_by_latitude(lat: Degrees) -> f32 {
    // Mangled from https://i.stack.imgur.com/YBgot.png
    const PEAK : f32 = 2000.0;
    let fudge = if (lat.0 > -50.0 && lat.0 < -5.0) || (lat.0 < 50.0 && lat.0 > 5.0) { 400.0 } else { 0.0 };
    let lat_rad : Radians = lat.into();
    let lat_sin_squared = lat_rad.0.sin() * lat_rad.0.sin();
    PEAK - (lat_sin_squared * PEAK) - fudge
}

pub fn temperature_decrease_by_altitude(altitude_meters: f32) -> f32 {
    altitude_meters * 6.5
}