mod camera;
mod world;

use camera::CameraPlugin;
use world::WorldPlugin;

use bevy::{prelude::*, window};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, WorldPlugin))
        .add_systems(Startup, window::close_on_esc)
        .run();
}
