use std::f32::consts::PI;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_objects);
    }
}

fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((PbrBundle {
        mesh: meshes.add(shape::Box::new(200.0, 0.2, 200.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },));

    // Cube
    commands.spawn((PbrBundle {
        mesh: meshes.add(shape::Cube::new(2.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::ORANGE,
            perceptual_roughness: 1.0,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    },));

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10_000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.0),
            ..default()
        },
        ..default()
    });
}
