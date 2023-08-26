use bevy::prelude::*;

use crate::{rotations, spider::Spider};

const FOLLOW_DISTANCE: f32 = 10.0;
const SPAWN_POSITION: Vec3 = Vec3::new(0.0, 6.0, 10.0);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            Update,
            (
                (update_target_position, update_target_rotation),
                (move_towards_spider, rotate_towards_spider),
            )
                .chain(),
        );
    }
}

#[derive(Component)]
struct SpiderCamera {
    target_position: Vec3,
    target_rotation: Quat,
}

impl SpiderCamera {
    fn new(position: Vec3, rotation: Quat) -> Self {
        SpiderCamera {
            target_position: position,
            target_rotation: rotation,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    let spawn_rotation = rotations::looking_at(SPAWN_POSITION, Vec3::ZERO, Vec3::Y);

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(SPAWN_POSITION).with_rotation(spawn_rotation),
            ..Default::default()
        },
        SpiderCamera::new(SPAWN_POSITION, spawn_rotation),
    ));
}

fn update_target_position(
    mut spider_camera: Query<(&mut SpiderCamera, &Transform)>,
    spider: Query<&Transform, (With<Spider>, Without<SpiderCamera>)>,
) {
    let (mut camera, camera_transform) = spider_camera.single_mut();
    let spider = spider.single();

    let flat_delta_position =
        get_flat_delta_position(spider.translation, camera_transform.translation);
    let direction = flat_delta_position.normalize_or_zero();

    let mut new_position = spider.translation + direction * FOLLOW_DISTANCE;
    new_position.y = camera_transform.translation.y;

    camera.target_position = new_position;
}

fn update_target_rotation(
    mut spider_camera: Query<(&mut SpiderCamera, &Transform)>,
    spider: Query<&Transform, (With<Spider>, Without<SpiderCamera>)>,
) {
    let (mut camera, camera_transform) = spider_camera.single_mut();
    let spider = spider.single();

    let target_rotation = rotations::looking_at(camera_transform.translation, spider.translation, Vec3::Y);
    camera.target_rotation = target_rotation;
}

fn move_towards_spider(mut spider_camera: Query<(&SpiderCamera, &mut Transform)>) {
    let (camera, mut camera_transform) = spider_camera.single_mut();

    camera_transform.translation = camera.target_position;
}

fn rotate_towards_spider(
    mut spider_camera: Query<(&mut Transform, &SpiderCamera)>,
) {
    let (mut camera_transform, camera) = spider_camera.single_mut();
    
    camera_transform.rotation = camera.target_rotation;
}

fn get_flat_delta_position(from: Vec3, to: Vec3) -> Vec3 {
    flatten_vector(to) - flatten_vector(from)
}

/// sets the y value to 0 and returns the vector
fn flatten_vector(vector: Vec3) -> Vec3 {
    Vec3 {
        x: vector.x,
        y: 0.0,
        z: vector.z,
    }
}
