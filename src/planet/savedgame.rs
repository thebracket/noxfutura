use super::Planet;
use super::Region;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Clone, Serialize, Deserialize)]
pub struct SavedGame {
    pub planet: Planet,
    pub current_region: Region,
    pub ecs_text : String
}

pub fn save_world(state: SavedGame) {
    use std::io::Write;
    let mut world_file = File::create("world.dat").unwrap();
    let tmp = ron::to_string(&state).unwrap();
    let mem_vec = tmp.as_bytes();
    let compressed_bytes = miniz_oxide::deflate::compress_to_vec(&mem_vec, 6);
    world_file.write_all(&compressed_bytes).expect("Unable to write file data");
}

pub fn load_game() -> SavedGame {
    use std::io::Read;
    use std::path::Path;
    let savepath = Path::new("world.dat");
    if !savepath.exists() {
        panic!("Saved game doesn't exist");
    }

    println!("LOADFILE");
    let mut f = File::open(&savepath).expect("Unable to open file");
    let mut buffer = Vec::<u8>::new();
    f.read_to_end(&mut buffer).expect("Unable to read file");
    println!("DECOMPRESS");
    let raw_bytes = miniz_oxide::inflate::decompress_to_vec(&buffer).expect("Unable to decompress file");
    let s = std::str::from_utf8(&raw_bytes).unwrap().to_string();
    println!("DESERIALIZE");

    let saved: crate::planet::SavedGame = ron::from_str(&s).unwrap();
    println!("DONE");
    saved
}
