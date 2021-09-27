use super::UiResources;
use crate::{simulation::planet_builder::PlanetBuilder, AppState};
use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use bevy_egui::{
    egui::{self, Pos2},
    EguiContext,
};

pub struct WorldGenPlanet;
pub struct WorldGenUi;

pub fn planet_builder_menu(
    egui_context: ResMut<EguiContext>,
    mut spinner: Query<&mut Transform, With<WorldGenPlanet>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    mut planet_builder: ResMut<PlanetBuilder>,
    res: Res<UiResources>,
    mut state: ResMut<State<AppState>>,
) {
    planet_builder.start(&res.worldgen_seed, res.worldgen_lacunarity);
    if planet_builder.is_done() {
        // Bail out
        state.set(AppState::MainMenu).unwrap();
    }

    egui::Window::new("Planet Builder")
        .title_bar(true)
        .fixed_pos(Pos2::new(25.0, 25.0))
        .show(egui_context.ctx(), |ui| {
            ui.label(&planet_builder.get_status());
        });

    if let Some(planet) = planet_builder.globe_info() {
        if let Some(mesh) = meshes.get_mut(planet_builder.globe_mesh_handle.as_ref().unwrap()) {
            mesh.set_attribute(
                Mesh::ATTRIBUTE_POSITION,
                VertexAttributeValues::Float3(planet.vertices.to_owned()),
            );
            mesh.set_attribute(
                Mesh::ATTRIBUTE_NORMAL,
                VertexAttributeValues::Float3(planet.normals.to_owned()),
            );
            mesh.set_attribute(
                Mesh::ATTRIBUTE_UV_0,
                VertexAttributeValues::Float2(planet.uv.to_owned()),
            );
        }
    }

    // Spin the globe
    for mut transform in spinner.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(0.5 * time.delta_seconds());
    }
}

pub fn resume_planet_builder_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ui_resources: Res<UiResources>,
) {
    println!("Building globe");

    use crate::simulation::planet_builder::PlanetMesh;
    let mut planet = PlanetMesh::new();
    planet.totally_round(1.0);

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(ui_resources.worldgen_tex.clone()),
        roughness: 0.5,
        unlit: false,
        ..Default::default()
    });

    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float3(planet.vertices),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float3(planet.normals),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_UV_0,
        VertexAttributeValues::Float2(planet.uv),
    );

    let globe_mesh_handle = meshes.add(mesh);

    // Planet
    commands
        .spawn_bundle(PbrBundle {
            mesh: globe_mesh_handle.clone(),
            material: material_handle,
            ..Default::default()
        })
        .insert(WorldGenPlanet {})
        .insert(WorldGenUi {});

    // light
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(0.0, 15.0, 0.0),
            light: Light {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(WorldGenUi {});
    // Camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
            ..Default::default()
        })
        .insert(WorldGenUi {});

    // Get the builder inserted
    let mut pb = PlanetBuilder::new();
    pb.globe_mesh_handle = Some(globe_mesh_handle);
    commands.insert_resource(pb);
}

pub fn planet_builder_exit(mut commands: Commands, q: Query<(Entity, &WorldGenUi)>) {
    q.iter().for_each(|(e, _)| commands.entity(e).despawn());
    commands.remove_resource::<PlanetBuilder>();
}
