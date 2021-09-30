use crate::components::Position;
use lazy_static::*;
use parking_lot::RwLock;

lazy_static! {
    pub static ref SPAWNS: RwLock<Vec<EntitySpawn>> = RwLock::new(Vec::new());
}

pub struct EntitySpawn {
    pub position: Position,
    pub event: SpawnRequest,
}

pub enum SpawnRequest {
    Tree,
    RawEntity { tag: String },
}

pub fn spawn_tree(position: Position) {
    SPAWNS.write().push(EntitySpawn {
        position,
        event: SpawnRequest::Tree,
    });
}

pub fn spawn_raws_entity(position: Position, tag: &str) {
    SPAWNS.write().push(EntitySpawn {
        position,
        event: SpawnRequest::RawEntity {
            tag: tag.to_string(),
        },
    });
}
