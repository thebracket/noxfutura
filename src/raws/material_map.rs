use std::collections::HashMap;
use super::formats::MaterialDef;

pub struct MaterialMap {
    map : HashMap<usize, MappedMaterial>
}

impl MaterialMap {
    pub fn new() -> Self {
        Self {
            map : HashMap::new()
        }
    }

    pub fn build(&mut self, matmap : &HashMap::<String, usize>, materials: &[MaterialDef]) {
        self.map.clear();
        for (i,m) in materials.iter().enumerate() {
            self.map.insert(
                i,
                MappedMaterial{
                    texture : matmap[&m.texture],
                    constructed : matmap[&m.constructed],
                    floor: matmap[&m.floor],
                    floor_constructed: matmap[&m.floor_constructed]
                }
            );
        }
        println!("Mapped materials: {:#?}", self.map);
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
    pub texture: usize,
    pub constructed: usize,
    pub floor: usize,
    pub floor_constructed: usize,
}