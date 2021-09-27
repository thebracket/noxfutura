use crate::simulation::{
    mapidx,
    terrain::{ground_z, submit_change_batch, ChangeRequest, MapChangeBatch, PlanetLocation},
};

pub(crate) fn debris_trail(region_id: PlanetLocation) {
    let ship_x = 128;
    let ship_y = 128;

    let mut changes = MapChangeBatch::new(region_id);
    for x in ship_x - 30..ship_x {
        for y in ship_y - 5..ship_y + 5 {
            let z = ground_z(region_id, x, y);
            let tile_idx = mapidx(x, y, z);
            changes.enqueue_change(ChangeRequest::NoVegetation { idx: tile_idx });
        }
    }
    submit_change_batch(changes);
}
