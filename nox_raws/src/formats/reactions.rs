use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Reactions {
    pub reactions: Vec<ReactionDef>,
}

impl Reactions {
    pub fn new() -> Self {
        Self {
            reactions: Vec::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReactionDef {
    pub name : String,
    pub workshop : String,
    pub difficulty: i32,
    pub automatic: bool,
    pub skill : String,
    pub inputs : Vec<ReactionItem>,
    pub outputs : Vec<ReactionItem>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReactionItem {
    pub tag: String,
    pub qty: i32
}

pub fn load_reactions() -> Reactions {
    let mat_path = "resources/raws/reactions.ron";
    let f = File::open(&mat_path).expect("Failed opening file");
    let reactions: Reactions = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load reactions: {}", e);
            std::process::exit(1);
        }
    };
    reactions
}
