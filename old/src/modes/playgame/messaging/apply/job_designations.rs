use bengine::geometry::*;
use legion::*;
use nox_components::*;

pub fn become_miner(ecs: &mut World, id: usize) {
    // Find the settler in question
    let mut success = false;
    if let Some(settler_pos) = <(&IdentityTag, &Position)>::query()
        .filter(component::<Settler>())
        .iter(ecs)
        .filter(|(sid, _)| sid.0 == id)
        .map(|(_, pos)| pos.as_point3())
        .nth(0)
    {
        if find_closest_tool(ecs, ToolType::Digging, id, &settler_pos) {
            success = true
        }
    }

    if success {
        <(&mut Settler, &IdentityTag)>::query()
            .iter_mut(ecs)
            .filter(|(_, sid)| sid.0 == id)
            .for_each(|(settler, _)| settler.miner = true);
    }
}

pub fn become_lumberjack(ecs: &mut World, id: usize) {
    println!("Become LJ");
    // Find the settler in question
    let mut success = false;
    if let Some(settler_pos) = <(&IdentityTag, &Position)>::query()
        .filter(component::<Settler>())
        .iter(ecs)
        .filter(|(sid, _)| sid.0 == id)
        .map(|(_, pos)| pos.as_point3())
        .nth(0)
    {
        println!("Found settler");
        if find_closest_tool(ecs, ToolType::Chopping, id, &settler_pos) {
            success = true
        }
    }

    if success {
        <(&mut Settler, &IdentityTag)>::query()
            .iter_mut(ecs)
            .filter(|(_, sid)| sid.0 == id)
            .for_each(|(settler, _)| settler.lumberjack = true);
    }
}

pub fn fire_miner(ecs: &mut World, id: usize) {
    drop_associated_tool(ecs, ToolType::Digging, id);
    <(&mut Settler, &IdentityTag)>::query()
        .iter_mut(ecs)
        .filter(|(_s, sid)| sid.0 == id)
        .for_each(|(s, _)| s.miner = false);
}

pub fn fire_lumberjack(ecs: &mut World, id: usize) {
    drop_associated_tool(ecs, ToolType::Chopping, id);
    <(&mut Settler, &IdentityTag)>::query()
        .iter_mut(ecs)
        .filter(|(_s, sid)| sid.0 == id)
        .for_each(|(s, _)| s.lumberjack = false);
}

fn find_closest_tool(ecs: &mut World, usage: ToolType, claimant: usize, position: &Point3) -> bool {
    println!("Looking for tool");
    let mut tools: Vec<(Entity, f32)> = <(Entity, &Tool, &Position)>::query()
        .filter(!component::<Claimed>())
        .iter(ecs)
        .filter(|(_, tool, _)| tool.usage == usage)
        .map(|(e, _, pos)| {
            (
                *e,
                DistanceAlg::Pythagoras.distance3d(*position, pos.as_point3()),
            )
        })
        .collect();

    if tools.is_empty() {
        println!("No tools found");
        return false;
    }

    println!("Marking the tool as claimed");
    tools.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    ecs.entry(tools[0].0)
        .unwrap()
        .add_component(Claimed { by: claimant });
    return true;
}

fn drop_associated_tool(ecs: &World, usage: ToolType, id: usize) {
    let item_req = Location::Carried { by: id };

    let to_drop: Vec<(usize, usize)> = <(&Claimed, &Tool, &Position, &IdentityTag)>::query()
        .iter(ecs)
        .filter(|(claim, tool, pos, _tid)| {
            claim.by == id && tool.usage == usage && pos.loc == item_req
        })
        .map(|(_claim, _tool, pos, id)| (pos.effective_location(ecs), id.0))
        .collect();

    to_drop.iter().for_each(|(pos, tid)| {
        super::super::drop_item(*tid, *pos);
    });
}
