use crate::{
    simulation::{REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH},
    ui::UiResources,
};
use bevy::{prelude::*, render::camera::Camera};

pub fn spawn_game_camera(
    commands: &mut Commands,
    tile_x: usize,
    tile_y: usize,
    x: usize,
    y: usize,
    z: usize,
) {
    let camera_x = tile_x as f32 * REGION_WIDTH as f32 + x as f32;
    let camera_y = tile_y as f32 * REGION_HEIGHT as f32 + y as f32;
    let camera_z = z as f32;

    let game_camera = GameCamera {
        tile_x,
        tile_y,
        x,
        y,
        z,
        mode: CameraMode::TopDown,
        zoom: 20,
    };

    let look_at = game_camera.look_at();

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(camera_x, camera_y, camera_z)
                .looking_at(look_at, Vec3::Z),
            ..Default::default()
        })
        .insert(game_camera.clone());

    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(camera_x, camera_y, camera_z),
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
        let camera_x = (self.tile_x as f32 * REGION_WIDTH as f32) + self.x as f32;
        let camera_y = (self.tile_y as f32 * REGION_HEIGHT as f32) + self.y as f32;
        let camera_z = self.z as f32;
        Vec3::new(camera_x, camera_y, camera_z)
    }

    fn look_at(&self) -> Vec3 {
        match self.mode {
            CameraMode::TopDown => self.pos_world() + Vec3::new(0.0, 20.0, -self.pos_world().z),
            CameraMode::DiagonalNW => {
                self.pos_world() + Vec3::new(self.zoom as f32, self.zoom as f32, -self.zoom as f32)
            }
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

pub fn game_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut GameCamera)>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    ui_resources: Res<UiResources>,
    mut commands: Commands,
) {
    let mut moved = false;
    for (mut trans, mut game_camera) in camera_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            if game_camera.x > 0 {
                game_camera.x -= 1;
                moved = true;
            } else {
                //game_camera.x = REGION_WIDTH - 1;
                //game_camera.tile_x -= 1;
            }
        }
        if keyboard_input.pressed(KeyCode::Right) {
            if game_camera.x < REGION_WIDTH - 2 {
                game_camera.x += 1;
                moved = true;
            } else {
                //game_camera.x = 0;
                //game_camera.tile_x += 1;
            }
        }
        if keyboard_input.pressed(KeyCode::Down) {
            if game_camera.y > 0 {
                game_camera.y -= 1;
                moved = true;
            }
        }
        if keyboard_input.pressed(KeyCode::Up) {
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
        if keyboard_input.pressed(KeyCode::Tab) {
            if game_camera.mode == CameraMode::TopDown {
                game_camera.mode = CameraMode::DiagonalNW;
            } else if game_camera.mode == CameraMode::DiagonalNW {
                game_camera.mode = CameraMode::TopDown;
            }
            moved = true;
        }

        if moved {
            //println!("Game camera movement detected.");
            trans.translation = game_camera.pos_world();
            let target = game_camera.look_at();
            //println!("Camera: {:?}", game_camera.pos_world());
            //let target = Vec3::new(17152.,13056., 144.);
            //println!("Look at: {:?}", target);
            trans.look_at(target, Vec3::Z);
            crate::simulation::terrain::CHUNK_STORE
                .write()
                .manage_for_camera(
                    &game_camera,
                    &mut mesh_assets,
                    ui_resources.world_material_handle.clone(),
                    &mut commands,
                );
        }
    }
}
