use super::formats::MaterialDef;
use std::collections::HashMap;

pub struct MaterialMap {
    map: HashMap<usize, usize>, // Map containing key as material index, value as palette entry
}

impl MaterialMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn build(&mut self, materials: &[MaterialDef], palette: &crate::modes::Palette) {
        self.map.clear();

        materials.iter().enumerate().for_each(|(idx, m)| {
            self.map
                .insert(idx, palette.find_palette(m.tint.0, m.tint.1, m.tint.2));
        });
        //self.map.insert(255, palette.find_palette(0.0, 1.0, 0.0));
        println!("{:?}", palette.find_palette(0.0, 0.0, 1.0));
    }

    pub fn get(&self, idx: usize) -> &usize {
        if self.map.contains_key(&idx) {
            self.map.get(&idx).unwrap()
        } else {
            println!("{:#?}", self.map);
            panic!("Material index {} does not exist", idx);
        }
    }
}
