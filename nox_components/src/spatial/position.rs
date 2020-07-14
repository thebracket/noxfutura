use crate::prelude::*;
use bracket_geometry::prelude::Point3;
use nox_spatial::*;

// Change to an enum and remove the storage/carried/etc options. Give helpful code.
// ca3f2ce0-9c8e-4abe-a3c6-12098b1b016a

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "ca3f2ce0-9c8e-4abe-a3c6-12098b1b016a"]
pub struct Position {
    pub loc: Location,
    pub region_idx: usize,
    pub dimensions: (i32, i32, i32),
}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "a9c5c6a1-a175-4a4f-9165-02e06689a9c4"]
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

    #[inline(always)]
    pub fn exact_position(&self, x: usize, y: usize, z: usize) -> bool {
        let test_idx = mapidx(x, y, z);
        match &self.loc {
            Location::Tile { idx } => test_idx == *idx,
            _ => false,
        }
    }

    pub fn as_point3(&self) -> Point3 {
        match &self.loc {
            Location::Tile { idx } => {
                let (x, y, z) = idxmap(*idx);
                Point3::new(x, y, z)
            }
            _ => Point3::new(0, 0, 0),
        }
    }

    pub fn as_xzy_f32(&self) -> [f32; 3] {
        match &self.loc {
            Location::Tile { idx } => {
                let (x, y, z) = idxmap(*idx);
                [x as f32, z as f32, y as f32]
            }
            _ => [0.0, 0.0, 0.0],
        }
    }

    pub fn as_point3_only_tile(&self) -> Option<Point3> {
        match &self.loc {
            Location::Tile { idx } => {
                let (x, y, z) = idxmap(*idx);
                Some(Point3::new(x, y, z))
            }
            _ => None,
        }
    }

    pub fn effective_location(&self, ecs: &legion::prelude::World) -> usize {
        use legion::prelude::*;
        match self.loc {
            Location::Tile { idx } => idx,
            Location::Stored { container } => <(Read<Identity>, Read<Position>)>::query()
                .iter(ecs)
                .filter(|(id, _)| id.id == container)
                .map(|(_, pos)| *pos)
                .nth(0)
                .unwrap()
                .effective_location(ecs),
            Location::Carried { by } => <(Read<Identity>, Read<Position>)>::query()
                .iter(ecs)
                .filter(|(id, _)| id.id == by)
                .map(|(_, pos)| *pos)
                .nth(0)
                .unwrap()
                .effective_location(ecs),
            Location::Worn { by } => <(Read<Identity>, Read<Position>)>::query()
                .iter(ecs)
                .filter(|(id, _)| id.id == by)
                .map(|(_, pos)| *pos)
                .nth(0)
                .unwrap()
                .effective_location(ecs),
        }
    }

    pub fn set_tile_loc(&mut self, pos: &(usize, usize, usize)) {
        self.loc = Location::Tile {
            idx: mapidx(pos.0, pos.1, pos.2),
        }
    }

    pub fn as_vec3(&self) -> ultraviolet::Vec3 {
        use ultraviolet::Vec3;
        match self.loc {
            Location::Tile { idx } => {
                let (x, y, z) = idxmap(idx);
                Vec3::new(x as f32, y as f32, z as f32)
            }
            _ => Vec3::zero(),
        }
    }

    pub fn as_vec3_glspace(&self) -> ultraviolet::Vec3 {
        use ultraviolet::Vec3;
        match self.loc {
            Location::Tile { idx } => {
                let (x, y, z) = idxmap(idx);
                Vec3::new(x as f32, z as f32, y as f32)
            }
            _ => Vec3::zero(),
        }
    }

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

    pub fn get_idx(&self) -> usize {
        match &self.loc {
            Location::Tile { idx } => *idx,
            _ => 0,
        }
    }

    pub fn is_in_container(&self, container_id: usize) -> bool {
        match &self.loc {
            Location::Stored { container } => *container == container_id,
            _ => false,
        }
    }

    pub fn to_carried(&mut self, by: usize) {
        self.loc = Location::Carried{ by };
    }

    pub fn to_stored(&mut self, container: usize) {
        self.loc = Location::Stored{ container };
    }

    pub fn to_ground(&mut self, idx: usize) {
        self.loc = Location::Tile{ idx };
    }

    pub fn to_worn(&mut self, by: usize) {
        self.loc = Location::Worn{ by };
    }
}
