use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Items {
    pub items: Vec<ItemDef>,
}

impl Items {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn item_by_tag(&self, tag: &str) -> Option<&ItemDef> {
        for b in self.items.iter() {
            if b.tag == tag {
                return Some(b);
            }
        }
        println!("Unable to find item tag: {}", tag);
        None
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemDef {
    pub tag: String,
    pub name: String,
    pub vox: String,
    pub item_type: Vec<ItemDefType>,
    pub description: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ItemDefType {
    Component, ToolChopping, ToolDigging, ToolFarming
}

pub fn load_items() -> Items {
    let mat_path = "resources/raws/items.ron";
    let f = File::open(&mat_path).expect("Failed opening file");
    let items: Items = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load items: {}", e);
            std::process::exit(1);
        }
    };
    items
}
