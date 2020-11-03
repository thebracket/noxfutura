use legion::*;
use nox_components::*;

#[system(for_each)]
pub fn end_turn(turn: &mut MyTurn) {
    turn.active = false;
}
