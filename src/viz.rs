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
    mut commands: Commands
){
    info!("Assets Ready");

    

}