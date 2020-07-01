mod calendar;
mod viewshed;

use legion::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(calendar::build())
        .add_system(viewshed::build())
        .build()
}
