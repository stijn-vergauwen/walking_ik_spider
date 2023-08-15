use bevy::prelude::*;

use super::{solve_chain_towards_target, IkChain};

const TARGET_RADIUS: f32 = 0.7;
const TARGET_COLOR: Color = Color::LIME_GREEN;
const TARGET_MOVE_SPEED: f32 = 2.0;

const FABRIK_ITERATIONS: i32 = 1;

pub struct IkLegPlugin;

impl Plugin for IkLegPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_test_leg, spawn_test_target))
            .add_systems(
                Update,
                (
                    move_test_target_from_input,
                    move_test_leg_to_target,
                    draw_test_target_gizmo,
                ),
            );
    }
}

#[derive(Component)]
struct TestLeg;

#[derive(Component)]
struct TestTarget;

fn spawn_test_leg(mut commands: Commands) {
    let test_points = vec![
        Vec3::new(-4.0, 1.0, 0.0),
        Vec3::new(-3.0, 3.0, 0.0),
        Vec3::new(-2.0, 1.0, 0.0),
    ];

    commands.spawn((IkChain::new(test_points), TestLeg));
}

fn move_test_leg_to_target(
    mut test_leg: Query<&mut IkChain, With<TestLeg>>,
    test_target: Query<&GlobalTransform, With<TestTarget>>,
) {
    if let Ok(mut leg) = test_leg.get_single_mut() {
        if let Ok(target) = test_target.get_single() {
            solve_chain_towards_target(&mut leg, target.translation(), FABRIK_ITERATIONS);
        }
    }
}

fn spawn_test_target(mut commands: Commands) {
    commands.spawn((
        TransformBundle {
            local: Transform::from_xyz(-2.0, 1.0, 0.0),
            ..default()
        },
        TestTarget,
    ));
}

fn move_test_target_from_input(
    mut test_target: Query<&mut Transform, With<TestTarget>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = test_target.get_single_mut() {
        let move_input = get_wasd_input_as_vector(&input);

        transform.translation += move_input * time.delta_seconds() * TARGET_MOVE_SPEED;
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

fn draw_test_target_gizmo(mut gizmos: Gizmos, target: Query<&Transform, With<TestTarget>>) {
    if let Ok(transform) = target.get_single() {
        let position = transform.translation;

        // Inner
        gizmos.circle(position, Vec3::Y, 0.1, TARGET_COLOR);

        // Outer
        gizmos.circle(position, Vec3::Y, TARGET_RADIUS, TARGET_COLOR);
    }
}
