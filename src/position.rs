
use::bevy::prelude::*;

use bevy::transform::components::Transform;
use crate::components::Position;

pub const FIELD_BOUNDARIES_LEFT: f32 = -500.0;
pub const FIELD_BOUNDARIES_RIGHT: f32 = 500.0;
pub const FIELD_BOUNDARIES_TOP: f32 = 350.0;
pub const FIELD_BOUNDARIES_BOTTOM: f32 = -350.0;

pub fn project_positions(mut postionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut postionables {
        transform.translation = position.0.extend(0.);
    }
}

