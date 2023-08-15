use bevy::prelude::*;

use super::IkChain;

const TARGET_RADIUS: f32 = 0.7;
const TARGET_COLOR: Color = Color::LIME_GREEN;

pub struct IkLegPlugin;

impl Plugin for IkLegPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_test_leg, spawn_test_target))
            .add_systems(Update, draw_test_target_gizmo);
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

fn spawn_test_target(mut commands: Commands) {
    commands.spawn((
        TransformBundle {
            local: Transform::from_xyz(-2.0, 1.0, 0.0),
            ..default()
        },
        TestTarget,
    ));
}

fn draw_test_target_gizmo(mut gizmos: Gizmos, target: Query<&Transform, With<TestTarget>>) {
    if let Ok(transform) = target.get_single() {
        let position = transform.translation;

        // Inner
        gizmos.circle(position, Vec3::Y, 0.1, TARGET_COLOR);

        // Outer
        gizmos.circle(position, Vec3::Y, TARGET_RADIUS, TARGET_COLOR);
    }
}
