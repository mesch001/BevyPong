use bevy::asset::Assets;
use bevy::ecs::system::{Commands, ResMut};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::default;
use bevy::{
    ecs::component::Component,
    prelude::{Bundle, Color, Mesh, Rectangle, Vec2},
    sprite::ColorMaterial,
};

use crate::position::{Position, FIELD_BOUNDARIES_RIGHT};
use crate::position::{FIELD_BOUNDARIES_BOTTOM, FIELD_BOUNDARIES_TOP};

const TOP_WALL_POSITION_Y: f32 = FIELD_BOUNDARIES_TOP;
const BOTTOM_WALL_POSITION_Y: f32 = FIELD_BOUNDARIES_BOTTOM;
const WALL_POSITION_X: f32 = 0.;

const WALL_WIDTH: f32 = FIELD_BOUNDARIES_RIGHT * 2.0;
pub const WALL_HEIGHT: f32 = 20.;

#[derive(Component)]
pub enum Wall {
    Top,
    Bottom,
}

impl Wall {
    fn position(&self) -> Position {
        match self {
            Wall::Top => Position(Vec2::new(WALL_POSITION_X, TOP_WALL_POSITION_Y)),
            Wall::Bottom => Position(Vec2::new(WALL_POSITION_X, BOTTOM_WALL_POSITION_Y)),
        }
    }
}

#[derive(Bundle)]
pub struct WallBundle {
    location: Wall,
    position: Position,
}

impl WallBundle {
    pub fn new(location: Wall) -> Self {
        Self {
            position: location.position(),
            location,
        }
    }
}

pub fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let wall = Mesh::from(Rectangle::new(WALL_WIDTH, WALL_HEIGHT));
    let wall_color = ColorMaterial::from(Color::srgb(0., 0., 0.));

    let mesh_handle = meshes.add(wall);
    let material_handle = materials.add(wall_color);

    commands.spawn((
        WallBundle::new(Wall::Top),
        MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: material_handle.clone(),
            ..default()
        },
    ));
    commands.spawn((
        WallBundle::new(Wall::Bottom),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}
