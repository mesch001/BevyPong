use bevy::prelude::*;

mod position;
use position::{project_positions, handle_collisions};

mod ball;
use ball::{move_ball, spawn_ball};

mod paddle;
use paddle::spawn_paddles;

mod wall;
use wall::spawn_walls;

mod movement;
use movement::{move_paddles, handle_player_input};

mod event;
use event::{Score, Scored, detect_scoring, reset_ball,update_score};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<Score>()
    .add_event::<Scored>()
    .add_systems(
        Startup,
        (
            spawn_ball,
            spawn_camera,
            spawn_paddles,
            spawn_walls,
        ),
    )
    .add_systems(
        Update,
        (
            move_ball,
            handle_player_input,
            detect_scoring,
            reset_ball.after(detect_scoring),
            update_score.after(detect_scoring),
            move_paddles.after(handle_player_input),
            project_positions.after(move_ball),
            handle_collisions.after(move_ball),
        ),
    ).run()
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}
