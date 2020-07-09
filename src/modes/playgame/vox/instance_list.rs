use super::super::chunks::Chunks;
use super::super::frustrum::Frustrum;
use crate::engine::VertexBuffer;
use crate::modes::playgame::vox::VoxBuffer;
use legion::prelude::*;
use nox_components::*;
use std::collections::HashMap;

const FRUSTRUM_CHECK_RANGE: f32 = 2.0;

#[derive(Debug)]
pub struct VMRender {
    position: [f32; 3],
    tint: [f32; 3],
    rotation: f32
}

#[derive(Debug)]
pub struct VMInstances {
    instances: HashMap<usize, Vec<VMRender>>
}

impl VMInstances {
    pub fn new() -> Self {
        Self{
            instances: HashMap::new()
        }
    }

    fn add(&mut self, model_id : usize, position: [f32; 3], tint: [f32; 3], rotation: f32) {
        if let Some(vmi) = self.instances.get_mut(&model_id) {
            vmi.push(
                VMRender{
                    position,
                    tint,
                    rotation
                }
            );
        } else {
            self.instances.insert(model_id, 
                vec![
                    VMRender{
                        position,
                        tint,
                        rotation
                    }
                ]
            );
        }
    }
}

const LAYERS_DOWN : usize = 20;

pub fn build_vox_instances2(
    ecs: &World,
    camera_z: usize,
    vox_models: &VoxBuffer,
    instance_buffer: &mut VertexBuffer<f32>,
    vox_instances: &mut Vec<(u32, u32, u32)>,
    frustrum: &Frustrum,
    chunks: &Chunks,
) {
    let mut instances = VMInstances::new();
    instance_buffer.clear();
    vox_instances.clear();

    // Models from the ECS
    let query = <(
        Read<Position>,
        Read<VoxelModel>,
        Read<Dimensions>,
        Read<Tint>,
    )>::query();
    query
        .iter(ecs)
        .filter(|(pos, _, _, _)| {
            pos.z > camera_z-LAYERS_DOWN && pos.z <= camera_z && frustrum.check_sphere(
                &(pos.x as f32, pos.y as f32, pos.z as f32).into(),
                FRUSTRUM_CHECK_RANGE,
            )
        })
        .for_each(|(pos, model, dims, tint)| {
            let mut x = pos.x as f32;
            let mut y = pos.y as f32;
            let z = pos.z as f32;

            if dims.width == 3 {
                x -= 1.0;
            }
            if dims.height == 3 {
                y -= 1.0;
            }

            instances.add(
                model.index, 
                [x, z, y], 
                [tint.color.0, tint.color.1, tint.color.2],
                model.rotation_radians
            );
        }
    );

    // Composite builder
    let query = <(Read<Position>, Read<CompositeRender>)>::query();
    query
        .iter(ecs)
        .filter(|(pos, _)| {
            pos.z > camera_z - LAYERS_DOWN && pos.z <= camera_z
                && frustrum.check_sphere(
                    &(pos.x as f32, pos.y as f32, pos.z as f32).into(),
                    FRUSTRUM_CHECK_RANGE,
                )
        })
        .for_each(|(pos, composite)| {
            for vm in composite.layers.iter() {
                let x = pos.x as f32;
                let y = pos.y as f32;
                let z = pos.z as f32;

                instances.add(
                    vm.model, 
                    [x, z, y], 
                    [vm.tint.0, vm.tint.1, vm.tint.2],
                    composite.rotation
                );
            }
        }
    );

    // Terrain chunks builder
    chunks.visible_chunks()
        .iter()
        .for_each(|c| {
            c.chunk_models
                .iter()
                .filter(|m| m.z > camera_z - LAYERS_DOWN && m.z <= camera_z)
                .for_each(|m| {
                    instances.add(
                        m.id, 
                        [m.x as f32, m.z as f32, m.y as f32], 
                        m.tint, 
                        m.rotation
                    );
                }
            );
        }
    );

    // Build the instanced data
    instances.instances.iter().for_each(|i| {
        vox_instances.push(
            (
                vox_models.offsets[*i.0].0,
                vox_models.offsets[*i.0].1,
                i.1.len() as u32
            )
        );
        i.1.iter().for_each(|vm| {
            instance_buffer.add_slice(&vm.position);
            instance_buffer.add_slice(&vm.tint);
            instance_buffer.add(vm.rotation);
        });
    });

    // Push the buffer update
    if !vox_instances.is_empty() {
        instance_buffer.update_buffer();
    }
}
