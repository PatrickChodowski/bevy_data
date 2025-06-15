use bevy::prelude::*;

mod assets;
mod camera;
mod light;
mod schedule;
mod viz;

use assets::AssetsPlugin;
use camera::CameraPlugin;
use light::LightPlugin;
use schedule::SchedulePlugin;
use viz::VizPlugin;

fn main() -> AppExit {
    App::new()
    .add_plugins(
        (
            DefaultPlugins,
            AssetsPlugin,
            CameraPlugin,
            LightPlugin,
            SchedulePlugin,
            VizPlugin
        )
    )
    .run()
}
