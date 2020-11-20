use crate::modes::playgame::systems::REGION;
use bengine::gui::*;
use legion::*;
use nox_components::*;
use nox_planet::{Region, TileType};
use nox_spatial::*;

pub enum ZoomRequest {
    None,
    Building { id: usize }
}

pub fn draw_tooltips(ecs: &World, mouse_world_pos: &(usize, usize, usize), imgui: &Ui) -> ZoomRequest {
    if imgui.io().want_capture_mouse {
        return ZoomRequest::None;
    }

    let mut zoom_mode = ZoomRequest::None;
    let mut lines: Vec<(bool, String)> = Vec::new();

    if mouse_world_pos.0 > 0
        && mouse_world_pos.0 < REGION_WIDTH
        && mouse_world_pos.1 > 0
        && mouse_world_pos.1 < REGION_HEIGHT
        && mouse_world_pos.2 > 0
        && mouse_world_pos.2 < REGION_DEPTH
    {
        let idx = mapidx(mouse_world_pos.0, mouse_world_pos.1, mouse_world_pos.2);
        let r = REGION.read();
        if !r.revealed[idx] {
            return ZoomRequest::None;
        }

        // Type info
        let mi = r.material_idx[idx];
        lines.push((
            true,
            format!(
                "{} ({})",
                match r.tile_types[idx] {
                    TileType::Empty => "Empty Space",
                    TileType::Floor => "Floor",
                    TileType::SemiMoltenRock => "Semi-Molten Rock",
                    TileType::Solid => "Solid",
                    TileType::Stairs { .. } => "Stairs",
                    TileType::Wall => "Wall",
                    TileType::Ramp { .. } => "Ramp",
                    _ => "",
                }
                .to_string(),
                nox_raws::RAWS.read().materials.materials[mi].name.clone()
            ),
        ));

        // Flags
        let mut l = String::new();
        if r.flag(idx, Region::SOLID) {
            l += "SOLID|";
        }
        if r.flag(idx, Region::CAN_STAND_HERE) {
            l += "ST|";
        }
        if r.flag(idx, Region::CAN_GO_NORTH) {
            l += "N|";
        }
        if r.flag(idx, Region::CAN_GO_SOUTH) {
            l += "S|";
        }
        if r.flag(idx, Region::CAN_GO_EAST) {
            l += "E|";
        }
        if r.flag(idx, Region::CAN_GO_WEST) {
            l += "W|";
        }
        if r.flag(idx, Region::CAN_GO_UP) {
            l += "U|";
        }
        if r.flag(idx, Region::CAN_GO_DOWN) {
            l += "D|";
        }
        if !l.is_empty() {
            lines.push((false, l));
        }
    }

    // This is eating a ton of frame time!
    let click = imgui.io().mouse_down[0];
    let mut tt = Tooltips::new();
    <(Entity, Read<Name>, Read<Position>, Read<IdentityTag>)>::query()
        .iter(ecs)
        .filter(|(_, _, pos, _)| pos.contains_point(mouse_world_pos))
        .for_each(|(entity, name, _, identity)| {
            tt.add_entry(ecs, entity, name, identity, click, &mut zoom_mode);
        }
    );
    tt.append_lines(&mut lines);

    if !lines.is_empty() {
        let im_lines: Vec<(bool, ImString)> = lines
            .iter()
            .map(|(heading, s)| (*heading, ImString::new(s)))
            .collect();

        let size = bengine::RENDER_CONTEXT.read().as_ref().unwrap().size;
        let mouse_pos = imgui.io().mouse_pos;
        let vsize = im_lines
            .iter()
            .map(|(_, s)| imgui.calc_text_size(s, false, 150.0)[1] + 10.0)
            .sum();

        let tip_pos = [
            f32::min(size.width as f32 - 300.0, mouse_pos[0]),
            f32::min(size.height as f32 - vsize, mouse_pos[1]),
        ];

        Window::new(im_str!("### tooltip"))
            .no_decoration()
            .size([300.0, vsize], Condition::Always)
            .collapsed(false, Condition::Always)
            .position(tip_pos, Condition::Always)
            .no_inputs()
            .build(imgui, || {
                im_lines.iter().for_each(|(heading, text)| {
                    if *heading {
                        imgui.text_colored([1.0, 1.0, 0.0, 1.0], text);
                    } else {
                        imgui.text_wrapped(text);
                    }
                });
            }
        );
    }

    zoom_mode
}

struct TooltipEntry {
    name : String,
    description: String,
    qty: i32,
    contents: Vec<String>
}

impl TooltipEntry {
    fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            qty: 1,
            contents : Vec::new()
        }
    }
}

struct Tooltips {
    entries : Vec<TooltipEntry>
}

impl Tooltips {
    fn new() -> Self {
        Self { entries: Vec::new() }
    }

    fn add_entry(&mut self, ecs: &World, entity: &Entity, name: &Name, identity: &IdentityTag, click: bool, zoom_mode: &mut ZoomRequest) {
        let mut tt = TooltipEntry::new(name.name.clone());

        // Building Info
        if let Ok(binfo) = ecs.entry_ref(*entity).unwrap().get_component::<Building>() {
            if !binfo.complete {
                tt.name = format!("{} - Incomplete", tt.name);
            }
            if click {
                *zoom_mode = ZoomRequest::Building{ id: identity.0 };
            }
        }

        // Description
        <(Entity, Read<Description>)>::query()
            .iter(ecs)
            .filter(|(e, _)| *e == entity)
            .for_each(|(_, d)| {
                tt.description = d.desc.clone();
            }
        );

        // Check container contents
        <(Read<Name>, Read<Position>)>::query()
            .iter(ecs)
            .filter(|(_, store)| store.is_in_container(identity.0))
            .for_each(|(name, _)| {
                tt.contents.push(name.name.clone());
            }
        );

        if let Some(ott) = self.entries.iter_mut().find(|e| e.name == tt.name) {
            ott.qty += 1;
        } else {
            self.entries.push(tt);
        }
    }

    fn append_lines(&self, lines: &mut Vec<(bool, String)>) {
        self.entries.iter().for_each(|tt| {
            if tt.qty > 1 {
                lines.push((true, format!("{} x{}", tt.name, tt.qty)));
            } else {
                lines.push((true, tt.name.clone()));
            }

            if !tt.description.is_empty() {
                lines.push((false, tt.description.clone()));
            }

            if !tt.contents.is_empty() {
                tt.contents.iter().for_each(|content| {
                    lines.push((false, format!(" - {}", content)));
                });
            }
        });
    }
}