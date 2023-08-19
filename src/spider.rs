use std::f32::consts::PI;

use bevy::prelude::*;

const MOVE_SPEED: f32 = 4.0;

const BODY_COLOR: Color = Color::BLACK;

pub struct SpiderPlugin;

impl Plugin for SpiderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spider)
            .add_systems(Update, (move_from_input, draw_spider));
    }
}

#[derive(Component)]
struct Spider;

fn spawn_spider(mut commands: Commands) {
    commands.spawn((
        Spider,
        TransformBundle {
            local: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
    ));
}

fn move_from_input(
    mut spider_transform: Query<&mut Transform, With<Spider>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = spider_transform.get_single_mut() {
        let move_input = get_wasd_input_as_vector(&input);

        transform.translation += move_input * time.delta_seconds() * MOVE_SPEED;
    }
}

fn get_wasd_input_as_vector(input: &Res<Input<KeyCode>>) -> Vec3 {
    let mut result = Vec3::ZERO;

    if input.pressed(KeyCode::W) {
        result.z -= 1.0;
    }
    if input.pressed(KeyCode::S) {
        result.z += 1.0;
    }
    if input.pressed(KeyCode::A) {
        result.x -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        result.x += 1.0;
    }

    result.normalize_or_zero()
}

// Gizmos

fn draw_spider(mut gizmos: Gizmos, spider: Query<&Transform, With<Spider>>) {
    if let Ok(transform) = spider.get_single() {
        gizmos.rect(
            transform.translation,
            Quat::from_rotation_x(PI / 2.0),
            Vec2::new(1.0, 1.0),
            BODY_COLOR,
        );
    }
}
