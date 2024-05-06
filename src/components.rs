use bevy::prelude::*;
use bevy::math::Vec2;

#[derive(Component)]
pub struct Shape(pub Vec2);

#[derive(Component, Debug)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);
