use crate::{
    rex::XpFile,
    simulation::{
        mapidx,
        terrain::{ground_z, submit_change_batch, ChangeRequest, MapChangeBatch, PlanetLocation},
        REGION_HEIGHT, REGION_WIDTH,
    },
};
use std::fs::File;

fn load_ship() -> XpFile {
    let mut f = File::open("assets/rex/spaceship.xp").unwrap();
    XpFile::read(&mut f).unwrap()
}

pub fn build_escape_pod(region_id: PlanetLocation) {
    let crash_x = REGION_WIDTH / 2;
    let crash_y = REGION_HEIGHT / 2;
    let ship = load_ship();
    let z = ground_z(region_id, crash_x, crash_y) - 1;

    let plasteel = crate::raws::RAWS.read().materials.find_by_name("Plasteel");

    let mut changes = MapChangeBatch::new(region_id);
    for (i, layer) in ship.layers.iter().enumerate() {
        for y in 0..layer.height {
            for x in 0..layer.width {
                let mx = (x as i32 - 5 + crash_x as i32) as usize;
                let my = (y as i32 - 11 + crash_y as i32) as usize;
                let mz = z + i;
                let tile_idx = mapidx(mx as usize, my as usize, mz);

                let glyph = layer.get(x, y);
                if let Some(glyph) = glyph {
                    changes.enqueue_change(ChangeRequest::RevealTile { idx: tile_idx });
                    match glyph.ch {
                        32 => {} // Ignore space
                        219 => changes.enqueue_change(ChangeRequest::SolidTile {
                            idx: tile_idx,
                            material: plasteel,
                        }),
                        _ => println!("No decoder for glyph {} in spaceship", glyph.ch),
                    }
                }
            }
        }
    }
    submit_change_batch(changes);
}
