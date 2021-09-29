use crate::simulation::{
    mapidx,
    terrain::{
        get_material_idx, ground_z, is_tile_floor, submit_change_batch, ChangeRequest,
        MapChangeBatch, PlanetLocation, RampDirection,
    },
    REGION_HEIGHT, REGION_WIDTH,
};

pub(crate) fn build_ramps(planet_idx: PlanetLocation) {
    let mut changes = MapChangeBatch::new(planet_idx);
    for y in 1..REGION_HEIGHT - 1 {
        for x in 1..REGION_WIDTH - 1 {
            let z = ground_z(planet_idx, x, y);
            let idx = mapidx(x, y, z);
            let mat = get_material_idx(planet_idx, idx - (REGION_WIDTH * REGION_HEIGHT));

            if is_tile_floor(planet_idx, idx) {
                if is_tile_floor(planet_idx, mapidx(x, y - 1, z + 1)) {
                    let mat = get_material_idx(planet_idx, idx - (REGION_WIDTH * REGION_HEIGHT));
                    changes.enqueue_change(ChangeRequest::Ramp {
                        idx,
                        material: mat,
                        direction: RampDirection::NorthSouth,
                    });
                } else if is_tile_floor(planet_idx, mapidx(x, y + 1, z + 1)) {
                    changes.enqueue_change(ChangeRequest::Ramp {
                        idx,
                        material: mat,
                        direction: RampDirection::SouthNorth,
                    });
                } else if is_tile_floor(planet_idx, mapidx(x + 1, y, z + 1)) {
                    changes.enqueue_change(ChangeRequest::Ramp {
                        idx,
                        material: mat,
                        direction: RampDirection::WestEast,
                    });
                } else if is_tile_floor(planet_idx, mapidx(x - 1, y, z + 1)) {
                    changes.enqueue_change(ChangeRequest::Ramp {
                        idx,
                        material: mat,
                        direction: RampDirection::EastWest,
                    });
                }
            }
        }
    }
    submit_change_batch(changes);
}
