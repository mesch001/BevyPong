use bevy::{
    window::Window,
    ecs::{
        query::With,
        system::{Query, Res},
    },
    prelude::{ButtonInput, KeyCode},
};

use crate::{position::{Position, Velocity}};
use crate::{
    paddle::{Paddle, PADDLE_HEIGHT, LeftPaddleBundle, RightPaddleBundle, PlayerType, Player1, Player2, Ai},
    wall::WALL_HEIGHT,
};

const PADDLE_SPEED: f32 = 5.;

pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_player1: Query<(&mut Velocity, &PlayerType), With<LeftPaddleBundle>>,
    mut paddle_player2: Query<(&mut Velocity, &PlayerType), With<RightPaddleBundle>>,
) {
    
    if let Ok((mut velocity, player_type)) = paddle_player1.get_single_mut() {
        if player_type == Player1{
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                velocity.0.y = 1.;
            } else if keyboard_input.pressed(KeyCode::ArrowDown) {
                velocity.0.y = -1.;
            } else {
                velocity.0.y = 0.;
            }
        }
    } 
        
    if let Ok((mut velocity, player_type)) = paddle_player1.get_single_mut() {
        if player_type == Player2{
            if keyboard_input.pressed(KeyCode::KeyW) {
                velocity.0.y = 1.;
            } else if keyboard_input.pressed(KeyCode::KeyS) {
                velocity.0.y = -1.;
            } else {
                velocity.0.y = 0.;
            }
        } else if player_type == Ai{
            if keyboard_input.pressed(KeyCode::KeyW) {
                velocity.0.y = 1.;
            } else if keyboard_input.pressed(KeyCode::KeyS) {
                velocity.0.y = -1.;
            } else {
                velocity.0.y = 0.;
            }
        }
    } 
}

pub fn move_paddles(
    mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();
        let max_y = window_height / 2. - WALL_HEIGHT - PADDLE_HEIGHT / 2.;

        for (mut position, velocity) in &mut paddle {
            let new_position = position.0 + velocity.0 * PADDLE_SPEED;
            if new_position.y.abs() < max_y {
                position.0 = new_position;
            }
        }
    }
}