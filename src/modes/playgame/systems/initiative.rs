use crate::components::*;
use legion::prelude::*;

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("initiative")
        .with_query(<Write<Initiative>>::query())
        .build(| commands, ecs, _, actors| {
            actors.iter_entities_mut(ecs).for_each(|(entity, mut i)| {
                i.initiative -= 1;
                if i.initiative + i.modifier < 1 {
                    // Re-roll initiative
                    i.initiative = 10; // TODO: Make random!

                    // Reset modifiers
                    i.modifier = 0;

                    // Apply the my turn tag
                    commands.add_tag(entity, MyTurn{});
                    println!("Turn!");
                }
            });
        })
}
