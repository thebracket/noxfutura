use super::renderflags::RenderFlags;
use super::{JobStep, WorldChange};
use parking_lot::Mutex;

lazy_static! {
    pub static ref RENDER_FLAGS: Mutex<RenderFlags> = Mutex::new(RenderFlags::new());
}

lazy_static! {
    pub static ref JOBS_QUEUE: Mutex<Vec<JobStep>> = Mutex::new(Vec::new());
}

lazy_static! {
    pub static ref WORLD_QUEUE: Mutex<Vec<WorldChange>> = Mutex::new(Vec::new());
}
