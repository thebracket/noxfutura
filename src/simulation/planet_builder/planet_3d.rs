use super::BlockType;
use crate::{
    geometry::{Degrees, Radians},
    simulation::*,
};

const LAT_STEP: f32 = 1.0;
const LON_STEP: f32 = 1.0;
const TEX_WIDTH: f32 = 0.0625;

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
        const CAPACITY: usize = ((360.0 / LON_STEP) * (180.0 / LAT_STEP)) as usize;
        PlanetMesh {
            vertices: Vec::with_capacity(CAPACITY),
            normals: Vec::with_capacity(CAPACITY),
            uv: Vec::with_capacity(CAPACITY),
        }
    }

    fn tex_idx(&self, n: f32) -> (f32, f32) {
        (n * TEX_WIDTH, (n + 1.0) * TEX_WIDTH)
    }

    fn get_texture_coords(&self, block_type: BlockType) -> (f32, f32) {
        match block_type {
            BlockType::Coastal => self.tex_idx(3.0),
            BlockType::Highlands => self.tex_idx(2.0),
            BlockType::Hills => self.tex_idx(5.0),
            BlockType::Marsh => self.tex_idx(6.0),
            BlockType::Mountains => self.tex_idx(4.0),
            BlockType::Plains => self.tex_idx(1.0),
            BlockType::Plateau => self.tex_idx(7.0),
            BlockType::SaltMarsh => self.tex_idx(6.0),
            BlockType::Water => self.tex_idx(0.0),
            BlockType::None => self.tex_idx(8.0),
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
        let (tx_min, tx_max) = self.get_texture_coords(BlockType::None);

        self.push_point(lat, lon, tx_min, 0.0, altitude);
        self.push_point(lat, lon + LON_STEP, tx_min, 1.0, altitude);
        self.push_point(lat + LAT_STEP, lon, tx_max, 0.0, altitude);

        self.push_point(lat, lon + LON_STEP, tx_min, 1.0, altitude);
        self.push_point(lat + LAT_STEP, lon + LON_STEP, tx_max, 1.0, altitude);
        self.push_point(lat + LAT_STEP, lon, tx_max, 0.0, altitude);
    }

    fn get_altitude(&self, planet: &super::Planet, x: usize, y: usize) -> f32 {
        let pidx = planet_idx(x % WORLD_WIDTH, y % WORLD_HEIGHT);
        ((planet.landblocks[pidx].height as f32 / 255.0) * 0.5) + 3.0
    }

    fn push_height_quad(&mut self, lat: f32, lon: f32, planet: &super::Planet) {
        let (tx_min, tx_max) = if self.get_altitude(planet, lon_to_x(lon), lat_to_y(lat)) > 3.3 {
            self.get_texture_coords(BlockType::Plains)
        } else {
            self.get_texture_coords(BlockType::Water)
        };

        let tl = self.get_altitude(planet, lon_to_x(lon), lat_to_y(lat));
        let tr = self.get_altitude(planet, lon_to_x(lon + LON_STEP), lat_to_y(lat));
        let bl = self.get_altitude(planet, lon_to_x(lon), lat_to_y(lat + LAT_STEP));
        let br = self.get_altitude(planet, lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP));

        self.push_point(lat, lon, tx_min, 0.0, tl);
        self.push_point(lat, lon + LON_STEP, tx_min, 1.0, tr);
        self.push_point(lat + LAT_STEP, lon, tx_max, 0.0, bl);

        self.push_point(lat, lon + LON_STEP, tx_min, 1.0, tr);
        self.push_point(lat + LAT_STEP, lon + LON_STEP, tx_max, 1.0, br);
        self.push_point(lat + LAT_STEP, lon, tx_max, 0.0, bl);
    }

    fn push_cat_quad(&mut self, lat: f32, lon: f32, planet: &super::Planet) {
        let (tx_min, tx_max) = self
            .get_texture_coords(planet.landblocks[planet_idx(lon_to_x(lon), lat_to_y(lat))].btype);

        let tl = self.get_altitude(planet, lon_to_x(lon), lat_to_y(lat));
        let tr = self.get_altitude(planet, lon_to_x(lon + LON_STEP), lat_to_y(lat));
        let bl = self.get_altitude(planet, lon_to_x(lon), lat_to_y(lat + LAT_STEP));
        let br = self.get_altitude(planet, lon_to_x(lon + LON_STEP), lat_to_y(lat + LAT_STEP));

        self.push_point(lat, lon, tx_min, 0.0, tl);
        self.push_point(lat, lon + LON_STEP, tx_min, 1.0, tr);
        self.push_point(lat + LAT_STEP, lon, tx_max, 0.0, bl);

        self.push_point(lat, lon + LON_STEP, tx_min, 1.0, tr);
        self.push_point(lat + LAT_STEP, lon + LON_STEP, tx_max, 1.0, br);
        self.push_point(lat + LAT_STEP, lon, tx_max, 0.0, bl);
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

        let mut lat = -90.0;
        let mut lon;
        while lat < 90.0 {
            lon = -180.0;
            while lon < 180.0 {
                self.push_height_quad(lat, lon, planet);

                lon += LON_STEP;
            }
            lat += LAT_STEP;
        }
    }

    pub fn with_category(&mut self, planet: &super::Planet) {
        self.vertices.clear();
        self.normals.clear();
        self.uv.clear();

        let mut lat = -90.0;
        let mut lon;
        while lat < 90.0 {
            lon = -180.0;
            while lon < 180.0 {
                self.push_cat_quad(lat, lon, planet);

                lon += LON_STEP;
            }
            lat += LAT_STEP;
        }
    }
}
