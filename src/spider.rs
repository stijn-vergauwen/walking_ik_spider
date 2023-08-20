use std::f32::consts::PI;

use bevy::{math::vec3, prelude::*};

use crate::ik::{leg::BasicLeg, IkChain};

const SPAWN_POSITION: Vec3 = Vec3::new(-2.0, 1.0, 2.0);
const MOVE_SPEED: f32 = 4.0;

const LEG_TARGET_OFFSET: Vec3 = Vec3::new(4.0, -0.5, 0.0);

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

#[derive(Component)]
struct SpiderLeg {
    movement_group: u8,
}

struct LegSpawnInfo {
    position_offset: Vec3,
    angle_offset: f32,
    movement_group: u8,
}

impl LegSpawnInfo {
    fn new(pos: Vec3, angle: f32, group: u8) -> Self {
        LegSpawnInfo {
            position_offset: pos,
            angle_offset: angle,
            movement_group: group,
        }
    }
}

fn spawn_spider(mut commands: Commands) {
    commands
        .spawn((
            Spider,
            TransformBundle {
                local: Transform::from_translation(SPAWN_POSITION),
                ..default()
            },
        ))
        .with_children(|spider| spawn_spider_legs(spider));
}

fn spawn_spider_legs(spider: &mut ChildBuilder) {
    let base_points = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 3.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
    ];

    let legs_data = [
        LegSpawnInfo::new(vec3(0.5, 0.0, -0.8), 40.0, 1),
        LegSpawnInfo::new(vec3(0.5, 0.0, -0.4), 10.0, 2),
        LegSpawnInfo::new(vec3(0.5, 0.0, 0.4), -10.0, 1),
        LegSpawnInfo::new(vec3(0.5, 0.0, 0.8), -40.0, 2),
        LegSpawnInfo::new(vec3(-0.5, 0.0, -0.8), 140.0, 2),
        LegSpawnInfo::new(vec3(-0.5, 0.0, -0.4), 170.0, 1),
        LegSpawnInfo::new(vec3(-0.5, 0.0, 0.4), 190.0, 2),
        LegSpawnInfo::new(vec3(-0.5, 0.0, 0.8), 220.0, 1),
    ];

    for data in legs_data.iter() {
        let rotation = Quat::from_axis_angle(Vec3::Y, data.angle_offset.to_radians());
        let points_of_current_leg = base_points
            .iter()
            .map(|point| SPAWN_POSITION + data.position_offset + (rotation * *point))
            .collect();

        let start = base_points[0];
        let target = start + (rotation * LEG_TARGET_OFFSET);

        spider.spawn((
            IkChain::new(points_of_current_leg),
            BasicLeg::new(rotation * LEG_TARGET_OFFSET, target),
            SpiderLeg {
                movement_group: data.movement_group,
            },
        ));
    }
}

fn move_from_input(
    mut spider: Query<(&mut Transform, &Children), With<Spider>>,
    mut spider_legs: Query<&mut IkChain, With<SpiderLeg>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, children)) = spider.get_single_mut() {
        let move_input = get_wasd_input_as_vector(&input);
        let delta_position = move_input * time.delta_seconds() * MOVE_SPEED;

        transform.translation += delta_position;

        for &child_id in children.iter() {
            if let Ok(mut leg) = spider_legs.get_mut(child_id) {
                leg.move_start(delta_position);
            }
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
