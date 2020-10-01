use super::RNG;
use crate::utils::attribute_modifier;
use legion::*;
use crate::components::*;

#[system(for_each)]
pub fn initiative(i: &mut Initiative, t: &mut MyTurn, attrib : &Attributes) {
    i.initiative -= 1;
    if i.initiative + i.modifier < 1 {
        // Re-roll initiative
        i.initiative = RNG.lock().roll_dice(2, 6) - attribute_modifier(attrib.dex);
        // TODO: Add modifiers from equipment etc.

        // Reset modifiers
        i.modifier = 0;

        // Apply the my turn tag
        t.active = true;
        t.shift = ScheduleTime::Leisure;
    }
}
