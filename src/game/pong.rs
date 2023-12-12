use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
pub struct ball {
    position: Position,
}

#[derive(Component, Clone)]
pub struct Paddel {
    name: String,
    position: Position,
    score: usize,
    round_score: usize,
}

#[derive(Resource, Default)]
pub struct GameState {
    current_round: usize,
    winning_player: Option<String>,
}

#[derive(Resource)]
pub struct GameRules {
    round_score: usize,
    winning_score: usize,
    max_rounds: usize,
}

pub fn check_score(
    rules: Res<GameRules>,
    mut state: ResMut<GameState>,
    query: Query<&Paddel>
) {
    for player in &query {
        if player.score == rules.winning_score {
            state.winning_player = Some(player.name.clone())
        }
    }
}

pub fn check_round_score(
    rules: Res<GameRules>,
    mut state: ResMut<GameState>,
    mut query: Query<&mut Paddel>,
) {
    for mut player in &mut query {
        if player.round_score == rules.round_score {
            player.score += 1;
            state.current_round += 1;
            println!("{} won the round", player.name);
        }
    }
}

pub fn game_over(rules: Res<GameRules>, state: Res<GameState>) {
    if let Some(player) = &state.winning_player {
        println!("{player} won the game");
    } else if state.current_round == rules.max_rounds {
        println!("nobody won the game DRAW")
    }
}

pub fn startup_game(mut commands: Commands) {
    commands.insert_resource(GameRules {
        round_score: 3,
        winning_score: 5,
        max_rounds: 3,
    });

    commands.spawn_batch(vec![
        (Paddel {
            name: "left".to_string(),
            position: Position { x: 0.0, y: 0.0 },
            round_score: 0,
            score: 0,
        }),
        (Paddel {
            name: "right".to_string(),
            position: Position { x: 0.0, y: 0.0 },
            round_score: 0,
            score: 0,
        }),
    ]);
}

pub fn player_movment() {
    
}
