mod game;

use bevy::{prelude::*, window::WindowMode};

#[derive(Component)]
pub struct MusicBox;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "pong2".into(),
                resolution: (800., 600.).into(),
                fit_canvas_to_parent: true,
                window_theme: Some(bevy::window::WindowTheme::Dark),
                visible: true,
                resizable: true,
                mode: WindowMode::Windowed,
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
        .add_systems(Startup, game::pong::spawn_game_items)
        .add_systems(Update, (game::pong::collision_detection, game::pong::update_ball_direction,))
        .add_systems(
            Update,
            (
                game::pong::player_movment,
                game::pong::ballmovment,
                game::pong::powerup_collisions,
                game::pong::check_for_goal,
                game::pong::update_scoreboard,
                game::pong::spawn_powerups,
                game::pong::end_game_checker,
            ).run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (game::pong::end_game.run_if(in_state(AppState::GameOver))),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(AudioBundle {
        source: asset_server.load("main_music.mp3"),
        settings: PlaybackSettings::LOOP,
    });
}

pub fn volume_system(
    keyboard_input: Res<Input<KeyCode>>,
    music_box_query: Query<&AudioSink, With<MusicBox>>,
) {
    if let Ok(sink) = music_box_query.get_single() {
        if keyboard_input.just_pressed(KeyCode::Plus) {
            sink.set_volume(sink.volume() + 0.1);
            println!("volume: {}", sink.volume());
        } else if keyboard_input.just_pressed(KeyCode::Minus) {
            sink.set_volume(sink.volume() - 0.1);
            println!("volume: {}", sink.volume());
        }
    }
}

pub fn player_movment() {}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    Menu,
    #[default]
    InGame,

    Paused,
    GameOver,
}
