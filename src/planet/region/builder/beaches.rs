use crate::planet::{Region, TileType, REGION_HEIGHT, REGION_WIDTH};
use crate::raws::get_material_by_tag;
use crate::utils::{ground_z, mapidx};

pub fn build_beaches(region: &mut Region) {
    let yellow_sand = get_material_by_tag("Yellow Sand").expect("Yellow Sand not found");
    for y in 1..REGION_HEIGHT - 1 {
        for x in 1..REGION_WIDTH - 1 {
            let z = ground_z(region, x, y);
            let idx = mapidx(x, y, z);
            if region.tile_types[idx] == TileType::Floor && region.water_level[idx] == 0 {
                if region.water_level[mapidx(x, y - 1, z - 1)] > 0
                    || region.water_level[mapidx(x, y + 1, z - 1)] > 0
                    || region.water_level[mapidx(x - 1, y, z - 1)] > 0
                    || region.water_level[mapidx(x + 1, y, z - 1)] > 0
                {
                    region.material_idx[idx] = yellow_sand;
                    // TODO: Clear vegetation
                }
            }
        }
    }
}
