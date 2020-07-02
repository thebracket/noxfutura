use crate::components::*;
use legion::prelude::*;

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("endturn")
    .with_query(<Read<Initiative>>::query().filter(tag::<MyTurn>()))
        .build(| commands, ecs, _, turn| {
            turn.iter_entities(ecs).for_each(|(entity, _)| {
                commands.remove_tag::<MyTurn>(entity);
            });
        })
}
