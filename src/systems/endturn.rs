use legion::systems::Schedulable;
use legion::*;
use nox_components::*;

pub fn build() -> impl Schedulable {
    SystemBuilder::new("endturn")
        .with_query(<Write<MyTurn>>::query())
        .build(|_, ecs, _, turn| {
            turn.iter_mut(ecs).for_each(|mut t| {
                t.active = false;
            });
        })
}
