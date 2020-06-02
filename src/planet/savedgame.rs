use serde::{Deserialize, Serialize};
use super::Planet;
use crate::region::Region;
use std::fs::File;

#[derive(Clone, Serialize, Deserialize)]
pub struct SavedGame {
    pub planet : Planet,
    pub current_region : Region
}

pub fn save_world(state : SavedGame) {
    use std::io::Write;
    let mut world_file = File::create("world.dat").unwrap();
    let tmp = ron::to_string(&state).unwrap();
    let mem_vec = tmp.as_bytes();
    let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    e.write_all(&mem_vec).expect("Compression fail");
    let compressed_bytes = e.finish().unwrap();
    let mut pos = 0;
    while pos < compressed_bytes.len() {
        let bytes_written = world_file.write(&compressed_bytes[pos..]).unwrap();
        pos += bytes_written;
    }
}