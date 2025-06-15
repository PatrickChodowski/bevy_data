use bevy::prelude::*;
use bevy::core_pipeline::{bloom::Bloom, tonemapping::Tonemapping};

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
        Camera3d::default(),
        Tonemapping::Reinhard,
        Projection::Orthographic(OrthographicProjection::default_3d()),
        Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
        Bloom::default(),
    ));
}

