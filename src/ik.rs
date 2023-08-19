pub mod leg;

use bevy::prelude::*;

use leg::IkLegPlugin;

const POINT_RADIUS: f32 = 0.5;
const POINT_COLOR: Color = Color::PURPLE;
const SEGMENT_COLOR: Color = Color::CYAN;

const DRAW_CHAIN_GIZMOS: bool = true;
const DRAW_ORIENTATION_GIZMOS: bool = false;

pub struct IkPlugin;

impl Plugin for IkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(IkLegPlugin)
            .add_systems(Update, draw_ik_chain_gizmos);
    }
}

#[derive(Component)]
pub struct IkChain {
    start: Vec3,
    points: Vec<Vec3>,
    lengths: Vec<f32>,
}

impl IkChain {
    pub fn new(points: Vec<Vec3>) -> Self {
        if points.len() < 2 {
            panic!(
                "Invalid vector! IK chain can't be made from {} points",
                points.len()
            );
        }

        let lengths = calculate_chain_lengths(&points);

        IkChain {
            start: points[0],
            points,
            lengths,
        }
    }

    fn get_segment(&self, index: usize) -> ChainSegment {
        if index >= self.points.len() - 1 {
            panic!(
                "Invalid index! get_segment called with index: {}, but only {} points",
                index,
                self.points.len()
            );
        }

        ChainSegment {
            start: self.points[index],
            end: self.points[index + 1],
            length: self.lengths[index],
        }
    }
}

struct ChainSegment {
    start: Vec3,
    end: Vec3,
    length: f32,
}

fn solve_chain_towards_target(
    chain: &mut IkChain,
    target: Vec3,
    iterations: i32,
    gizmos: &mut Gizmos,
) {
    for _ in 0..iterations {
        backward_fabrik_pass(chain, target);
        forward_fabrik_pass(chain);
        constrain_chain_orientation(chain, gizmos);
    }
}

fn forward_fabrik_pass(chain: &mut IkChain) {
    let points_count = chain.points.len();

    chain.points[0] = chain.start;
    for i in 0..points_count - 1 {
        let segment = chain.get_segment(i);
        let direction = (segment.end - segment.start).normalize_or_zero();

        chain.points[i + 1] = segment.start + direction * segment.length;
    }
}

fn backward_fabrik_pass(chain: &mut IkChain, target: Vec3) {
    let points_count = chain.points.len();

    chain.points[points_count - 1] = target;
    for i in (0..points_count - 1).rev() {
        let segment = chain.get_segment(i);
        let direction = (segment.start - segment.end).normalize_or_zero();

        chain.points[i] = segment.end + direction * segment.length;
    }
}

fn constrain_chain_orientation(chain: &mut IkChain, gizmos: &mut Gizmos) {
    // After learning about rotations and asking chat-gpt, here's my plan:
    // 1. use the first and last points to calculate the leg's local orientation (the leg points in the negative z direction)
    // 2. calculate the orientation from the first point to the middle joint
    // 3. get the delta quaternion and convert it to euler angle
    // 4. get the delta components to constrain (y & x)
    // 5. adjust the quaternion towards the middle joint by these delta components so it aligns with the leg's orientation
    // 6. place the middle joint on this new position
    // 7. repeat for each middle joint <- this last part isn't implemented, but also not needed in this case

    let first_point = chain.points[0];
    let last_point = chain.points[chain.points.len() - 1];
    let leg_orientation = rotation_looking_at(first_point, last_point, Vec3::Y);

    let middle_point = chain.points[1];
    let joint_orientation = rotation_looking_at(first_point, middle_point, Vec3::Y);

    let delta_orientation = leg_orientation.inverse() * joint_orientation;
    let delta_euler = delta_orientation.to_euler(EulerRot::XYZ);

    let x_adjustment = if delta_euler.0 < 0.01 {
        -delta_euler.0 + 0.01
    } else {
        0.0
    };

    let orientation_adjustment = Quat::from_euler(EulerRot::XYZ, x_adjustment, -delta_euler.1, 0.0);

    // calculate new direction
    let adjusted_orientation = joint_orientation * orientation_adjustment;
    let segment = chain.get_segment(0);

    // place middle point on new position
    chain.points[1] = first_point + adjusted_orientation * (Vec3::NEG_Z * segment.length);

    // For debugging and visualizing
    if DRAW_ORIENTATION_GIZMOS {
        let gizmo_point = first_point;
        let gizmo_orientation = leg_orientation;
    
        gizmos.ray(gizmo_point, gizmo_orientation * Vec3::X, Color::GREEN);
        gizmos.ray(gizmo_point, gizmo_orientation * Vec3::Y, Color::RED);
        gizmos.ray(gizmo_point, gizmo_orientation * Vec3::Z, Color::BLUE);
    
        let gizmo_point = first_point;
        let gizmo_orientation = joint_orientation;
    
        gizmos.ray(gizmo_point, gizmo_orientation * Vec3::X, Color::GREEN);
        gizmos.ray(gizmo_point, gizmo_orientation * Vec3::Y, Color::RED);
        gizmos.ray(gizmo_point, gizmo_orientation * Vec3::Z, Color::BLUE);
    }
}

fn rotation_looking_at(start: Vec3, target: Vec3, up: Vec3) -> Quat {
    rotation_towards(target - start, up)
}

fn rotation_towards(direction: Vec3, up: Vec3) -> Quat {
    let back = -direction.try_normalize().unwrap_or(Vec3::NEG_Z);
    let up = up.try_normalize().unwrap_or(Vec3::Y);
    let right = up
        .cross(back)
        .try_normalize()
        .unwrap_or_else(|| up.any_orthonormal_vector());
    let up = back.cross(right);
    Quat::from_mat3(&Mat3::from_cols(right, up, back))
}

fn calculate_chain_lengths(points: &Vec<Vec3>) -> Vec<f32> {
    let mut lengths: Vec<f32> = Vec::new();

    for index in 0..points.len() - 1 {
        let start = points[index];
        let end = points[index + 1];
        lengths.push(start.distance(end));
    }

    lengths
}

// Gizmos

fn draw_ik_chain_gizmos(mut gizmos: Gizmos, ik_chains: Query<&IkChain>) {
    if DRAW_CHAIN_GIZMOS {
        for chain in ik_chains.iter() {
            for point in chain.points.iter() {
                gizmos.sphere(*point, Quat::IDENTITY, POINT_RADIUS, POINT_COLOR);
            }
    
            for index in 0..chain.points.len() - 1 {
                let segment = chain.get_segment(index);
                gizmos.line(segment.start, segment.end, SEGMENT_COLOR);
            }
        }
    }
}
