use legion::prelude::*;
use crate::{Region, StairsType, TileType, RampDirection};
use nox_raws::*;
use nox_components::*;

pub fn transform_terrain_to_ecs(region: &mut Region, ecs: &mut World) {
    let stairs_up = RAWS.read().vox.get_model_idx("stairs_up");
    let stairs_down = RAWS.read().vox.get_model_idx("stairs_down");
    let stairs_updown = RAWS.read().vox.get_model_idx("stairs_updown");
    let ramp = RAWS.read().vox.get_model_idx("rampnew");

    region
        .tile_types
        .iter()
        .enumerate()
        .filter(|(_, tt)| {
            match tt {
                TileType::Stairs{..} => true,
                TileType::Ramp{..} => true,
                _ => false
            }
        })
        .for_each(|(idx, tt)| {
            let model_id = match tt {
                TileType::Stairs{direction} => {
                    match direction {
                        StairsType::Up => stairs_up,
                        StairsType::Down => stairs_down,
                        StairsType::UpDown => stairs_updown
                    }
                }
                TileType::Ramp{..} => ramp,
                _ => 0
            };

            let rotation = match tt {
                TileType::Ramp{ direction } => match direction {
                    RampDirection::NorthSouth => 3.14159,
                    RampDirection::SouthNorth => 0.0,
                    RampDirection::WestEast => 1.5708,
                    RampDirection::EastWest => 4.71239,
                },
                _ => 0.0
            };

            let tint = if region.flag(idx, Region::CONSTRUCTED) {
                RAWS.read().matmap.get(region.material_idx[idx]).floor.tint
            } else {
                RAWS.read().matmap.get(region.material_idx[idx]).floor_constructed.tint
            };

            ecs.insert(
                (Terrain{}, ),
                vec![(
                    Position::with_tile_idx(idx, region.world_idx, (1, 1, 1)),
                    nox_components::VoxelModel{ index: model_id, rotation_radians: rotation },
                    Tint{ color: tint }
                )]
            );
        }
    );
}
