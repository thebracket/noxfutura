use legion::*;
use crate::components::*;

#[system(for_each)]
pub fn work_shift(turn: &mut MyTurn) {
    if turn.active && turn.shift == ScheduleTime::Work {
        turn.order = WorkOrder::MoveRandomly;
    }
}
