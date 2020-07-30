use legion::*;
use legion::systems::Schedulable;
use nox_components::*;

pub fn build() -> impl Schedulable {
    SystemBuilder::new("sleep")
        .with_query(<Write<MyTurn>>::query())
        .build(|_, ecs, _, actors| {
            actors
                .iter_mut(ecs)
                .filter(|turn| turn.active && turn.shift == ScheduleTime::Sleep)
                .for_each(|mut turn| {
                    turn.order = WorkOrder::MoveRandomly;
                });
        })
}
