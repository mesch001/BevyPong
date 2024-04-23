use crate::position::{Position, Velocity, Shape};
use bevy::{
    asset::Assets,
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Commands, ResMut},
    },
    math::{primitives::Circle, Vec2},
    prelude::{Query, With},
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    utils::default,
};

pub const BALL_SIZE: f32 = 15.;

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    shape: Shape,
    position: Position,
    velocity: Velocity,
}

impl BallBundle {
    pub fn new(x: f32,y:f32) -> Self {
        Self {
            ball: Ball,
            shape: Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
            position: Position(Vec2::new(0., 0.)),
            velocity: Velocity(Vec2::new(x, y)),
        }
    }
}

pub fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0 * 2.;
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
        BallBundle::new(1., 1.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}
