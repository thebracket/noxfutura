use crate::components::*;
use legion::prelude::*;

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("endturn")
        .with_query(<Write<MyTurn>>::query())
        .build(|_, ecs, _, turn| {
            turn.iter_mut(ecs).for_each(|mut t| {
                t.0 = false;
            });
        })
}
