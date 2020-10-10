use super::{REGION, RNG};
use nox_components::*;
use nox_planet::Region;
use legion::*;

#[system(for_each)]
pub fn move_randomly(pos: &mut Position, turn: &MyTurn, id: &IdentityTag) {
    if turn.active && turn.order == WorkOrder::MoveRandomly {
        let idx = pos.get_idx();
        let delta = random_move(idx);
        let mut destination = pos.as_point3();
        destination.x += delta.0;
        destination.y += delta.1;
        destination.z += delta.2;
        super::messaging::entity_moved(id.0, &destination);
    }
}

fn random_move(idx: usize) -> (i32, i32, i32) {
    let roll = RNG.lock().range(1, 7);
    match roll {
        1 => {
            if REGION.read().flag(idx, Region::CAN_GO_NORTH) {
                (0, -1, 0)
            } else {
                (0, 0, 0)
            }
        }

        2 => {
            if REGION.read().flag(idx, Region::CAN_GO_SOUTH) {
                (0, 1, 0)
            } else {
                (0, 0, 0)
            }
        }

        3 => {
            if REGION.read().flag(idx, Region::CAN_GO_EAST) {
                (1, 0, 0)
            } else {
                (0, 0, 0)
            }
        }

        4 => {
            if REGION.read().flag(idx, Region::CAN_GO_WEST) {
                (-1, 0, 0)
            } else {
                (0, 0, 0)
            }
        }

        5 => {
            if REGION.read().flag(idx, Region::CAN_GO_UP) {
                (0, 0, 1)
            } else {
                (0, 0, 0)
            }
        }

        _ => {
            if REGION.read().flag(idx, Region::CAN_GO_DOWN) {
                (0, 0, -1)
            } else {
                (0, 0, 0)
            }
        }
    }
}
