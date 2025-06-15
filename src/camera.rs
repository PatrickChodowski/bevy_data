use bevy::prelude::*;

use crate::schedule::GameState;

pub struct CameraPlugin;


impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Init), init)
        ;
  }
}

#[derive(Component)]
pub struct MainCamera;

fn init(
    mut commands: Commands, 
){
    commands.spawn((
        Camera2d::default(),
        MainCamera
    ));
}

