use crate::components::prelude::*;
use legion::*;
use parking_lot::Mutex;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct IdentityTag(pub usize);

lazy_static! {
    pub static ref IDENTITY: Mutex<usize> = Mutex::new(0);
}

impl IdentityTag {
    pub fn new() -> Self {
        let mut lock = IDENTITY.lock();
        *lock += 1;
        let id = *lock;
        Self(id)
    }
}

pub fn rebuild_identity(ecs: &World) {
    let mut lock = IDENTITY.lock();
    let mut query = Read::<IdentityTag>::query();
    let max_id = query.iter(ecs).map(|i| i.0).max().unwrap();
    *lock = max_id;
}
