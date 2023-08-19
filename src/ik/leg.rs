use bevy::prelude::*;

use super::{solve_chain_towards_target, IkChain};

const TARGET_OFFSET: Vec3 = Vec3::new(2.0, -0.5, 0.0);
const NEW_TARGET_DISTANCE_THRESHOLD: f32 = 2.0;

const TARGET_RADIUS: f32 = 0.7;
const TARGET_COLOR: Color = Color::ORANGE_RED;
const CURRENT_TARGET_COLOR: Color = Color::LIME_GREEN;

const FABRIK_ITERATIONS: i32 = 6;

const LEG_BASE_MOVE_SPEED: f32 = 4.0;

pub struct IkLegPlugin;

impl Plugin for IkLegPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_basic_leg).add_systems(
            Update,
            (
                // move_test_target_from_input,
                move_basic_leg_to_target,
                draw_leg_gizmos,
                move_leg_base_from_input,
                set_new_target_if_threshold_reached,
            ),
        );
    }
}

#[derive(Component)]
struct BasicLeg {
    target_offset: Vec3,
    current_target: Vec3,
}

fn spawn_basic_leg(mut commands: Commands) {
    let test_points = vec![
        Vec3::new(-4.0, 1.0, 0.0),
        Vec3::new(-3.0, 3.0, 0.0),
        Vec3::new(-2.0, 1.0, 0.0),
    ];

    let start = test_points[0];
    let target = start + TARGET_OFFSET;

    commands.spawn((
        IkChain::new(test_points),
        BasicLeg {
            target_offset: TARGET_OFFSET,
            current_target: target,
        },
    ));
}

fn move_basic_leg_to_target(mut basic_leg: Query<(&mut IkChain, &BasicLeg)>, mut gizmos: Gizmos) {
    if let Ok((mut chain, leg)) = basic_leg.get_single_mut() {
        let target = leg.current_target;
        solve_chain_towards_target(&mut chain, target, FABRIK_ITERATIONS, &mut gizmos);
    }
}

fn move_leg_base_from_input(
    mut chain: Query<&mut IkChain, With<BasicLeg>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut chain) = chain.get_single_mut() {
        let move_input = get_wasd_input_as_vector(&input);

        chain.start += move_input * time.delta_seconds() * LEG_BASE_MOVE_SPEED;
    }
}

fn set_new_target_if_threshold_reached(mut basic_leg: Query<(&IkChain, &mut BasicLeg)>) {
    if let Ok((chain, mut leg)) = basic_leg.get_single_mut() {
        let target_position = chain.start + leg.target_offset;
        let current_position = leg.current_target;

        if target_position.distance(current_position) > NEW_TARGET_DISTANCE_THRESHOLD {
            leg.current_target = target_position;
        }
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
    if input.pressed(KeyCode::R) {
        result.y += 1.0;
    }
    if input.pressed(KeyCode::F) {
        result.y -= 1.0;
    }

    result.normalize_or_zero()
}

// Gizmos

fn draw_leg_gizmos(mut gizmos: Gizmos, basic_leg: Query<(&IkChain, &BasicLeg)>) {
    if let Ok((chain, leg)) = basic_leg.get_single() {
        draw_target(
            &mut gizmos,
            chain.start + leg.target_offset,
            TARGET_RADIUS,
            TARGET_COLOR,
        );
        draw_target(
            &mut gizmos,
            leg.current_target,
            TARGET_RADIUS,
            CURRENT_TARGET_COLOR,
        );
    }
}

fn draw_target(gizmos: &mut Gizmos, position: Vec3, radius: f32, color: Color) {
    // Inner
    gizmos.circle(position, Vec3::Y, 0.1, color);

    // Outer
    gizmos.circle(position, Vec3::Y, radius, color);
}
