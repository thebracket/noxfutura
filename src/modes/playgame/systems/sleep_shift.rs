use legion::*;
use nox_components::*;

#[system(for_each)]
pub fn sleep_shift(turn: &mut MyTurn) {
    if turn.active && turn.shift == ScheduleTime::Sleep {
        turn.order = WorkOrder::MoveRandomly;
    }
}
