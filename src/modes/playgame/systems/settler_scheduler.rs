use legion::*;
use world::SubWorld;
use crate::components::*;

#[system]
#[write_component(MyTurn)]
#[read_component(WorkSchedule)]
#[read_component(Calendar)]
pub fn settler_schedule(ecs: &mut SubWorld) {
    let mut actors = <(&mut MyTurn, &WorkSchedule)>::query();
    let mut calendars = <&Calendar>::query();
    let hour = calendars.iter(ecs).nth(0).unwrap().hour as usize;
    actors
        .iter_mut(ecs)
        .filter(|(turn, _)| turn.active)
        .for_each(|(mut turn, schedule)| {
            turn.shift = schedule.hours[hour];
            turn.shift = ScheduleTime::Leisure;
        }
    );
}
