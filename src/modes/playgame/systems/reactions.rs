use super::super::messaging;
use crate::modes::playgame::systems::REGION;
use legion::*;
use nox_components::*;
use nox_planet::pathfinding::a_star_search;

#[system(for_each)]
pub fn reactions(turn: &MyTurn, pos: &Position, id: &IdentityTag, _settler: &Settler) {
    if turn.active
        && turn.shift == ScheduleTime::Work
        && match turn.job {
            JobType::Reaction { .. } => true,
            _ => false,
        }
    {
        if let JobType::Reaction {
            workshop_id,
            workshop_pos,
            reaction_id,
            components,
            step,
        } = &turn.job
        {}
    }
}
