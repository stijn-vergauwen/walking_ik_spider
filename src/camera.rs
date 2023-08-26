use bevy::prelude::*;

use crate::spider::Spider;

const FOLLOW_DISTANCE: f32 = 10.0;
const SPAWN_POSITION: Vec3 = Vec3::new(0.0, 6.0, 10.0);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, move_towards_spider);
    }
}

#[derive(Component)]
struct SpiderCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(SPAWN_POSITION).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        SpiderCamera,
    ));
}

fn move_towards_spider(
    mut spider_camera: Query<&mut Transform, With<SpiderCamera>>,
    spider: Query<&Transform, (With<Spider>, Without<SpiderCamera>)>,
) {
    let mut camera = spider_camera.single_mut();
    let spider = spider.single();

    let delta_position = get_flat_delta_position(spider.translation, camera.translation);
    let direction = delta_position.normalize_or_zero();

    let mut new_position = spider.translation + direction * FOLLOW_DISTANCE;
    new_position.y = camera.translation.y;

    camera.translation = new_position;
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
