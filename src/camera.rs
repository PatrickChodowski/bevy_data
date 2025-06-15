use bevy::prelude::*;
use bevy::core_pipeline::{bloom::Bloom, tonemapping::Tonemapping};
use bevy::render::camera::ScalingMode;

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
        Projection::Orthographic(OrthographicProjection{
            scale: 0.035,
            near: 0.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: ScalingMode::WindowSize,
            area: Rect::new(-1.0, -1.0, 1.0, 1.0),
        }),
        Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
        Bloom::default(),
    ));
}

