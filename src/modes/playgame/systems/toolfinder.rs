use super::REGION;
use legion::*;
use nox_components::*;

pub fn tool_finder(ecs: &World) {
    <(&Tool, &Position, &IdentityTag)>::query()
        .iter(ecs)
        .for_each(|(tool, pos, id)| {
            REGION
                .write()
                .jobs_board
                .update_tool(id.0, tool.usage, pos.effective_location(ecs));
        })
}
