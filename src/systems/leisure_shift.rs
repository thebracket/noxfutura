use legion::systems::Schedulable;
use legion::*;
use nox_components::*;

pub fn build() -> impl Schedulable {
    SystemBuilder::new("leisure")
        .with_query(<Write<MyTurn>>::query())
        .build(|_, ecs, _, actors| {
            actors
                .iter_mut(ecs)
                .filter(|turn| turn.active && turn.shift == ScheduleTime::Leisure)
                .for_each(|mut turn| {
                    turn.order = WorkOrder::MoveRandomly;
                });
        })
}
