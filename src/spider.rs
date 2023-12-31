use bevy::{math::vec3, prelude::*};

use crate::{
    ik::{leg::AnimatedLeg, IkChain},
    rotations,
};

const SPAWN_POSITION: Vec3 = Vec3::new(-2.0, 1.0, 2.0);
const MOVE_SPEED: f32 = 6.0;

const LEG_TARGET_OFFSET: Vec3 = Vec3::new(4.0, -0.5, 0.0);
const LEG_ERROR_THRESHOLD: f32 = 12.0;

const BODY_COLOR: Color = Color::BLACK;
const LEGS_COLOR: Color = Color::DARK_GRAY;

pub struct SpiderPlugin;

impl Plugin for SpiderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spider).add_systems(
            Update,
            (
                move_from_input,
                update_leg_error,
                retarget_if_threshold_reached,
                position_leg_pieces_on_chain,
            ),
        );
    }
}

#[derive(Component)]
pub struct Spider {
    combined_leg_position_error: f32,
    movement_group: u8,
}

impl Spider {
    fn switch_movement_group(&mut self) {
        self.movement_group = match self.movement_group {
            1 => 2,
            _ => 1,
        };
    }
}

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

#[derive(Component)]
struct LegPiece {
    /// what chain segment this leg piece belongs to
    index_in_chain: u8,
}

impl LegPiece {
    fn new(position_in_chain: u8) -> Self {
        Self {
            index_in_chain: position_in_chain,
        }
    }
}

fn spawn_spider(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(shape::Box::new(1.4, 0.8, 1.8).into());

    let material = materials.add(StandardMaterial {
        base_color: BODY_COLOR,
        perceptual_roughness: 1.0,
        ..default()
    });

    commands
        .spawn((
            Spider {
                combined_leg_position_error: 0.0,
                movement_group: 2,
            },
            PbrBundle {
                transform: Transform::from_translation(SPAWN_POSITION),
                mesh,
                material,
                ..default()
            },
        ))
        .with_children(|spider| spawn_spider_legs(spider, &mut meshes, &mut materials));
}

fn spawn_spider_legs(
    spider: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(shape::Box::new(0.2, 0.2, 3.2).into());

    let material = materials.add(StandardMaterial {
        base_color: LEGS_COLOR,
        perceptual_roughness: 1.0,
        ..default()
    });

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

        spider
            .spawn((
                IkChain::new(points_of_current_leg),
                AnimatedLeg::new(rotation * LEG_TARGET_OFFSET, target),
                SpiderLeg {
                    movement_group: data.movement_group,
                },
                TransformBundle::default(),
                VisibilityBundle::default(),
            ))
            .with_children(|chain| {
                // Upper leg
                chain.spawn((
                    PbrBundle {
                        // Position is wrong but it gets fixed in the first update
                        transform: Transform::from_translation(SPAWN_POSITION),
                        mesh: mesh.clone(),
                        material: material.clone(),
                        ..default()
                    },
                    LegPiece::new(0),
                ));

                // Lower leg
                chain.spawn((
                    PbrBundle {
                        // Position is wrong but it gets fixed in the first update
                        transform: Transform::from_translation(SPAWN_POSITION),
                        mesh: mesh.clone(),
                        material: material.clone(),
                        ..default()
                    },
                    LegPiece::new(1),
                ));
            });
    }
}

fn move_from_input(
    mut spider: Query<(&mut Transform, &Children), With<Spider>>,
    mut spider_legs: Query<&mut IkChain, With<SpiderLeg>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, children) = spider.single_mut();

    let move_input = get_wasd_input_as_vector(&input);
    let delta_position = move_input * time.delta_seconds() * MOVE_SPEED;

    transform.translation += delta_position;

    for &child_id in children.iter() {
        if let Ok(mut leg) = spider_legs.get_mut(child_id) {
            leg.move_start(delta_position);
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

fn update_leg_error(
    mut spider: Query<(&mut Spider, &Children)>,
    spider_legs: Query<(&IkChain, &AnimatedLeg), With<SpiderLeg>>,
) {
    let (mut spider, children) = spider.single_mut();

    let combined_error = children
        .iter()
        .filter_map(|&child| spider_legs.get(child).ok())
        .fold(0.0, |combined, (chain, leg)| {
            combined + (chain.start + leg.reposition_target_offset).distance(leg.current_target)
        });

    spider.combined_leg_position_error = combined_error;
}

fn retarget_if_threshold_reached(
    mut spider: Query<(&mut Spider, &Children)>,
    mut spider_legs: Query<(&IkChain, &mut AnimatedLeg, &SpiderLeg)>,
) {
    let (mut spider, children) = spider.single_mut();

    if spider.combined_leg_position_error > LEG_ERROR_THRESHOLD {
        spider.switch_movement_group();

        // get all legs of current movement group
        // set their current target to new position
        for &child_id in children.iter() {
            if let Ok((chain, mut leg, spider_leg)) = spider_legs.get_mut(child_id) {
                if spider_leg.movement_group == spider.movement_group {
                    let target = chain.start + leg.reposition_target_offset;
                    leg.set_new_target(target);
                }
            }
        }
    }
}

/// updates the position of the leg piece objects on the chain they belong to
fn position_leg_pieces_on_chain(
    spider_legs: Query<(&IkChain, &GlobalTransform, &Children), With<SpiderLeg>>,
    mut leg_pieces: Query<(&LegPiece, &mut Transform)>,
) {
    for (chain, global_transform, children) in spider_legs.iter() {
        for &child_id in children.iter() {
            if let Ok((leg, mut transform)) = leg_pieces.get_mut(child_id) {
                let segment = chain.get_segment(leg.index_in_chain as usize);

                let segment_direction = (segment.end - segment.start).normalize_or_zero();
                let segment_orientation = rotations::looking_towards(segment_direction, Vec3::Y);
                let segment_middle = segment.start + segment_direction * segment.length / 2.0;

                let local_position = segment_middle - global_transform.translation();

                transform.translation = local_position;
                transform.rotation = segment_orientation;
            }
        }
    }
}
