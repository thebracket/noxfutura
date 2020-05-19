use super::{ChunkType, chunk_idx, CHUNK_SIZE};
use crate::region::{Region, TileType};
use crate::utils::mapidx;

#[derive(Clone)]
pub enum Primitive {
    Cube {x: usize, y: usize, z: usize, w: usize, h: usize, d: usize}
}

#[derive(Clone)]
pub struct Chunk {
    pub t : ChunkType,
    pub idx : usize,
    pub base : (usize, usize, usize)
}

impl Chunk {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self{
            t : ChunkType::Empty,
            idx : chunk_idx(x, y, z),
            base : ( x * CHUNK_SIZE, y * CHUNK_SIZE, z * CHUNK_SIZE )
        }
    }

    pub fn iter(&self) -> ChunkIter {
        ChunkIter{
            base : self.base,
            current : (0,0,0),
            done: false
        }
    }

    pub fn rebuild(&mut self, region: &Region) {
        let len = self.iter().count();
        let mut count_empty = 0;
        let mut count_solid = 0;
        self.iter().for_each(|idx| {
            let idx = mapidx(idx.0, idx.1, idx.2);
            match region.tile_types[idx] {
                TileType::Solid => count_solid += 1,
                TileType::Empty => count_empty += 1,
                _ => {}
            }
        });
        /*
        println!("Base chunk at {:?}", self.base);
        println!("X Range: {}..{}",
            self.iter().map(|(x,_,_)| x).min().unwrap(),
            self.iter().map(|(x,_,_)| x).max().unwrap()
        );
        println!("Y Range: {}..{}",
            self.iter().map(|(_,x,_)| x).min().unwrap(),
            self.iter().map(|(_,x,_)| x).max().unwrap()
        );
        println!("Z Range: {}..{}",
            self.iter().map(|(_,_,x)| x).min().unwrap(),
            self.iter().map(|(_,_,x)| x).max().unwrap()
        );
        println!("Total: {}, Empty: {}, Solid: {}", len, count_empty, count_solid);
        */
        if count_empty == len {
            self.t = ChunkType::Empty;
        } else if count_solid == len {
            self.t = ChunkType::Solid;
        } else {
            self.t = ChunkType::Partial;
        }
    }

    pub fn geometry(&self, region: &Region) -> Option<Vec<Primitive>> {
        match self.t {
            ChunkType::Partial | ChunkType::Solid => {
                let mut p = Vec::new();
                let mut cubes = Vec::new();
                self.iter().for_each(|pos| {
                    let idx = mapidx(pos.0, pos.1, pos.2);
                    match region.tile_types[idx] {
                        TileType::Solid => {
                            //println!("{},{},{} = {}", pos.0, pos.1, pos.2, idx);
                            cubes.push(idx);
                        },
                        _ => {}
                    }
                });
                p.append(&mut super::greedy::greedy_cubes(cubes));
                Some(p)
            },
            _ => None
        }
    }
}

pub struct ChunkIter {
    base : (usize, usize, usize),
    current: (usize, usize, usize),
    done: bool
}

impl Iterator for ChunkIter {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<(usize, usize, usize)> {
        if self.done {
            return None;
        }
        let result = (
            self.current.0 + self.base.0,
            self.current.1 + self.base.1,
            self.current.2 + self.base.2,
        );

        self.current.0 += 1;
        if self.current.0 == CHUNK_SIZE {
            self.current.0 = 0;

            self.current.1 += 1;
            if self.current.1 == CHUNK_SIZE {
                self.current.1 = 0;

                self.current.2 += 1;
                if self.current.2 == CHUNK_SIZE {
                    self.done = true;
                }
            }
        }

        Some(result)
    }
}