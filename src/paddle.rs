use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        system::{Commands, ResMut},
    },
    prelude::{Bundle, Color, Mesh, Rectangle, Vec2},
    sprite::ColorMaterial,
    sprite::MaterialMesh2dBundle,
    utils::default,
};

use crate::position::{FIELD_BOUNDARIES_LEFT, FIELD_BOUNDARIES_RIGHT};
use crate::components::{Position, Velocity, Shape};

const PADDLE_POSITION_Y: f32 = 0.;
const PADDLE_POSITION_X_LEFT: f32 = FIELD_BOUNDARIES_LEFT + 50.;
const PADDLE_POSITION_X_RIGHT: f32 = FIELD_BOUNDARIES_RIGHT - 50.;
pub const PADDLE_WIDTH: f32 = 20.;
pub const PADDLE_HEIGHT: f32 = 100.;
pub const PADDLE_VELOCITY: f32 = 4.;

#[derive(Debug, PartialEq)]
pub enum PaddleLocation {
    Left,
    Right,
}

#[derive(Component, Debug)]
pub struct Paddle {
    pub location: PaddleLocation,
}

impl Paddle {
    fn position(&self) -> Position {
        match self.location {
            PaddleLocation::Left => Position(Vec2::new(PADDLE_POSITION_X_LEFT, PADDLE_POSITION_Y)),
            PaddleLocation::Right => {
                Position(Vec2::new(PADDLE_POSITION_X_RIGHT, PADDLE_POSITION_Y))
            }
        }
    }
}

#[derive(Bundle)]
pub struct PaddleBundle {
    location: Paddle,
    position: Position,
    velocity: Velocity,
    shape: Shape,
}

impl PaddleBundle {
    pub fn new(location: Paddle) -> Self {
        Self {
            position: location.position(),
            location,
            velocity: Velocity(Vec2::new(0.,0.)),
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT))
        }
    }
}

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let paddle = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
    let paddle_color = ColorMaterial::from(Color::rgb(255., 0., 132.));

    let mesh_handle = meshes.add(paddle);
    let material_handle = materials.add(paddle_color);

    commands.spawn((
        PaddleBundle::new(Paddle {
            location: PaddleLocation::Left,
        }),
        MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: material_handle.clone(),
            ..default()
        },
    ));
    commands.spawn((
        PaddleBundle::new(Paddle {
            location: PaddleLocation::Right,
        }),
        MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: material_handle.clone(),
            ..default()
        },
    ));
}
