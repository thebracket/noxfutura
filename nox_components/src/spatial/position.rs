use crate::prelude::*;
use crate::IdentityTag;
use bengine::geometry::Point3;
use bengine::uv::vec::Vec3;
use nox_spatial::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub loc: Location,
    pub region_idx: usize,
    pub dimensions: (i32, i32, i32),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Location {
    Tile { idx: usize },
    Stored { container: usize },
    Carried { by: usize },
    Worn { by: usize },
}

impl Position {
    pub fn with_tile_idx(idx: usize, region_idx: usize, dimensions: (i32, i32, i32)) -> Self {
        Self {
            loc: Location::Tile { idx },
            region_idx,
            dimensions,
        }
    }

    pub fn with_tile(
        x: usize,
        y: usize,
        z: usize,
        region_idx: usize,
        dimensions: (i32, i32, i32),
    ) -> Self {
        Self {
            loc: Location::Tile {
                idx: mapidx(x, y, z),
            },
            region_idx,
            dimensions,
        }
    }

    pub fn worn(wearer: usize) -> Self {
        Self {
            loc: Location::Worn { by: wearer },
            region_idx: 0,
            dimensions: (1, 1, 1),
        }
    }

    pub fn stored(container: usize) -> Self {
        Self {
            loc: Location::Stored { container },
            region_idx: 0,
            dimensions: (1, 1, 1),
        }
    }

    pub fn carried(by: usize) -> Self {
        Self {
            loc: Location::Carried { by },
            region_idx: 0,
            dimensions: (1, 1, 1),
        }
    }

    #[inline]
    pub fn exact_position(&self, x: usize, y: usize, z: usize) -> bool {
        let test_idx = mapidx(x, y, z);
        match &self.loc {
            Location::Tile { idx } => test_idx == *idx,
            _ => false,
        }
    }

    #[inline]
    pub fn as_point3(&self) -> Point3 {
        match &self.loc {
            Location::Tile { idx } => {
                let (x, y, z) = idxmap(*idx);
                Point3::new(x, y, z)
            }
            _ => Point3::new(0, 0, 0),
        }
    }

    #[inline]
    pub fn as_xzy_f32(&self) -> [f32; 3] {
        match &self.loc {
            Location::Tile { idx } => {
                let (x, y, z) = idxmap(*idx);
                [x as f32, z as f32, y as f32]
            }
            _ => [0.0, 0.0, 0.0],
        }
    }

    #[inline]
    pub fn as_point3_only_tile(&self) -> Option<Point3> {
        match &self.loc {
            Location::Tile { idx } => {
                let (x, y, z) = idxmap(*idx);
                Some(Point3::new(x, y, z))
            }
            _ => None,
        }
    }

    pub fn effective_location(&self, ecs: &legion::World) -> usize {
        use legion::*;
        match self.loc {
            Location::Tile { idx } => idx,
            Location::Stored { container } => <(&Position, &IdentityTag)>::query()
                .iter(ecs)
                .filter(|(_, id)| id.0 == container)
                .map(|(pos, _)| *pos)
                .nth(0)
                .unwrap()
                .effective_location(ecs),
            Location::Carried { by } => <(&Position, &IdentityTag)>::query()
                .iter(ecs)
                .filter(|(_, id)| id.0 == by)
                .map(|(pos, _)| *pos)
                .nth(0)
                .unwrap()
                .effective_location(ecs),
            Location::Worn { by } => <(&Position, &IdentityTag)>::query()
                .iter(ecs)
                .filter(|(_, id)| id.0 == by)
                .map(|(pos, _)| *pos)
                .nth(0)
                .unwrap()
                .effective_location(ecs),
        }
    }

    pub fn effective_location_sw(&self, ecs: &legion::world::SubWorld) -> usize {
        use legion::*;
        match self.loc {
            Location::Tile { idx } => idx,
            Location::Stored { container } => <(&Position, &IdentityTag)>::query()
                .iter(ecs)
                .filter(|(_, id)| id.0 == container)
                .map(|(pos, _)| *pos)
                .nth(0)
                .unwrap()
                .effective_location_sw(ecs),
            Location::Carried { by } => <(&Position, &IdentityTag)>::query()
                .iter(ecs)
                .filter(|(_, id)| id.0 == by)
                .map(|(pos, _)| *pos)
                .nth(0)
                .unwrap()
                .effective_location_sw(ecs),
            Location::Worn { by } => <(&Position, &IdentityTag)>::query()
                .iter(ecs)
                .filter(|(_, id)| id.0 == by)
                .map(|(pos, _)| *pos)
                .nth(0)
                .unwrap()
                .effective_location_sw(ecs),
        }
    }

    #[inline]
    pub fn set_tile_loc(&mut self, pos: &(usize, usize, usize)) {
        self.loc = Location::Tile {
            idx: mapidx(pos.0, pos.1, pos.2),
        }
    }

    #[inline]
    pub fn as_vec3(&self) -> Vec3 {
        match self.loc {
            Location::Tile { idx } => {
                let (x, y, z) = idxmap(idx);
                Vec3::new(x as f32, y as f32, z as f32)
            }
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }

    #[inline]
    pub fn as_vec3_glspace(&self) -> Vec3 {
        match self.loc {
            Location::Tile { idx } => {
                let (x, y, z) = idxmap(idx);
                Vec3::new(x as f32, z as f32, y as f32)
            }
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }

    #[inline]
    pub fn apply_delta(&mut self, dx: i32, dy: i32, dz: i32) {
        match &mut self.loc {
            Location::Tile { idx } => {
                let (mut x, mut y, mut z) = idxmap(*idx);
                x = (x as i32 + dx) as usize;
                y = (y as i32 + dy) as usize;
                z = (z as i32 + dz) as usize;
                *idx = mapidx(x, y, z);
            }
            _ => {}
        }
    }

    #[inline]
    pub fn contains_point(&self, point: &(usize, usize, usize)) -> bool {
        match &self.loc {
            Location::Tile { idx } => {
                let (tx, ty, tz) = idxmap(*idx);

                if self.dimensions.0 == 1 && self.dimensions.1 == 1 && self.dimensions.2 == 1 {
                    point.0 == tx && point.1 == ty && point.2 == tz
                } else if self.dimensions.0 == 3 && self.dimensions.1 == 3 {
                    for x in tx - 1..tx + self.dimensions.0 as usize - 1 {
                        for y in ty - 1..ty + self.dimensions.1 as usize - 1 {
                            for z in tz..tz + self.dimensions.2 as usize {
                                if point.0 == x && point.1 == y && point.2 == z {
                                    return true;
                                }
                            }
                        }
                    }
                    false
                } else {
                    for x in tx..tx + self.dimensions.0 as usize {
                        for y in ty..ty + self.dimensions.1 as usize {
                            for z in tz..tz + self.dimensions.2 as usize {
                                if point.0 == x && point.1 == y && point.2 == z {
                                    return true;
                                }
                            }
                        }
                    }
                    false
                }
            }
            _ => false,
        }
    }

    #[inline]
    pub fn get_idx(&self) -> usize {
        match &self.loc {
            Location::Tile { idx } => *idx,
            _ => 0,
        }
    }

    #[inline]
    pub fn is_in_container(&self, container_id: usize) -> bool {
        match &self.loc {
            Location::Stored { container } => *container == container_id,
            _ => false,
        }
    }

    #[inline]
    pub fn is_carried(&self, by_id: usize) -> bool {
        match &self.loc {
            Location::Carried { by } => *by == by_id,
            _ => false,
        }
    }

    #[inline]
    pub fn to_carried(&mut self, by: usize) {
        self.loc = Location::Carried { by };
    }

    #[inline]
    pub fn to_stored(&mut self, container: usize) {
        self.loc = Location::Stored { container };
    }

    #[inline]
    pub fn to_ground(&mut self, idx: usize) {
        self.loc = Location::Tile { idx };
    }

    #[inline]
    pub fn to_worn(&mut self, by: usize) {
        self.loc = Location::Worn { by };
    }
}
