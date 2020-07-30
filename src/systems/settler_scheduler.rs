use legion::*;
use legion::systems::Schedulable;
use nox_components::*;

pub fn build() -> impl Schedulable {
    SystemBuilder::new("settler_schedule")
        .with_query(<(Write<MyTurn>, Read<WorkSchedule>)>::query())
        .with_query(<Read<Calendar>>::query())
        .build(|_, ecs, _, (actors, calendars)| {
            let hour = calendars.iter(ecs).nth(0).unwrap().hour as usize;
            actors
                .iter_mut(ecs)
                .filter(|(turn, _)| turn.active)
                .for_each(|(mut turn, schedule)| {
                    turn.shift = schedule.hours[hour];
                    turn.shift = ScheduleTime::Work;
                });
        })
}
