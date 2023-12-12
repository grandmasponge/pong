mod game;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Startup, game::pong::spawn_paddels)
        .add_systems(Startup, game::pong::spawn_ball)
        .add_systems(Startup, game::pong::spawn_scoreboard)
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
