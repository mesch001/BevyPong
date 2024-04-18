use bevy::{
    ecs::{
        query::{With, Without},
        system::{Query, Res},
    },
    prelude::{ButtonInput, KeyCode},
};

use crate::position::Position;
use crate::{ball::Ball, position::VELOCITY};
use crate::{
    paddle::{Paddle, PaddleLocation, PADDLE_HEIGHT},
    position::{FIELD_BOUNDARIES_BOTTOM, FIELD_BOUNDARIES_TOP},
    wall::WALL_HEIGHT,
};

pub fn move_right_paddle(
    ball: Query<&Position, With<Ball>>,
    mut paddles: Query<(&mut Position, &Paddle), Without<Ball>>,
) {
    for (mut paddle_pos, paddle) in &mut paddles {
        if paddle.location == PaddleLocation::Right {
            if let Ok(position) = ball.get_single() {
                if position.0.x > 0. && position.0.x < paddle_pos.0.x{
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

pub fn move_player_paddle(
    input: Res<ButtonInput<KeyCode>>,
    mut pos: Query<(&mut Position, &Paddle), With<Paddle>>,
) {
    for (mut paddle_pos, paddle) in &mut pos {
        if paddle.location == PaddleLocation::Left {
            if input.pressed(KeyCode::ArrowUp)
                && paddle_pos.0.y + PADDLE_HEIGHT / 2.0 + WALL_HEIGHT / 2.0 < FIELD_BOUNDARIES_TOP
            {
                paddle_pos.0.y += VELOCITY;
            }
            if input.pressed(KeyCode::ArrowDown)
                && paddle_pos.0.y - PADDLE_HEIGHT / 2.0 - WALL_HEIGHT / 2.0 > FIELD_BOUNDARIES_BOTTOM
            {
                paddle_pos.0.y -= VELOCITY;
            }
        }
    }
}
