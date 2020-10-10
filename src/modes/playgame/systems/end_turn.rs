use nox_components::*;
use legion::*;

#[system(for_each)]
pub fn end_turn(turn: &mut MyTurn) {
    turn.active = false;
}
