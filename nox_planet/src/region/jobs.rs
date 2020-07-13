use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use nox_components::*;
use bracket_geometry::prelude::*;
use crate::idxmap;

#[derive(Clone, Serialize, Deserialize)]
pub struct JobsBoard {
    designated_trees: HashSet<usize>,
    all_jobs: Vec<JobBoardListing>
}

impl JobsBoard {
    pub fn new() -> Self {
        Self {
            designated_trees : HashSet::new(),
            all_jobs: Vec::new()
        }
    }

    pub fn evaluate_jobs(&mut self, identity:usize, pos:&Position) -> Option<JobType> {
        let mut available_jobs : Vec<(usize, f32)> = self.all_jobs
            .iter()
            .enumerate()
            .filter(|(_, j)| j.claimed.is_none())
            .map(|(i, j) | (i, job_cost(pos, &j.job)) )
            .collect()
        ;

        if available_jobs.is_empty() {
            return None;
        }

        available_jobs.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
        let job_index = available_jobs[0].0;
        self.all_jobs[job_index].claimed = Some(identity);
        Some(self.all_jobs[job_index].job.clone())
    }

    pub fn get_trees(&self) -> &HashSet<usize> {
        &self.designated_trees
    }

    pub fn set_tree(&mut self, id: usize, tree_pos: usize) {
        self.designated_trees.insert(id);
        let matching_jobs = self.all_jobs.iter().filter(|j| {
            if let JobType::FellTree{tree_id,..} = j.job {
                tree_id == id
            } else {
                false
            }
        }).count();
        if matching_jobs == 0 {
            self.all_jobs.push(
                JobBoardListing{
                    job: JobType::FellTree{ tree_id: id, tree_pos, step: LumberjackSteps::FindAxe },
                    claimed: None
                }
            );
        }
    }

    pub fn remove_tree(&mut self, id: &usize) {
        self.designated_trees.remove(id);
        self.all_jobs.retain(|j| {
            if let JobType::FellTree{tree_id,..} = j.job {
                tree_id != *id
            } else {
                true
            }
        });
    }
}

fn job_cost(pos: &Position, job: &JobType) -> f32 {
    match job {
        JobType::FellTree{tree_pos, ..} => {
            let (tx, ty, tz) = idxmap(*tree_pos);
            DistanceAlg::Pythagoras.distance3d(
                Point3::new(pos.x, pos.y, pos.z), 
                Point3::new(tx, ty, tz)
            )
        }
        _ => 0.0
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct JobBoardListing {
    pub job : JobType,
    pub claimed : Option<usize>
}