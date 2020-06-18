mod calendar;

use legion::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(calendar::build())
        .build()
}