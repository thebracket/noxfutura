mod calendar;
mod endturn;
mod initiative;
mod settler_scheduler;
mod shared_state;
mod viewshed;
mod leisure_shift;
mod work_shift;
mod sleep_shift;
mod move_randomly;
pub use shared_state::*;
mod ui;
pub use ui::*;

use legion::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(calendar::build())
        .add_system(viewshed::build())
        .add_system(initiative::build())
        .flush()
        .add_system(settler_scheduler::build())
        .flush()
        .add_system(leisure_shift::build())
        .add_system(work_shift::build())
        .add_system(sleep_shift::build())
        .flush()
        .add_system(move_randomly::build())
        .flush()
        .add_system(endturn::build())
        .build()
}

pub fn paused_scheduler() -> Schedule {
    Schedule::builder().add_system(viewshed::build()).build()
}
