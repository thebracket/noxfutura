use legion::prelude::*;
use crate::components::*;
use ultraviolet::Vec3;
use crate::utils::mapidx;
use crate::planet::{REGION_WIDTH, REGION_HEIGHT, REGION_DEPTH, Region};
use crate::modes::playgame::REGION;

pub fn build() -> Box<dyn Schedulable> {
    SystemBuilder::new("calendar")
    .with_query(<(Read<Position>, Write<FieldOfView>)>::query())
    .build(|_, ecs, _, fov_list| {
        fov_list
            .iter_mut(ecs)
            .filter(|(_, fov)| fov.is_dirty)
            .for_each(|(pos, mut fov)| {
            //println!("{:?}", fov);
                fov.visible_tiles.clear();
                let radius = fov.radius as i32;
                reveal(mapidx(pos.x, pos.y, pos.z), &mut *fov);
                let radius_range = (0i32 - radius) .. radius;
                for z in radius_range {
                    for i in (0i32 - radius) .. radius {
                        internal_view_to(&*pos, &mut *fov, i as i32, radius as i32, z as i32);
                        internal_view_to(&*pos, &mut *fov, i as i32, 0i32 - radius as i32, z as i32);
                        internal_view_to(&*pos, &mut *fov, radius as i32, i as i32, z as i32);
                        internal_view_to(&*pos, &mut *fov, 0i32 - radius as i32, i as i32, z as i32);
                    }
                }
                fov.is_dirty = false;
            });
    })
}

#[inline(always)]
fn internal_view_to(pos: &Position, fov: &mut FieldOfView, x: i32, y: i32, z: i32) {
    let radius = fov.radius as f32;
    let start : Vec3 = (pos.x as f32, pos.y as f32, pos.z as f32).into();
    let end : Vec3 = (x as f32 + start.x, y as f32 + start.y, z as f32 + start.z).into();
    let mut blocked = false;
    line_func_3d(start, end, |pos| {
        if pos.x > 0.0 && pos.x < REGION_WIDTH as f32 && pos.y > 0.0 && pos.y < REGION_HEIGHT as f32 && pos.z > 0.0 && pos.z < REGION_DEPTH as f32 {
            let distance = (pos - start).abs().mag();
            if distance < radius {
                let idx = mapidx(pos.x as usize, pos.y as usize, pos.z as usize);
                if !blocked {
                    reveal(idx, fov);
                }
                if REGION.read().flag(idx, Region::SOLID) {
                    blocked = true;
                }
            }
        }
    });
}

fn line_func_3d<F: FnMut(Vec3)>(start: Vec3, end: Vec3, mut func : F) {
    //println!("{:?} -> {:?}", start, end);
    let mut pos = start.clone();
    let length = (start - end).abs().mag();
    //println!("{:?}", length);
    let step = (start - end) / length;
    for _ in 0 .. f32::floor(length) as usize {
        pos += step;
        func(pos);
    }
}

fn reveal(idx: usize, view:&mut FieldOfView) {
    REGION.write().revealed[idx] = true; // TODO: Make conditional
    view.visible_tiles.insert(idx);
}