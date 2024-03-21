use bevy::prelude::*;

mod position;
use position::{collision, project_positions};

mod ball;
use ball::{move_ball, spawn_ball};

mod wall;
use wall::spawn_walls;

mod scoreboard;
use scoreboard::{spawn_scoreboard, Scoreboard, update_scoreboard};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard::default())
        .add_systems(Startup, (setup, spawn_ball, spawn_walls, spawn_scoreboard))
        .add_systems(
            Update,
            (
                collision.before(move_ball),
                move_ball,
                project_positions.after(move_ball),
                update_scoreboard
            ),
        )
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}
