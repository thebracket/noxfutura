use super::REGION;
use legion::*;
use nox_components::*;
use nox_planet::MiningMap;

#[system(for_each)]
pub fn work_shift(
    turn: &mut MyTurn,
    pos: &Position,
    settler: &Settler,
    id: &IdentityTag,
    #[resource] mining: &MiningMap,
) {
    if turn.active && turn.shift == ScheduleTime::Work && turn.job == JobType::None {
        turn.order = WorkOrder::None;
        // todo: include more attributes
        if let Some(job) = REGION
            .write()
            .jobs_board
            .evaluate_jobs(id.0, &*pos, settler, mining)
        {
            turn.job = job;
            println!("Assigned job: {:?}", turn.job);
        } else {
            turn.order = WorkOrder::MoveRandomly;
        }
    }
}
