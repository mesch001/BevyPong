use bevy::prelude::*;

mod position;
use position::project_positions;

mod ball;
use ball::spawn_ball;

mod paddle;
use paddle::spawn_paddles;

mod wall;
use wall::spawn_walls;

mod scoreboard;
use scoreboard::{spawn_scoreboard, update_scoreboard, Scoreboard};

mod collision;
use collision::handle_collisions;

mod components;

mod movement;
use movement::{move_player_paddle, move_right_paddle, move_ball, reset_ball};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard::default())
        .add_systems(
            Startup,
            (
                setup,
                spawn_ball,
                spawn_walls,
                spawn_scoreboard,
                spawn_paddles,
            ),
        )
        .add_systems(
            Update,
            (
                move_ball,
                project_positions.after(move_ball),
                handle_collisions.after(move_ball),
                reset_ball,
                update_scoreboard,
                move_right_paddle,
                move_player_paddle,
            ),
        )
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}
