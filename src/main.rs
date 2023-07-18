use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::Rng;
use std::f32::consts::PI;
use std::thread::spawn;

// TODO - Remove or refactor the use of these variables
// variables for setting defaults for display size
pub const HEIGHT: f32 = 1080.0;
pub const WIDTH: f32 = 1920.0;

// COMPONENTS
// ----------------------------------------------
// These are examples of components
// new tower component with a Timer
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target {
    speed: f32,
    direction: Vec3,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    value: i32,
}
// ----------------------------------------------

// CAN REMOVE
// fn move_targets(
//     mut commands: Commands,
//     mut targets: Query<(Entity, &mut Target)>, // Entity is only one don't need & or &mut
//     time: Res<Time>,
// ) {
//     // iterate over Lifetimes on bulet component
//     // only gets entities with Lifetime component
//     for (entity, mut target) in &mut targets {
//         //target.transform.translation += target.direction * time.delta_seconds();

//         // check if timer just finished and despawn bullet
//         if lifetime.timer.just_finished() {
//             commands.entity(entity).despawn_recursive(); // almost always use despawn_recursive
//         }
//     }
// }
// CAN REMOVE
fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<&mut Tower>, // gets all tower components in game. can only use &mut or & for Query
    time: Res<Time>,
) {
    // get a random number between 0.0 and 4.0 for some fun
    let mut rng = rand::thread_rng();
    let random_float: f32 = rng.gen_range(0.0..4.0);
    // iterate over tower components queried
    for mut tower in &mut towers {
        // tick the timer
        tower.shooting_timer.tick(time.delta());
        // check if timer just finished and spawn bullet
        if tower.shooting_timer.just_finished() {
            // create new transform for bullet
            let spawn_transform = Transform::from_xyz(0.0, 0.7, random_float)
                .with_rotation(Quat::from_rotation_y(-PI / 2.0));
            // pbr bundle - bullet
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    transform: spawn_transform,
                    ..default()
                })
                .insert(Lifetime {
                    // add a lifetime component to the bullet
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                })
                .insert(Name::new("Bullet"));
        }
    }
}
// CAN REMOVE
fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    // iterate over Lifetimes on bulet component
    // only gets entities with Lifetime component
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        // check if timer just finished and despawn bullet
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive(); // almost always use despawn_recursive
        }
    }
}
// Simple camera spawn
fn spawn_camera(mut commands: Commands) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
// Simple basic scene
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

    // basic shape moving
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: 0.1,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.5, 0.7, 0.4).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(Target {
            speed: 4.0,
            direction: Vec3::Y,
        })
        .insert(Name::new("Moving Target")); //entity name
}
// Simple light spawn
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
        .register_type::<Tower>() // Register Custom Tower component
        .register_type::<Lifetime>() // Register Lifetime component
        .register_type::<Target>() // Register Target component
        .register_type::<Health>() // Register Health component
        .add_plugins(DefaultPlugins) // add default bevy plugins
        .add_plugin(WorldInspectorPlugin::new()) // add the plugin for setting up inspector
        .add_plugin(FrameTimeDiagnosticsPlugin::default()) // add the plugin for displaying fps
        .add_system(tower_shooting)
        .add_system(bullet_despawn) // despawn bullet
        .add_system(keyboard_input_system)
        .run();
}

// input system
/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::W) {
        info!("'W' currently pressed");
    }

    if keyboard_input.pressed(KeyCode::A) {
        info!("'A' currently pressed");
    }

    if keyboard_input.pressed(KeyCode::S) {
        info!("'S'  currently pressed");
    }

    if keyboard_input.pressed(KeyCode::D) {
        info!("'D'  currently pressed");
    }
}
