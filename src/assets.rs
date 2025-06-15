use bevy::prelude::*;
use polars::prelude::*;

use crate::schedule::GameState;

pub struct AssetsPlugin;


impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Init), init)
        ;
  }
}

fn init(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>
){

    let df: DataFrame = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("assets/data/titanic.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    info!("df: {:?}", df);
    info!("{:?}", df.get_column_names());

    commands.spawn((
        Data::new(df), 
        ActiveViz{col_index: 1, current_row: 0}
    ));

    next_state.set(GameState::AssetsReady);

}

#[derive(Component)]
pub struct Data {
    pub df: DataFrame 
}
impl Data {
    fn new(df: DataFrame) -> Self {
        Data{df}
    }
}


#[derive(Component)]
pub struct ActiveViz {
    pub col_index: usize,
    pub current_row: usize
}