use bevy::prelude::*;

use crate::schedule::GameState;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Init), init)
        ;
  }
}

fn init(
    mut commands: Commands
){
    commands.insert_resource(AmbientLight{
        color: Color::WHITE,
        brightness: 1000.0,
        affects_lightmapped_meshes: true,
    });
}