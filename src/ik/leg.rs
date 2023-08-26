use bevy::prelude::*;

use super::{solve_chain_towards_target, IkChain};

const TARGET_RADIUS: f32 = 0.7;
const TARGET_COLOR: Color = Color::ORANGE_RED;
const CURRENT_TARGET_COLOR: Color = Color::LIME_GREEN;

const FABRIK_ITERATIONS: i32 = 6;

const LERP_SPEED: f32 = 6.0;
const CURVE_HEIGHT: f32 = 0.7;

const DRAW_TARGET_GIZMOS: bool = false;

pub struct IkLegPlugin;

impl Plugin for IkLegPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    draw_animated_leg_gizmos,
                    animate_leg_towards_target,
                ),
            );
    }
}

#[derive(Component)]
pub struct AnimatedLeg {
    pub reposition_target_offset: Vec3,
    pub previous_target: Vec3,
    pub current_target: Vec3,
    pub lerp_fraction: f32,
}

impl AnimatedLeg {
    pub fn new(reposition_target_offset: Vec3, position: Vec3) -> Self {
        AnimatedLeg {
            reposition_target_offset,
            previous_target: position,
            current_target: position,
            lerp_fraction: 0.0,
        }
    }

    fn increase_lerp_fraction(&mut self, delta: f32) {
        self.lerp_fraction = (self.lerp_fraction + delta).min(1.0);
    }

    pub fn set_new_target(&mut self, target: Vec3) {
        self.previous_target = self.current_target;
        self.current_target = target;
        self.lerp_fraction = 0.0;
    }
}

fn animate_leg_towards_target(
    mut animated_legs: Query<(&mut IkChain, &mut AnimatedLeg)>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    for (mut chain, mut leg) in animated_legs.iter_mut() {
        leg.increase_lerp_fraction(LERP_SPEED * time.delta_seconds());

        let start = leg.previous_target;
        let end = leg.current_target;
        let distance = start.distance(end);
        let curve_anchor = start.lerp(end, 0.5) + Vec3::Y * distance * CURVE_HEIGHT; // This is the point in the air to lerp upwards

        let start_to_anchor = start.lerp(curve_anchor, leg.lerp_fraction);
        let anchor_to_end = curve_anchor.lerp(end, leg.lerp_fraction);
        let interpolated_target = start_to_anchor.lerp(anchor_to_end, leg.lerp_fraction);

        solve_chain_towards_target(
            &mut chain,
            interpolated_target,
            FABRIK_ITERATIONS,
            &mut gizmos,
        );
    }
}

// Gizmos

fn draw_animated_leg_gizmos(mut gizmos: Gizmos, animated_legs: Query<(&IkChain, &AnimatedLeg)>) {
    if DRAW_TARGET_GIZMOS {
        for (chain, leg) in animated_legs.iter() {
            draw_target(
                &mut gizmos,
                chain.start + leg.reposition_target_offset,
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
}

fn draw_target(gizmos: &mut Gizmos, position: Vec3, radius: f32, color: Color) {
    // Inner
    gizmos.circle(position, Vec3::Y, 0.1, color);

    // Outer
    gizmos.circle(position, Vec3::Y, radius, color);
}
