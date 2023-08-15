mod leg;

use bevy::prelude::*;

use leg::IkLegPlugin;

const POINT_RADIUS: f32 = 0.5;
const POINT_COLOR: Color = Color::PURPLE;
const SEGMENT_COLOR: Color = Color::BLUE;

pub struct IkPlugin;

impl Plugin for IkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(IkLegPlugin)
            .add_systems(Update, draw_ik_chain_gizmos);
    }
}

#[derive(Component)]
struct IkChain {
    start: Vec3,
    points: Vec<Vec3>,
    lengths: Vec<f32>,
}

impl IkChain {
    fn new(points: Vec<Vec3>) -> Self {
        if points.len() < 2 {
            panic!(
                "Invalid vector! IK chain can't be made from {} points",
                points.len()
            );
        }

        let lengths = calculate_distances_between_points(&points);

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

fn solve_chain_towards_target(chain: &mut IkChain, target: Vec3, iterations: i32) {
    for _ in 0..iterations {
        backward_fabrik_pass(chain, target);
        forward_fabrik_pass(chain);
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

fn calculate_distances_between_points(points: &Vec<Vec3>) -> Vec<f32> {
    let mut distances: Vec<f32> = Vec::new();

    for index in 0..points.len() - 1 {
        let start = points[index];
        let end = points[index + 1];
        distances.push(start.distance(end));
    }

    distances
}

// Gizmos

fn draw_ik_chain_gizmos(mut gizmos: Gizmos, ik_chains: Query<&IkChain>) {
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
