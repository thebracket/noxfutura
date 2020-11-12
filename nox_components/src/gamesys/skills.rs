use crate::prelude::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Skills(pub HashMap<Skill, i32>);

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Skill {
    Lumberjack,
    Mining
}

impl Skills {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get_skill(&self, skill: Skill) -> i32 {
        if let Some(n) = self.0.get(&skill) {
            *n
        } else {
            0
        }
    }

    pub fn improve_skill(&mut self, skill: Skill, by: i32) {
        if let Some(n) = self.0.get_mut(&skill) {
            *n += by;
        } else {
            self.0.insert(skill, by);
        }
    }
}