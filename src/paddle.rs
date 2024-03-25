use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Commands, Query, ResMut},
    },
    prelude::{Bundle, Color, Mesh, Rectangle, Vec2},
    sprite::ColorMaterial,
    sprite::MaterialMesh2dBundle,
    utils::default,
};

use crate::position::{Position, FIELD_BOUNDARIES_LEFT, FIELD_BOUNDARIES_RIGHT};
use crate::{ball::Ball, position::VELOCITY};

const PADDLE_POSITION_Y: f32 = 0.;
const PADDLE_POSITION_X_LEFT: f32 = FIELD_BOUNDARIES_LEFT + 50.;
const PADDLE_POSITION_X_RIGHT: f32 = FIELD_BOUNDARIES_RIGHT - 50.;
pub const PADDLE_WIDTH: f32 = 20.0;
pub const PADDLE_HEIGHT: f32 = 50.;

#[derive(Debug, PartialEq)]
pub enum PaddleLocation {
    Left,
    Right,
}

#[derive(Component, Debug)]
pub struct Paddle {
    location: PaddleLocation,
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
}

impl PaddleBundle {
    pub fn new(location: Paddle) -> Self {
        Self {
            position: location.position(),
            location,
        }
    }
}

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let wall = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
    let wall_color = ColorMaterial::from(Color::rgb(128., 128., 128.));

    let mesh_handle = meshes.add(wall);
    let material_handle = materials.add(wall_color);

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
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}

pub fn move_right_paddle(
    ball: Query<&Position, With<Ball>>,
    mut paddles: Query<(&mut Position, &Paddle), Without<Ball>>,
) {
    for (mut paddle_pos, paddle) in &mut paddles {
        if paddle.location == PaddleLocation::Right {
            if let Ok(position) = ball.get_single() {
                if position.0.x > 0. {
                    if position.0.y < paddle_pos.0.y {
                        paddle_pos.0.y -= VELOCITY;
                    } else {
                        paddle_pos.0.y += VELOCITY;
                    }
                }
            }
        }
    }
}
