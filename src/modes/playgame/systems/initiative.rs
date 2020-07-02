use crate::components::*;
use legion::prelude::*;

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("initiative")
        .with_query(<(Write<Initiative>, Write<MyTurn>)>::query())
        .build(| _, ecs, _, actors| {
            actors.iter_mut(ecs).for_each(|(mut i, mut t)| {
                i.initiative -= 1;
                if i.initiative + i.modifier < 1 {
                    // Re-roll initiative
                    i.initiative = 10; // TODO: Make random!

                    // Reset modifiers
                    i.modifier = 0;

                    // Apply the my turn tag
                    t.0 = true;
                }
            });
        })
}
