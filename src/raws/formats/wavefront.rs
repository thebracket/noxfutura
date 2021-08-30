use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WavefrontModels {
    pub models: Vec<WavefrontObj>,
    pub colors: Vec<MappedColor>,
}

impl WavefrontModels {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
            colors: Vec::new(),
        }
    }

    pub fn get_model_idx(&self, tag: &str) -> usize {
        for (i, b) in self.models.iter().enumerate() {
            if b.tag == tag {
                return i;
            }
        }
        println!("No wavefront OBJ match for {}", tag);
        0
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WavefrontObj {
    pub tag: String,
    pub file: String,
    pub scale: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MappedColor {
    pub tag: String,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}