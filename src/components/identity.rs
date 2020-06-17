use crate::components::prelude::*;
use legion::prelude::*;
use parking_lot::Mutex;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "32327503-a48b-483a-bde4-cc2825164b45"]
pub struct Identity {
    pub id: usize,
}

lazy_static! {
    pub static ref IDENTITY: Mutex<usize> = Mutex::new(0);
}

impl Identity {
    pub fn new() -> Self {
        let mut lock = IDENTITY.lock();
        *lock += 1;
        let id = *lock;
        Self { id }
    }
}

pub fn rebuild_identity(ecs: &World) {
    let mut lock = IDENTITY.lock();
    let query = Read::<Identity>::query();
    let max_id = query.iter(ecs).map(|i| i.id).max().unwrap();
    *lock = max_id;
}
