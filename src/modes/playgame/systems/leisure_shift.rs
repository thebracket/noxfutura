use legion::*;
use nox_components::*;

#[system(for_each)]
pub fn leisure_shift(turn: &mut MyTurn) {
    if turn.active && turn.shift == ScheduleTime::Leisure {
        turn.order = WorkOrder::MoveRandomly;
    }
}
