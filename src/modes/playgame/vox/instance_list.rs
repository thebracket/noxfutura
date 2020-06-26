use legion::prelude::*;
use crate::components::*;
use crate::modes::playgame::vox::VoxBuffer;
use crate::modes::playgame::ChunkModel;
use crate::engine::VertexBuffer;

pub fn build_vox_instances(
    ecs: &World,
    camera_z: usize,
    vox_models: &VoxBuffer,
    instance_buffer: &mut VertexBuffer<f32>,
    chunk_models: &[ChunkModel]
) -> Vec<(u32, u32, i32)> {
    // Instances builder
    instance_buffer.clear();
    let mut vox_instances = Vec::new();
    let query = <(
        Read<Position>,
        Read<VoxelModel>,
        Read<Dimensions>,
        Read<Tint>,
    )>::query();
    let mut n = 0;
    for (pos, vm, dims, tint) in query.iter(&ecs) {
        if pos.z <= camera_z {
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
            instance_buffer
                .add3(tint.color.0, tint.color.1, tint.color.2);
        }
    }

    // Composite Models
    let query = <(Read<Position>, Read<CompositeRender>)>::query();
    for (pos, composite) in query.iter(&ecs) {
        if pos.z <= camera_z {
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
    for m in chunk_models {
        if m.z <= camera_z {
            let first = vox_models.offsets[m.id].0;
            let last = vox_models.offsets[m.id].1;
            vox_instances.push((first, last, n));
            n += 1;

            instance_buffer
                .add3(m.x as f32, m.z as f32, m.y as f32);
            instance_buffer.add3(1.0, 1.0, 1.0);
        }
    }

    if !vox_instances.is_empty() {
        instance_buffer.update_buffer();
    }

    vox_instances
}