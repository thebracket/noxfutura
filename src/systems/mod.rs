mod calendar;
mod endturn;
mod initiative;
mod shared_state;
mod viewshed;
mod settler_scheduler;
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
        .add_system(endturn::build())
        .build()
}

pub fn paused_scheduler() -> Schedule {
    Schedule::builder().add_system(viewshed::build()).build()
}
