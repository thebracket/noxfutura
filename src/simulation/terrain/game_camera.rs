use std::collections::HashSet;

use super::{RenderChunk, PLANET_STORE};
use crate::simulation::{REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH, WORLD_WIDTH};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;

use super::{
    chunk_mesh::chunk_to_mesh,
    region_chunk::ChunkBuilderTask,
    region_chunk_state::{ChunkMesh, ChunkStatus},
};

pub fn spawn_game_camera(
    commands: &mut Commands,
    tile_x: usize,
    tile_y: usize,
    x: usize,
    y: usize,
    z: usize,
) {
    let game_camera = GameCamera {
        tile_x,
        tile_y,
        x,
        y,
        z,
        mode: CameraMode::TopDown,
        zoom: 20,
    };

    let camera_pos = game_camera.pos_world();
    let look_at = game_camera.look_at();

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(camera_pos.x, camera_pos.y, camera_pos.z)
                .looking_at(look_at, Vec3::Z),
            ..Default::default()
        })
        .insert(game_camera.clone());

    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(camera_pos.x, camera_pos.y, camera_pos.z),
            light: Light {
                color: Color::rgb(1.0, 1.0, 1.0),
                fov: 90.0,
                depth: -256.0..256.0,
                range: 256.0,
                intensity: 5000.0,
            },
            ..Default::default()
        })
        .insert(game_camera);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraMode {
    TopDown,
    Front,
    DiagonalNW,
    DiagonalNE,
    DiagonalSW,
    DiagonalSE,
}

#[derive(Clone)]
pub struct GameCamera {
    pub tile_x: usize,
    pub tile_y: usize,
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub mode: CameraMode,
    pub zoom: i32,
}

impl GameCamera {
    pub fn pos_world(&self) -> Vec3 {
        match self.mode {
            CameraMode::Front => {
                self.look_at() + Vec3::new(0.0, self.zoom as f32 / 3.0, self.zoom as f32)
            }
            CameraMode::TopDown => self.look_at() + Vec3::new(0.0, 0.1, self.zoom as f32),
            CameraMode::DiagonalNW => {
                self.look_at() - Vec3::new(self.zoom as f32, self.zoom as f32, -self.zoom as f32)
            }
            CameraMode::DiagonalNE => {
                self.look_at() - Vec3::new(-self.zoom as f32, self.zoom as f32, -self.zoom as f32)
            }
            CameraMode::DiagonalSW => {
                self.look_at() - Vec3::new(self.zoom as f32, -self.zoom as f32, -self.zoom as f32)
            }
            CameraMode::DiagonalSE => {
                self.look_at() - Vec3::new(-self.zoom as f32, -self.zoom as f32, -self.zoom as f32)
            }
        }
    }

    pub fn look_at(&self) -> Vec3 {
        let camera_x = (self.tile_x as f32 * REGION_WIDTH as f32) + self.x as f32;
        let camera_y = (self.tile_y as f32 * REGION_HEIGHT as f32) + self.y as f32;
        let camera_z = self.z as f32;
        Vec3::new(camera_x, camera_y, camera_z)
    }
}

pub fn game_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut GameCamera)>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    task_master: Res<AsyncComputeTaskPool>,
) {
    let mut moved = false;
    for (mut trans, mut game_camera) in camera_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Right) {
            if game_camera.x > 0 {
                game_camera.x -= 1;
                moved = true;
            }
        }
        if keyboard_input.pressed(KeyCode::Left) {
            if game_camera.x < REGION_WIDTH - 2 {
                game_camera.x += 1;
                moved = true;
            }
        }
        if keyboard_input.pressed(KeyCode::Up) {
            if game_camera.y > 0 {
                game_camera.y -= 1;
                moved = true;
            }
        }
        if keyboard_input.pressed(KeyCode::Down) {
            if game_camera.y < REGION_HEIGHT - 2 {
                game_camera.y += 1;
                moved = true;
            }
        }
        if keyboard_input.just_pressed(KeyCode::Comma) {
            if game_camera.z < REGION_DEPTH - 2 {
                game_camera.z += 1;
                moved = true;
            }
        }
        if keyboard_input.just_pressed(KeyCode::Period) {
            if game_camera.z > 0 {
                game_camera.z -= 1;
                moved = true;
            }
        }
        if keyboard_input.just_pressed(KeyCode::Tab) {
            game_camera.mode = next_camera(game_camera.mode);
            moved = true;
        }

        if moved {
            trans.translation = game_camera.pos_world();
            let target = game_camera.look_at();
            trans.look_at(target, Vec3::Z);
            crate::simulation::terrain::CHUNK_STORE
                .write()
                .manage_for_camera(
                    &game_camera,
                    &mut mesh_assets,
                    &mut commands,
                    task_master.clone(),
                );
        }
    }
}

fn next_camera(mode: CameraMode) -> CameraMode {
    match mode {
        CameraMode::TopDown => CameraMode::Front,
        CameraMode::Front => CameraMode::DiagonalNE,
        CameraMode::DiagonalNE => CameraMode::DiagonalNW,
        CameraMode::DiagonalNW => CameraMode::DiagonalSE,
        CameraMode::DiagonalSE => CameraMode::DiagonalSW,
        CameraMode::DiagonalSW => CameraMode::TopDown,
    }
}

pub fn manage_terrain_tasks(
    mut commands: Commands,
    mut generators: Query<(Entity, &mut Task<ChunkBuilderTask>)>,
    mut meshers: Query<(Entity, &mut Task<MeshBuilderTask>)>,
    task_master: Res<AsyncComputeTaskPool>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    chunk_meshes: Query<(Entity, &RenderChunk)>,
) {
    let mut lock = super::CHUNK_STORE.write();
    let mut chunk_meshes_to_delete = HashSet::new();
    lock.regions.iter_mut().for_each(|(_pidx, r)| {
        for t in r.chunk_builder_tasks.drain(..) {
            commands.spawn().insert(t);
        }
        for c in r.chunk_meshes_to_delete.drain(..) {
            chunk_meshes_to_delete.insert(c);
        }
    });
    std::mem::drop(lock);

    if !chunk_meshes_to_delete.is_empty() {
        chunk_meshes.iter().for_each(|(entity, cm)| {
            if chunk_meshes_to_delete.contains(&cm.0) {
                commands.entity(entity).despawn();
            }
        });
    }

    for (entity, mut task) in generators.iter_mut() {
        if let Some(task) = future::block_on(future::poll_once(&mut *task)) {
            let mut lock = super::CHUNK_STORE.write();
            if let Some(region) = lock.regions.get_mut(&task.planet_idx) {
                let chunk_copy = task.chunk.clone();
                let planet_idx = task.planet_idx;
                let chunk_id = task.chunk_id;
                region.chunks[task.chunk_id].chunk = Some(task.chunk);
                region.chunks[task.chunk_id].status = ChunkStatus::AsyncMeshing;

                let task = task_master.spawn(async move {
                    let mesh = chunk_to_mesh(&chunk_copy);
                    MeshBuilderTask {
                        mesh,
                        planet_idx: planet_idx,
                        chunk_id: chunk_id,
                    }
                });
                commands.spawn().insert(task);
            }
            commands.entity(entity).despawn();
        }
    }

    for (entity, mut task) in meshers.iter_mut() {
        if let Some(task) = future::block_on(future::poll_once(&mut *task)) {
            let mut lock = super::CHUNK_STORE.write();
            if let Some(region) = lock.regions.get_mut(&task.planet_idx) {
                if task.mesh.is_some() {
                    let tile_x = task.planet_idx % WORLD_WIDTH;
                    let tile_y = task.planet_idx / WORLD_WIDTH;

                    for (mat, mesh) in task.mesh.unwrap().drain(..) {
                        let asset_handle = mesh_assets.add(mesh);
                        if let Some(mesh_list) = &mut region.chunks[task.chunk_id].mesh {
                            mesh_list.push(ChunkMesh(asset_handle.clone()));
                        } else {
                            region.chunks[task.chunk_id].mesh = Some(vec![ChunkMesh(asset_handle.clone())]);
                        }
                        let chunk_id = region.chunks[task.chunk_id].id;
                        let mx = (tile_x * REGION_WIDTH) as f32;
                        let my = (tile_y * REGION_HEIGHT) as f32;
                        let mz = 0.0;
                        commands
                            .spawn_bundle(PbrBundle {
                                mesh: asset_handle.clone(),
                                material: PLANET_STORE
                                    .read()
                                    .world_material_handle
                                    .as_ref()
                                    .unwrap()
                                    [mat]
                                    .clone(),
                                transform: Transform::from_xyz(mx, my, mz),
                                ..Default::default()
                            })
                            .insert(RenderChunk(chunk_id));
                    }
                }
                region.chunks[task.chunk_id].status = ChunkStatus::Loaded;
            }
            commands.entity(entity).despawn();
        }
    }
}

pub struct MeshBuilderTask {
    pub mesh: Option<Vec<(usize, Mesh)>>,
    pub planet_idx: usize,
    pub chunk_id: usize,
}
