use super::super::chunks::Chunks;
use super::super::frustrum::Frustrum;
use crate::components::*;
use crate::engine::VertexBuffer;
use crate::modes::playgame::vox::VoxBuffer;
use legion::prelude::*;

const FRUSTRUM_CHECK_RANGE: f32 = 2.0;

pub fn build_vox_instances(
    ecs: &World,
    camera_z: usize,
    vox_models: &VoxBuffer,
    instance_buffer: &mut VertexBuffer<f32>,
    vox_instances: &mut Vec<(u32, u32, i32)>,
    frustrum: &Frustrum,
    chunks: &Chunks,
) {
    // Instances builder
    instance_buffer.clear();
    vox_instances.clear();
    let query = <(
        Read<Position>,
        Read<VoxelModel>,
        Read<Dimensions>,
        Read<Tint>,
    )>::query();
    let mut n = 0;
    for (pos, vm, dims, tint) in query.iter(&ecs) {
        if pos.z <= camera_z
            && frustrum.check_sphere(
                &(pos.x as f32, pos.y as f32, pos.z as f32).into(),
                FRUSTRUM_CHECK_RANGE,
            )
        {
            let first = vox_models.offsets[vm.index].0;
            let last = vox_models.offsets[vm.index].1;
            vox_instances.push((first, last, n));
            n += 1;

            let mut x = pos.x as f32;
            let mut y = pos.y as f32;
            let z = pos.z as f32;

            if dims.width == 3 {
                x -= 1.0;
            }
            if dims.height == 3 {
                y -= 1.0;
            }

            instance_buffer.add3(x, z, y);
            instance_buffer.add3(tint.color.0, tint.color.1, tint.color.2);
        }
    }

    // Composite Models
    let query = <(Read<Position>, Read<CompositeRender>)>::query();
    for (pos, composite) in query.iter(&ecs) {
        if pos.z <= camera_z
            && frustrum.check_sphere(
                &(pos.x as f32, pos.y as f32, pos.z as f32).into(),
                FRUSTRUM_CHECK_RANGE,
            )
        {
            for vm in composite.layers.iter() {
                let first = vox_models.offsets[vm.model].0;
                let last = vox_models.offsets[vm.model].1;
                vox_instances.push((first, last, n));
                n += 1;

                let x = pos.x as f32;
                let y = pos.y as f32;
                let z = pos.z as f32;

                instance_buffer.add3(x, z, y);
                instance_buffer.add3(vm.tint.0, vm.tint.1, vm.tint.2);
            }
        }
    }

    // Terrain chunk models
    chunks.visible_chunks().iter().for_each(|c| {
        for m in c.chunk_models.iter() {
            let first = vox_models.offsets[m.id].0;
            let last = vox_models.offsets[m.id].1;
            vox_instances.push((first, last, n));
            n += 1;

            instance_buffer.add3(m.x as f32, m.z as f32, m.y as f32);
            instance_buffer.add3(1.0, 1.0, 1.0);
        }
    });
    /*
    for m in chunk_models {
        if m.z <= camera_z&& frustrum.check_sphere(&(m.x as f32, m.y as f32, m.z as f32).into(), FRUSTRUM_CHECK_RANGE) {
            let first = vox_models.offsets[m.id].0;
            let last = vox_models.offsets[m.id].1;
            vox_instances.push((first, last, n));
            n += 1;

            instance_buffer.add3(m.x as f32, m.z as f32, m.y as f32);
            instance_buffer.add3(1.0, 1.0, 1.0);
        }
    }*/

    if !vox_instances.is_empty() {
        instance_buffer.update_buffer();
    }
}
