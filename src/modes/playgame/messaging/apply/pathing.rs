use legion::*;
use nox_components::*;
use nox_spatial::idxmap;
use super::MOVER_LIST;

pub(crate) fn follow_path(ecs: &mut World, id: usize) {
    <(&mut MyTurn, &IdentityTag)>::query()
        .iter_mut(ecs)
        .filter(|(_, idt)| idt.0 == id)
        .for_each(|(turn, _)| {
            let path = match &mut turn.job {
                JobType::CollectTool {step, ..} => {
                    match step {
                        CollectToolSteps::TravelToTool { path } => Some(path),
                        _ => None
                    }
                }
                JobType::ConstructBuilding { step, .. } => {
                    match step {
                        BuildingSteps::TravelToComponent { path, .. } => Some(path),
                        BuildingSteps::TravelToTBuilding { path, .. } => Some(path),
                        _ => None,
                    }
                }
                _ => None
            };

            if let Some(path) = path {
                let destination = path[0];
                path.remove(0);
                let (x, y, z) = idxmap(destination);
                MOVER_LIST.lock().insert(id, (x, y, z));
            }
        }
    );
}