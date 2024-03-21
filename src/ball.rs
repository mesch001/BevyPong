use crate::position::{Position, Velocity};
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

const BALL_SIZE: f32 = 10.;

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    position: Position,
    velocity: Velocity,
}

impl BallBundle {
    pub fn new() -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::new(0., 0.)),
            velocity: Velocity(Vec2::new(5., 5.)),
        }
    }
}

pub fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0.x += velocity.0.x;
        position.0.y += velocity.0.y;
    }
}

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball");

    let ball = Mesh::from(Circle::new(BALL_SIZE));
    let ball_color = ColorMaterial::from(Color::rgb(0., 0., 1.));

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