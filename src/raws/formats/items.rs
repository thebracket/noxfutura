use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Items {
    pub items: Vec<ItemDef>,
}

impl Items {
    pub fn new() -> Self {
        Self { items: Vec::new() }
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
    Component,
    ToolChopping,
    ToolDigging,
    ToolFarming,
}
