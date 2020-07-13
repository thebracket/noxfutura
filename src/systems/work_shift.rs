use legion::prelude::*;
use nox_components::*;
use crate::systems::REGION;

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("work")
        .with_query(<(Write<MyTurn>, Read<Position>, Read<Identity>)>::query())
        .build(|_, ecs, _, actors| {
            // Look for a job to do
            actors
                .iter_mut(ecs)
                .filter(|(turn, _, _)| turn.active && turn.shift == ScheduleTime::Work && turn.job == JobType::None)
                .for_each(|(mut turn, pos, id)| {
                    turn.order = WorkOrder::None;
                    // todo: include more attributes
                    if let Some(job) = REGION.write().jobs_board.evaluate_jobs(id.id, &*pos) {
                        turn.job = job;
                        println!("Assigned job: {:?}", turn.job);
                    } else {
                        turn.order = WorkOrder::MoveRandomly;
                    }
                }
            );
        }
    )
}
