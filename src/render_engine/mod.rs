mod assets;
mod device;
mod game_mode;
mod main_loop;
mod simple2d;

pub use assets::{Shader, ASSETS};
use device::init_render_context;
pub use device::{OUTPUT_FORMAT, RENDER_CONTEXT};
pub use game_mode::{GameMode, TickResult};
pub use main_loop::run;
pub use simple2d::*;
