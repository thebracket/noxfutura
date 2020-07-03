use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VoxelModels {
    pub vox: Vec<VoxelModel>,
}

impl VoxelModels {
    pub fn new() -> Self {
        Self { vox: Vec::new() }
    }

    pub fn get_model_idx(&self, tag: &str) -> usize {
        for (i, b) in self.vox.iter().enumerate() {
            if b.tag == tag {
                return i;
            }
        }
        println!("No vox match for {}", tag);
        0
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VoxelModel {
    pub tag: String,
    pub file: String,
}

pub fn load_vox() -> VoxelModels {
    let mat_path = "resources/raws/vox.ron";
    let f = File::open(&mat_path).expect("Failed opening file");
    let vox: VoxelModels = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load vox list: {}", e);
            std::process::exit(1);
        }
    };
    vox
}
