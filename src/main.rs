use std::thread::spawn;

use bevy::prelude::*;

pub const HEIGHT: f32 = 1080.0;
pub const WIDTH: f32 = 1920.0;

fn spawn_camera(mut commands: Commands) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.5, 0.0, 0.0).into()),
        ..default()
    });
    // capsule
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.5,
            ..Default::default()
        })),
        material: materials.add(Color::rgb(0.3, 0.2, 0.9).into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });
}

fn spawn_light(mut commands: Commands) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3750.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_light)
        .add_plugins(DefaultPlugins)
        .run();
}
