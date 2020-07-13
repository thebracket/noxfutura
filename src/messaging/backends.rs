use super::renderflags::RenderFlags;
use parking_lot::Mutex;
use super::JobStep;

lazy_static! {
    pub static ref RENDER_FLAGS: Mutex<RenderFlags> = Mutex::new(RenderFlags::new());
}

lazy_static! {
    pub static ref JOBS_QUEUE: Mutex<Vec<JobStep>> = Mutex::new(Vec::new());
}