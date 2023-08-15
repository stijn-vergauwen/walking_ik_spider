mod camera;
mod ik;
mod world;

use bevy::{prelude::*, window};
use camera::CameraPlugin;
use ik::IkPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, WorldPlugin, IkPlugin))
        .add_systems(Update, window::close_on_esc)
        .run();
}
