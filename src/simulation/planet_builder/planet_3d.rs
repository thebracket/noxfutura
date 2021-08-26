use bevy::math::Vec3;

use crate::{geometry::{Degrees, Radians}, simulation::*};

const LAT_STEP: f32 = 10.0;
const LON_STEP: f32 = 10.0;


pub fn sphere_vertex<A: Into<Radians>>(altitude: f32, lat: A, lon: A) -> (f32, f32, f32) {
    let rlat = lat.into();
    let rlon = lon.into();
    let sinlat = f32::sin(rlat.0);
    let coslat = f32::cos(rlat.0);
    let sinlon = f32::sin(rlon.0);
    let coslon = f32::cos(rlon.0);
    (
        altitude * coslat * coslon,
        altitude * coslat * sinlon,
        altitude * sinlat,
    )
}

pub struct PlanetMesh {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uv: Vec<[f32; 2]>,
}

impl PlanetMesh {
    pub fn new() -> Self {
        const CAPACITY : usize = ((360.0 / LON_STEP) * (180.0 / LAT_STEP)) as usize;
        PlanetMesh{
            vertices: Vec::with_capacity(CAPACITY),
            normals: Vec::with_capacity(CAPACITY),
            uv: Vec::with_capacity(CAPACITY),
        }
    }

    fn push_point(&mut self, lat: f32, lon: f32, tx: f32, ty: f32, altitude: f32) {
        let (a, b, c) = sphere_vertex(altitude, Degrees::new(lat), Degrees::new(lon));
        self.vertices.push([a, b, c]);
        //let n = Vec3::new(a, b, c).normalize();
        //self.normals.push([ n.x, n.y, n.z ]);
        self.normals.push([a, b, c]);
        self.uv.push([tx, ty]);
    }

    fn push_flat_quad(&mut self, lat: f32, lon: f32, altitude: f32) {
        self.push_point(lat, lon, 0.0, 0.0, altitude);
        self.push_point(lat, lon + LON_STEP, 0.0, 0.5, altitude);
        self.push_point(lat + LAT_STEP, lon, 0.5, 0.0, altitude);

        self.push_point(lat, lon + LON_STEP, 0.0, 0.5, altitude);
        self.push_point(lat + LAT_STEP, lon + LON_STEP, 0.5, 0.5, altitude);
        self.push_point(lat + LAT_STEP, lon, 0.5, 0.0, altitude);
    }

    fn get_altitude(&self, planet: &super::Planet, x: usize, y: usize) -> f32 {
        let pidx = planet_idx(x % WORLD_WIDTH, y % WORLD_HEIGHT);
        ((planet.landblocks[pidx].height as f32 / 256.0) * 0.25) + 3.0
    }

    fn push_height_quad(&mut self, lat: f32, lon: f32, planet: &super::Planet, x: usize, y: usize) {
        self.push_point(lat, lon, 0.0, 0.0, self.get_altitude(planet, x, y));
        self.push_point(lat, lon + LON_STEP, 0.0, 0.5, self.get_altitude(planet, x+LON_STEP as usize, y));
        self.push_point(lat + LAT_STEP, lon, 0.5, 0.0, self.get_altitude(planet, x, y+LAT_STEP as usize));

        self.push_point(lat, lon + LON_STEP, 0.0, 0.5, self.get_altitude(planet, x+LON_STEP as usize, y));
        self.push_point(lat + LAT_STEP, lon + LON_STEP, 0.5, 0.5, self.get_altitude(planet, x+LON_STEP as usize, y+LAT_STEP as usize));
        self.push_point(lat + LAT_STEP, lon, 0.5, 0.0, self.get_altitude(planet, x, y+LAT_STEP as usize));
    }

    pub fn totally_round(&mut self, altitude: f32) {
        self.vertices.clear();
        self.normals.clear();
        self.uv.clear();

        let mut lat = -90.0;
        let mut lon;
        while lat < 90.0 {
            lon = -180.0;
            while lon < 180.0 {
                self.push_flat_quad(lat, lon, altitude);

                lon += LON_STEP;
            }
            lat += LAT_STEP;
        }
    }

    pub fn with_altitude(&mut self, planet: &super::Planet) {
        self.vertices.clear();
        self.normals.clear();
        self.uv.clear();

        let mut y = 0;
        let mut lat = -90.0;
        let mut lon;
        while lat < 90.0 {
            lon = -180.0;
            let mut x = 0;
            while lon < 180.0 {
                self.push_height_quad(lat, lon, planet, x, y);

                lon += LON_STEP;
                x += LON_STEP as usize;
            }
            lat += LAT_STEP;
            y += LAT_STEP as usize;
        }
    }
}