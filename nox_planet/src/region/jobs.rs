use super::{MiningMap, LumberMap};
use bengine::geometry::*;
use nox_components::*;
use nox_spatial::idxmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct JobsBoard {
    all_jobs: Vec<JobBoardListing>,
    component_ownership: HashMap<usize, ComponentClaim>,
    autojobs : HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>> // Key (building/reaction/pos), value (component, pos, mat)
}

impl JobsBoard {
    pub fn new() -> Self {
        Self {
            all_jobs: Vec::new(),
            component_ownership: HashMap::new(),
            autojobs: HashMap::new()
        }
    }

    pub fn evaluate_jobs(
        &mut self,
        identity: usize,
        pos: &Position,
        settler: &Settler,
        mining_map: &MiningMap,
        lumber_map: &LumberMap
    ) -> Option<JobType> {
        let mut available_jobs: Vec<(usize, f32)> = self
            .all_jobs
            .iter()
            .enumerate()
            .filter(|(_, j)| j.claimed.is_none() && self.is_possible(j, settler))
            .map(|(i, j)| (i, job_cost(pos, &j.job)))
            .collect();

        if settler.miner {
            let idx = pos.get_idx();
            if mining_map.dijkstra[idx] < f32::MAX {
                available_jobs.push((usize::MAX, mining_map.dijkstra[idx]));
            }
        }

        if settler.lumberjack {
            let idx = pos.get_idx();
            if lumber_map.dijkstra[idx] < f32::MAX {
                available_jobs.push((usize::MAX-1, lumber_map.dijkstra[idx]));
            }
        }

        if available_jobs.is_empty() {
            if self.autojobs.is_empty() {
                return None;
            } else {
                // Add the first autojob for consideraton
                // Removing it from the autojobs list
                let autojob_key = self.autojobs.keys().nth(0).unwrap().clone();
                let job = self.autojobs.get(&autojob_key).unwrap();

                let cids = job
                    .iter()
                    .map(|(cid, pos, mat)| (*cid, *pos, false, *mat)).collect();

                self.all_jobs.push(
                    JobBoardListing{
                        job: JobType::Reaction {
                            workshop_id : autojob_key.0,
                            reaction_id: autojob_key.1,
                            workshop_pos: autojob_key.2,
                            components: cids,
                            step: ReactionSteps::ClaimEverything
                        },
                        claimed: None
                    }
                );
                available_jobs.push((self.all_jobs.len()-1, 0.0));

                self.autojobs.remove(&autojob_key);
            }
        }

        available_jobs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let job_index = available_jobs[0].0;
        if job_index == usize::MAX {
            // Mining
            Some(JobType::Mining {
                step: MiningSteps::FindPick,
                tool_id: None,
            })
        } else if job_index == usize::MAX -1 {
            // Lumberjack
            Some(JobType::FellTree {
                step: LumberjackSteps::FindAxe,
                tool_id: None,
            })
        } else {
            // Everything else
            self.all_jobs[job_index].claimed = Some(identity);
            Some(self.all_jobs[job_index].job.clone())
        }
    }

    fn is_possible(&self, job: &JobBoardListing, settler: &Settler) -> bool {
        match job.job {
            JobType::FellTree { .. } => settler.lumberjack,
            _ => true,
        }
    }

    pub fn restore_job(&mut self, _job: &JobType) {
    }

    pub fn is_component_claimed(&self, id: usize) -> bool {
        self.component_ownership.contains_key(&id)
    }

    pub fn claim_component_for_building(
        &mut self,
        building_id: usize,
        component_id: usize,
        effective_location: usize,
    ) {
        self.component_ownership.insert(
            component_id,
            ComponentClaim {
                claimed_by_building: building_id,
                effective_location,
            },
        );
    }

    pub fn relinquish_component_for_building(&mut self, component_id: usize) {
        self.component_ownership.remove(&component_id);
    }

    pub fn add_building_job(
        &mut self,
        building_id: usize,
        building_pos: usize,
        comps: &[(usize, usize)],
    ) {
        let components = comps.iter().map(|(idx, id)| (*idx, *id, false)).collect();
        self.all_jobs.push(JobBoardListing {
            claimed: None,
            job: JobType::ConstructBuilding {
                building_id,
                building_pos,
                step: BuildingSteps::FindComponent,
                components,
            },
        });
    }

    pub fn autojob_registered(&mut self, building_id: usize, reaction_id: usize, pos: usize) -> bool {
        if let Some(j) = self.autojobs.get(&(building_id, reaction_id, pos)) {
            let mut still_possible = true;
            j.iter().for_each(|(id, _, _)| {
                if self.is_component_claimed(*id) {
                    still_possible = false;
                }
            });
            if !still_possible {
                self.autojobs.remove(&(building_id, reaction_id, pos));
            }

            still_possible
        } else {
            false
        }
    }

    pub fn register_autojob(&mut self, building_id: usize, reaction_id: usize, pos: usize, components: &[(usize, usize, usize)]) {
        let mut c = Vec::new();
        c.extend_from_slice(components);
        self.autojobs.insert(
            (building_id, reaction_id, pos),
            c
        );
    }
}

fn job_cost(pos: &Position, job: &JobType) -> f32 {
    match job {
        JobType::ConstructBuilding { building_pos, .. } => {
            let (tx, ty, tz) = idxmap(*building_pos);
            DistanceAlg::Pythagoras.distance3d(pos.as_point3(), Point3::new(tx, ty, tz))
        }
        _ => 0.0,
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct JobBoardListing {
    pub job: JobType,
    pub claimed: Option<usize>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ToolClaim {
    pub claimed: Option<usize>,
    pub usage: ToolType,
    pub effective_location: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ComponentClaim {
    pub claimed_by_building: usize,
    pub effective_location: usize,
}
