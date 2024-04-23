use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        system::{Commands, ResMut, Query},
    },
    prelude::{Bundle, Color, Mesh, Rectangle, Vec2},
    sprite::ColorMaterial,
    sprite::MaterialMesh2dBundle,
    utils::default,
    window::Window,
};

use crate::position::{Position, Shape, Velocity};

pub const PADDLE_WIDTH: f32 = 10.;
pub const PADDLE_HEIGHT: f32 = 50.;

#[derive(Component, Debug)]
pub struct Paddle ;

#[derive(Bundle)]
pub struct PaddleBundle {
    paddle: Paddle,
    shape: Shape,
    position: Position,
    velocity: Velocity,
}

#[derive(Component)]
pub struct Player ;

#[derive(Component)]
pub struct Ai ;

impl PaddleBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            position: Position(Vec2::new(x, y)),
            velocity: Velocity(Vec2::new(0.,0.)),
        }
    }
}

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    println!("Spawning paddles...");
    if let Ok(window) = window.get_single(){
        let window_width = window.resolution.width();
        let padding = 50.;
        let right_paddle_x = window_width / 2. - padding;
        let left_paddle_x = padding - window_width / 2.;

        let paddle_mesh = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
        let paddle_color = ColorMaterial::from(Color::rgb(255., 0., 132.));

        let mesh_handle = meshes.add(paddle_mesh);
        let material_handle = materials.add(paddle_color);
    

    commands.spawn((
        Player,
        PaddleBundle::new(left_paddle_x, 0.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: material_handle.clone(),
            ..default()
        },
    ));
    commands.spawn((
        Ai,
        PaddleBundle::new(right_paddle_x, 0.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: material_handle.clone(),
            ..default()
        },
    
    ));
    }
}
