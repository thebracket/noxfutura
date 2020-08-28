use crate::systems::RNG;
use crate::utils::attribute_modifier;
use legion::systems::Schedulable;
use legion::*;
use nox_components::*;

pub fn build() -> impl Schedulable {
    SystemBuilder::new("initiative")
        .with_query(<(Write<Initiative>, Write<MyTurn>, Read<Attributes>)>::query())
        .build(|_, ecs, _, actors| {
            actors.iter_mut(ecs).for_each(|(mut i, mut t, attrib)| {
                i.initiative -= 1;
                if i.initiative + i.modifier < 1 {
                    // Re-roll initiative
                    i.initiative = RNG.lock().roll_dice(2, 6) - attribute_modifier(attrib.dex);
                    // TODO: Add modifiers from equipment etc.

                    // Reset modifiers
                    i.modifier = 0;

                    // Apply the my turn tag
                    t.active = true;
                    t.shift = ScheduleTime::Work;
                }
            });
        })
}