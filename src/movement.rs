use bevy::{
    ecs::{
        query::{With, Without},
        system::{Query, Res},
    },
    math::Vec2,
    prelude::{ButtonInput, KeyCode},
};

use crate::components::{Position, Velocity};
use crate::position::{FIELD_BOUNDARIES_LEFT, FIELD_BOUNDARIES_RIGHT, FIELD_BOUNDARIES_BOTTOM, FIELD_BOUNDARIES_TOP};
use crate::ball::{Ball, BALL_VELOCITY};
use crate::paddle::{Paddle, PaddleLocation, PADDLE_HEIGHT, PADDLE_VELOCITY};
use crate::wall::WALL_HEIGHT;

pub fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0;
    }
}

pub fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
) {
    if let Ok((mut position,mut velocity )) = ball.get_single_mut() {
        if position.0.x >= FIELD_BOUNDARIES_RIGHT{
                position.0 = Vec2::new(0., 0.);
                velocity.0 = Vec2::new(-1. * BALL_VELOCITY, BALL_VELOCITY);
        } else if position.0.x <= FIELD_BOUNDARIES_LEFT {
                position.0 = Vec2::new(0., 0.);
                velocity.0 = Vec2::new(BALL_VELOCITY, BALL_VELOCITY);
            }
        }
    }

pub fn move_right_paddle(
    ball: Query<&Position, With<Ball>>,
    mut paddles: Query<(&mut Position, &Paddle), Without<Ball>>,
) {
    for (mut paddle_pos, paddle) in &mut paddles {
        if paddle.location == PaddleLocation::Right {
            if let Ok(position) = ball.get_single() {
                if position.0.x > 0. && position.0.x < paddle_pos.0.x{
                    if position.0.y < paddle_pos.0.y {
                        paddle_pos.0.y -= PADDLE_VELOCITY;
                    } else {
                        paddle_pos.0.y += PADDLE_VELOCITY;
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
                paddle_pos.0.y += PADDLE_VELOCITY;
            }
            if input.pressed(KeyCode::ArrowDown)
                && paddle_pos.0.y - PADDLE_HEIGHT / 2.0 - WALL_HEIGHT / 2.0 > FIELD_BOUNDARIES_BOTTOM
            {
                paddle_pos.0.y -= PADDLE_VELOCITY;
            }
        }
    }
}
