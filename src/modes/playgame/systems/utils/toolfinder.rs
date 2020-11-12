
use legion::*;
use legion::world::SubWorld;
use nox_components::*;

pub enum ToolCarrying {
    Carried{tool_id: usize},
    AtLocation{idx: usize, tool_id: usize},
    NoTool
}

pub fn am_i_carrying_tool(ecs: &SubWorld, holder: usize, usage: ToolType) -> ToolCarrying {
    <(&Claimed, &Tool, &Position, &IdentityTag)>::query()
        .iter(ecs)
        .filter(|(claim, tool, _, _)| claim.by == holder && tool.usage == usage )
        .map(|(_claim, _tool, pos, tool_id)| {
            match pos.loc {
                Location::Carried{by} => {
                    if by == holder {
                        ToolCarrying::Carried{tool_id:tool_id.0}
                    } else {
                        ToolCarrying::AtLocation{idx : pos.effective_location_sw(ecs), tool_id:tool_id.0 }
                    }
                },
                _ => ToolCarrying::AtLocation{idx : pos.effective_location_sw(ecs), tool_id:tool_id.0 }
            }
        })
        .nth(0)
        .unwrap_or(ToolCarrying::NoTool)
}