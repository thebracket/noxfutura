use serde::{Deserialize, Serialize};

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

    pub fn reaction_by_tag(&self, tag: &str) -> Option<ReactionDef> {
        for b in self.reactions.iter() {
            if b.name == tag {
                return Some(b.clone());
            }
        }
        println!("Unable to find reaction tag: {}", tag);
        None
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReactionDef {
    pub name: String,
    pub workshop: String,
    pub difficulty: i32,
    pub automatic: bool,
    pub skill: String,
    pub inputs: Vec<ReactionItem>,
    pub outputs: Vec<ReactionItem>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReactionItem {
    pub tag: String,
    pub qty: i32,
}
