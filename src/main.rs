use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::thread::spawn;

// TODO - Remove or refactor the use of these variables
// variables for setting defaults for display size
pub const HEIGHT: f32 = 1080.0;
pub const WIDTH: f32 = 1920.0;

// new tower component with a Timer
#[derive(Component)]
pub struct Tower {
    shooting_timer: Timer,
}

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
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(5.0).into()),
            material: materials.add(Color::rgb(0.5, 0.0, 0.0).into()),
            ..default()
        })
        .insert(Name::new("Flat Plane")); //entity name

    // capsule, Tower
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.3, 0.2, 0.9).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }) // insert tower component
        .insert(Name::new("Tower")); //entity name
}
fn spawn_light(mut commands: Commands) {
    // light
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 3750.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("PointLight")); //entity name
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2))) // set background color for scene
        .add_startup_system(spawn_basic_scene) // create a basic scene
        .add_startup_system(spawn_camera) // spawn a camera
        .add_startup_system(spawn_light) // spawn a light
        .add_plugins(DefaultPlugins) // add default bevy plugins
        .add_plugin(WorldInspectorPlugin::new()) // add the plugin for setting up inspector
        .add_plugin(FrameTimeDiagnosticsPlugin::default()) // add the plugin for displaying fps
        .run();
}
