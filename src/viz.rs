use bevy::prelude::*;

use crate::schedule::GameState;

pub struct VizPlugin;


impl Plugin for VizPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::AssetsReady), init)
        ;
  }
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    info!("Assets Ready");

    commands.spawn((
        Mesh3d(meshes.add(Circle::new(10.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));



    commands.spawn(
        (
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.5, 0.0),
            GraphPart,
        )
    );

    

}


#[derive(Component)]
pub struct GraphPart;