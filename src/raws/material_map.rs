use super::formats::MaterialDef;
use std::collections::HashMap;

pub struct MaterialMap {
    map: HashMap<usize, MappedMaterial>,
    pub bark_id: usize,
    pub leaf_id: usize,
    pub water_id: usize,
    pub grass_id: usize,
}

impl MaterialMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            bark_id: 0,
            leaf_id: 0,
            water_id: 0,
            grass_id: 0,
        }
    }

    pub fn build(&mut self, matmap: &HashMap<String, usize>, materials: &[MaterialDef]) {
        self.map.clear();
        for (i, m) in materials.iter().enumerate() {
            self.map.insert(
                i,
                MappedMaterial {
                    texture: MappedTexture {
                        texture: matmap[&m.texture],
                        tint: m.tint,
                    },
                    constructed: MappedTexture {
                        texture: matmap[&m.constructed],
                        tint: m.tint,
                    },
                    floor: MappedTexture {
                        texture: matmap[&m.floor],
                        tint: m.tint,
                    },
                    floor_constructed: MappedTexture {
                        texture: matmap[&m.floor_constructed],
                        tint: m.tint,
                    },
                },
            );
        }

        // Map static materials
        for (k, v) in matmap.iter() {
            if k == "bark" {
                self.bark_id = *v
            }
            if k == "leaf" {
                self.leaf_id = *v
            }
            if k == "water" {
                self.water_id = *v
            }
            if k == "grass" {
                self.grass_id = *v
            }
        }
    }

    pub fn get(&self, idx: usize) -> &MappedMaterial {
        if self.map.contains_key(&idx) {
            self.map.get(&idx).unwrap()
        } else {
            panic!("Material index {} does not exist", idx);
        }
    }
}

#[derive(Debug)]
pub struct MappedMaterial {
    pub texture: MappedTexture,
    pub constructed: MappedTexture,
    pub floor: MappedTexture,
    pub floor_constructed: MappedTexture,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MappedTexture {
    pub texture: usize,
    pub tint: (f32, f32, f32),
}
