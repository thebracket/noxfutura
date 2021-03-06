use crate::{Region, StairsType, TileType};
use legion::*;
use nox_components::*;
use nox_raws::*;

pub fn transform_terrain_to_ecs(region: &mut Region, ecs: &mut World) {
    let stairs_up = RAWS.read().vox.get_model_idx("stairs_up");
    let stairs_down = RAWS.read().vox.get_model_idx("stairs_down");
    let stairs_updown = RAWS.read().vox.get_model_idx("stairs_updown");

    region
        .tile_types
        .iter()
        .enumerate()
        .filter(|(_, tt)| {
            match tt {
                TileType::Stairs { .. } => true,
                //TileType::Ramp{..} => true,
                _ => false,
            }
        })
        .for_each(|(idx, tt)| {
            let model_id = match tt {
                TileType::Stairs { direction } => match direction {
                    StairsType::Up => stairs_up,
                    StairsType::Down => stairs_down,
                    StairsType::UpDown => stairs_updown,
                },
                _ => 0,
            };

            let tint = *RAWS.read().matmap.get(region.material_idx[idx]);

            ecs.push((
                Terrain {},
                Position::with_tile_idx(idx, region.world_idx, (1, 1, 1)),
                nox_components::VoxelModel {
                    index: model_id,
                    rotation_radians: 0.0,
                },
                Tint { color: tint },
            ));
        });
}
