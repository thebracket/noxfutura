use legion::prelude::*;
use crate::components::*;

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("calendar")
    .with_query(<Write<Calendar>>::query())
    .build(|_, ecs, _, calendars| {
        calendars.iter_mut(ecs).for_each(|mut c| {
            c.minute += 1;
            if c.minute > 59 { c.minute = 0; c.hour +=1; }
            if c.hour > 23 { c.hour = 0; c.day += 1; }
            if c.day > 30 { c.day = 0; c.month += 1; }
            if c.month > 11 { c.month = 0; c.year += 1; }
        });
    })
}