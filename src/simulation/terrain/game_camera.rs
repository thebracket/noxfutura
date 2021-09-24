use crate::simulation::{REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH};
use bevy::prelude::*;

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

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(camera_pos.x, camera_pos.y, camera_pos.z + 128.0),
        light: Light {
            color: Color::rgb(1.0, 1.0, 1.0),
            fov: 360.0,
            depth: -512.0..512.0,
            range: 512.0,
            intensity: 50_000.0,
        },
        ..Default::default()
    });
    //.insert(game_camera);
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
        if keyboard_input.pressed(KeyCode::Comma) {
            if game_camera.z < REGION_DEPTH - 2 {
                game_camera.z += 1;
                moved = true;
            }
        }
        if keyboard_input.pressed(KeyCode::Period) {
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
