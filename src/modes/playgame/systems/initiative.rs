use crate::components::*;
use legion::prelude::*;
use crate::modes::playgame::shared_state::RNG;

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("initiative")
        .with_query(<(Write<Initiative>, Write<MyTurn>)>::query())
        .build(|_, ecs, _, actors| {
            actors.iter_mut(ecs).for_each(|(mut i, mut t)| {
                i.initiative -= 1;
                if i.initiative + i.modifier < 1 {
                    // Re-roll initiative
                    i.initiative = RNG.lock().roll_dice(2, 6);
                    // TODO: Add dex bonus and everything else

                    // Reset modifiers
                    i.modifier = 0;

                    // Apply the my turn tag
                    t.0 = true;
                }
            });
        })
}
