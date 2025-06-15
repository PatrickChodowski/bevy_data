use bevy::prelude::*;
use polars::prelude::*;

use crate::assets::{ActiveViz, Data};
use crate::calcs::group_by_sum;
use crate::schedule::GameState;


pub struct VizPlugin;


impl Plugin for VizPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, update.run_if(in_state(GameState::AssetsReady)))
        ;
    }
}

fn update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut datas:    Query<(Entity, &Data, &mut ActiveViz)>,
){
    for (entity, data, mut active_viz) in datas.iter_mut(){
        let agg_df: DataFrame = group_by_sum(&data.df, "Pclass", "Survived");
        info!("{:?}", agg_df);
        bar_chart(&mut commands, &mut meshes, &mut materials, &agg_df);
        commands.entity(entity).remove::<ActiveViz>();
    }
}



#[derive(Component)]
pub struct GraphBase;

#[derive(Component)]
pub struct GraphPart;

#[derive(Component)]
pub struct BarChart;


fn graph_base(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    return (
        Mesh2d(meshes.add(Rectangle::new(1000.0, 1000.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GraphPart,
        GraphBase
    );
}

#[derive(Debug)]
struct BarData {
    width: f32,
    height: f32,
    loc: Vec2
}
impl BarData {
    fn spawn(
        &self,     
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> impl Bundle {
        return (
            Mesh2d(meshes.add(Rectangle::new(self.width, self.height))),
            MeshMaterial2d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(self.loc.x, self.loc.y, 1.0),
            GraphPart,
            BarChart
        );
    }
}


fn bar_chart(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    df: &DataFrame
){

    commands.spawn(graph_base(meshes, materials));



    let origin_x: f32 = 0.0;
    let origin_y: f32 = 0.0;
    let min_y: f32 = 0.0;
    let max_y: f32 = 150.0;
    let spacing: f32 = 20.0;
    let bar_width: f32 = 50.0;
    let y_range = max_y - min_y;

    let x_values = df.column("Pclass").ok().unwrap().i64().ok().unwrap();
    let y_values = df.column("Survived").ok().unwrap().i64().ok().unwrap();
    let count = x_values.len();

    let total_width = count as f32 * bar_width + (count - 1) as f32 * spacing;
    let half_width = total_width / 2.0;

    let mut bar_datas: Vec<BarData> = Vec::with_capacity(count);


    for i in 0..count {
        let height = y_values.get(i).unwrap_or(0) as f32;
        let center_x = i as f32 * (bar_width + spacing) + bar_width / 2.0 - half_width;
        let center_y = (min_y + height) / 2.0;
        // let y_scale = height/y_range;

        bar_datas.push(BarData{
            width: bar_width,
            loc: Vec2::new(center_x, center_y),
            height: height
        });
    }

    info!("{:?}", bar_datas);

    for bd in bar_datas.iter(){
        commands.spawn(bd.spawn(meshes, materials));
    }



    // for i in 0..df.height(){
    //     let Ok(row) = df.get_row(i) else {return;};

    //     match row.0[1] {
    //         AnyValue::Int64(v) => {

    //         }
    //         _ => {}
    //     }

}
