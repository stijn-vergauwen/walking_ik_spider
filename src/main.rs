mod camera;
mod ik;
mod world;
mod spider;

use bevy::{prelude::*, window};
use camera::CameraPlugin;
use ik::IkPlugin;
use spider::SpiderPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, WorldPlugin, IkPlugin, SpiderPlugin))
        .add_systems(Update, window::close_on_esc)
        .run();
}
