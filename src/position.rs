use bevy::ecs::{component::Component, system::Query};
use bevy::math::Vec2;
use bevy::prelude::{ResMut, With, Without};
use bevy::transform::components::Transform;

use crate::ball::{Ball, BALL_SIZE};
use crate::paddle::PADDLE_HEIGHT;
use crate::scoreboard::Scoreboard;
use crate::wall::WALL_HEIGHT;

pub const FIELD_BOUNDARIES_LEFT: f32 = -500.0;
pub const FIELD_BOUNDARIES_RIGHT: f32 = 500.0;
pub const FIELD_BOUNDARIES_TOP: f32 = 350.0;
pub const FIELD_BOUNDARIES_BOTTOM: f32 = -350.0;
const WALL_STOP: f32 = WALL_HEIGHT / 2.0;
pub const VELOCITY: f32 = 5.;

#[derive(Component, Debug)]
pub struct Position(pub Vec2);

pub fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub fn collision(
    mut scoreboard: ResMut<Scoreboard>,
    mut ball: Query<(&Position, &mut Velocity), With<Ball>>,
    paddles: Query<&Position, Without<Ball>>,
) {
    if let Ok((ball_pos, mut velocity)) = ball.get_single_mut() {
        for paddle_pos in &paddles {
            if ball_pos.0.x == paddle_pos.0.x
                && ((ball_pos.0.y <= paddle_pos.0.y + PADDLE_HEIGHT / 2.)
                    && (ball_pos.0.y >= paddle_pos.0.y - PADDLE_HEIGHT / 2.))
            {
                velocity.0.x += -2.0 * velocity.0.x;
            }
        }

        if ball_pos.0.x <= FIELD_BOUNDARIES_LEFT - velocity.0.x {
            scoreboard.points_left += 1;
            velocity.0.x += -2.0 * velocity.0.x;
        } else if ball_pos.0.x >= FIELD_BOUNDARIES_RIGHT - velocity.0.x {
            scoreboard.points_right += 1;
            velocity.0.x += -2.0 * velocity.0.x;
        } else if ball_pos.0.y >= FIELD_BOUNDARIES_TOP - WALL_STOP - BALL_SIZE - velocity.0.y {
            velocity.0.y += -2.0 * velocity.0.y;
        } else if ball_pos.0.y <= FIELD_BOUNDARIES_BOTTOM + WALL_STOP + BALL_SIZE - velocity.0.y {
            velocity.0.y += -2.0 * velocity.0.y;
        }
    }
}
