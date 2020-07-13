use legion::prelude::*;
use nox_components::*;
use crate::systems::REGION;

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("lumberjack")
        .with_query(<(Write<MyTurn>, Read<Position>, Read<Identity>)>::query())
        .build(|_, ecs, _, actors| {
            // Look for a job to do
            actors
                .iter_mut(ecs)
                .filter(|(turn, _, _)| {
                    // Filter out anything that isn't a lumberjack job
                    turn.active && turn.shift == ScheduleTime::Work
                    && match turn.job {
                        JobType::FellTree{..} => true,
                        _ => false
                    }
                })
                .for_each(|(mut turn, pos, id)| {
                    println!("I'm a lumberjack!");
                    turn.order = WorkOrder::None;

                    if let JobType::FellTree{ tree_id, tree_pos, step } = &turn.job {
                        match step {
                            LumberjackSteps::FindAxe => {
                                
                            }
                            _ => println!("Warning - LumberJack fell through with no steps.")
                        }
                    } else {
                        panic!("Not doing a lumberjack job but wound up in the LJ system!");
                    }
                }
            );
        }
    )
}
