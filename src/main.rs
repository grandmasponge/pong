mod game;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "pong2".into(),
                resolution: (1920., 1080.).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                window_theme: Some(bevy::window::WindowTheme::Dark),
                visible: true,
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Startup, game::pong::spawn_paddels)
        .add_systems(Startup, game::pong::spawn_ball)
        .add_systems(Startup, game::pong::spawn_scoreboard)
        .add_systems(Startup, game::pong::spawn_recources)
        .add_systems(Update, game::pong::player_movment)
        .add_systems(Update, game::pong::ballmovment)
        .add_systems(Update, game::pong::update_ball_direction)
        .add_systems(Update, game::pong::collision_detection)
        .add_systems(Update, game::pong::check_for_goal)
        .add_systems(Update, game::pong::update_scoreboard)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn player_movment() {}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
    Paused,
    GameOver,
}
