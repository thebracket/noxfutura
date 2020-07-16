use super::renderflags::RenderFlags;
use super::JobStep;
use parking_lot::Mutex;
use std::collections::{VecDeque, HashMap};

lazy_static! {
    pub static ref RENDER_FLAGS: Mutex<RenderFlags> = Mutex::new(RenderFlags::new());
}

lazy_static! {
    pub static ref JOBS_QUEUE: Mutex<VecDeque<JobStep>> = Mutex::new(VecDeque::new());
}

lazy_static! {
    pub static ref MOVER_LIST: Mutex<HashMap<usize, (usize, usize, usize)>> = Mutex::new(HashMap::new());
}
