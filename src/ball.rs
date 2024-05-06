use bevy::{
    asset::Assets,
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Commands, ResMut},
    },
    math::{primitives::Circle, Vec2},
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    utils::default,
};

use crate::components::{Shape, Position, Velocity};

const BALL_SIZE: f32 = 15.;
pub const BALL_VELOCITY: f32 = 6.;
const BALL_START_POSITION_X: f32 = 0.;
const BALL_START_POSITION_Y: f32 = 0.;


#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    position: Position,
    velocity: Velocity,
    shape: Shape,
}

impl BallBundle {
    pub fn new() -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::new(BALL_START_POSITION_X, BALL_START_POSITION_Y)),
            velocity: Velocity(Vec2::new(BALL_VELOCITY, BALL_VELOCITY)),
            shape: Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
        }
    }
}



pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball");

    let ball = Mesh::from(Circle::new(BALL_SIZE));
    let ball_color = ColorMaterial::from(Color::rgb(245., 255., 0.));

    let mesh_handle = meshes.add(ball);
    let material_handle = materials.add(ball_color);

    commands.spawn((
        BallBundle::new(),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}
