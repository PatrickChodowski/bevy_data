use bevy::prelude::*;

mod assets;
mod camera;
mod schedule;
mod viz;

use assets::AssetsPlugin;
use camera::CameraPlugin;
use schedule::SchedulePlugin;
use viz::VizPlugin;

fn main() -> AppExit {
    App::new()
    .add_plugins(
        (
            DefaultPlugins,
            AssetsPlugin,
            CameraPlugin,
            SchedulePlugin,
            VizPlugin
        )
    )
    .run()
}
