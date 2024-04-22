use bevy::asset::Assets;
use bevy::ecs::system::{Commands, ResMut, Query};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::default;
use bevy::{
    ecs::component::Component,
    prelude::{Bundle, Color, Mesh, Rectangle, Vec2},
    sprite::ColorMaterial,
    window::Window,
};

use crate::position::{Position, Shape, FIELD_BOUNDARIES_RIGHT, FIELD_BOUNDARIES_BOTTOM, FIELD_BOUNDARIES_TOP};

const TOP_WALL_POSITION_Y: f32 = FIELD_BOUNDARIES_TOP;
const BOTTOM_WALL_POSITION_Y: f32 = FIELD_BOUNDARIES_BOTTOM;
const WALL_POSITION_X: f32 = 0.;

const WALL_WIDTH: f32 = FIELD_BOUNDARIES_RIGHT * 2.0;
pub const WALL_HEIGHT: f32 = 20.;

#[derive(Component)]
struct Wall;

#[derive(Bundle)]
pub struct WallBundle {
    wall: Wall,
    shape: Shape,
    position: Position,
}

impl WallBundle {
    pub fn new(x:f32, y:f32,width:f32) -> Self {
        Self {
            wall:Wall,
            shape: Shape(Vec2::new(width, WALL_HEIGHT)),
            position: Position(Vec2::new(x,y)),
        }
    }
}

pub fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single(){
        let window_width = window.resolution.width();
        let window_height = window.resolution.height();

        let wall_x = 0.;
        let top_wall_y = window_height / 2. - WALL_HEIGHT / 2.;
        let bottom_wall_y = WALL_HEIGHT / 2. - window_height / 2.;

        let top_wall = WallBundle::new(wall_x,top_wall_y, window_width);
        let bottom_wall = WallBundle::new(wall_x,bottom_wall_y, window_width);

        let wall = Mesh::from(Rectangle::from_size(top_wall.shape.0));
        let wall_color = ColorMaterial::from(Color::rgb(0., 0., 0.));

        let mesh_handle = meshes.add(wall);
        let material_handle = materials.add(wall_color);
    

    commands.spawn((
        top_wall,
        MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: material_handle.clone(),
            ..default()
        },
    ));
    commands.spawn((
        bottom_wall,
        MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: material_handle.clone(),
            ..default()
        },
    ));
}
}
