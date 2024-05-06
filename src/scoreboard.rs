use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res, Resource},
    },
    prelude::TextBundle,
    render::color::Color,
    text::TextSection,
    text::{Text, TextStyle},
    ui::{PositionType, Style, Val},
    utils::default,
};

const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCOREBOARD_FONT_SIZE: f32 = 20.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Resource, Default)]
pub struct Scoreboard {
    pub points_left: u8,
    pub points_right: u8,
}

#[derive(Component)]
pub struct ScoreboardUI;

pub fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        ScoreboardUI,
        TextBundle::from_sections([
            TextSection::new(
                "Score left: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
            TextSection::new(
                "Score right: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    ));
}

pub fn update_scoreboard(
    scoreboard: Res<Scoreboard>,
    mut query: Query<&mut Text, With<ScoreboardUI>>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.points_left.to_string();
    text.sections[3].value = scoreboard.points_right.to_string();
}
