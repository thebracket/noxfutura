use serde::{Deserialize, Serialize};

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
