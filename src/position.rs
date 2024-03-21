use bevy::ecs::{component::Component, system::Query};
use bevy::math::Vec2;
use bevy::prelude::{With, ResMut};
use bevy::transform::components::Transform;

use crate::ball::Ball;
use crate::wall::WALL_HEIGHT;
use crate::scoreboard::Scoreboard;

pub const FIELD_BOUNDARIES_LEFT: f32 = -500.0;
pub const FIELD_BOUNDARIES_RIGHT: f32 = 500.0;
pub const FIELD_BOUNDARIES_TOP: f32 = 350.0;
pub const FIELD_BOUNDARIES_BOTTOM: f32 = -350.0;
const WALL_STOP: f32 = WALL_HEIGHT / 2.0;

#[derive(Component)]
pub struct Position(pub Vec2);

pub fn project_positions(mut postionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut postionables {
        transform.translation = position.0.extend(0.);
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub fn collision(
    mut scoreboard: ResMut<Scoreboard>,
    mut ball: Query<(&Position, &mut Velocity), With<Ball>>) {
    if let Ok((position, mut velocity)) = ball.get_single_mut() {
        if position.0.x <= FIELD_BOUNDARIES_LEFT - velocity.0.x {
            scoreboard.points_left += 1;
            velocity.0.x += -2.0 * velocity.0.x;
        } else if position.0.x >= FIELD_BOUNDARIES_RIGHT - velocity.0.x {
            scoreboard.points_right += 1;
            velocity.0.x -= 2.0 * velocity.0.x;
        } else if position.0.y >= FIELD_BOUNDARIES_TOP - WALL_STOP - velocity.0.y {
            velocity.0.y -= 2.0 * velocity.0.y;
        } else if position.0.y <= FIELD_BOUNDARIES_BOTTOM + WALL_STOP - velocity.0.y {
            velocity.0.y += -2.0 * velocity.0.y;
        }
    }
}
