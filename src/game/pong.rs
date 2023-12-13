use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{CursorGrabMode, PresentMode, WindowLevel, WindowTheme},
};
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}
#[derive(Component, Debug)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Ball {
    speed: f32,
}

#[derive(Component)]
pub struct Collider;

#[derive(Component, Clone)]
pub struct Player {
    score: usize,
    side: Side,
}

#[derive(Resource)]
pub struct ScoreBoard {
    score_left: usize,
    score_right: usize,
}

#[derive(Resource, Default)]
pub struct GameState {
    current_round: usize,
    current_winning_player: Option<Side>,
}

#[derive(Resource)]
pub struct GameRules {
    max_rounds: usize,
    max_score: usize,
}

pub fn spawn_paddels(mut commands: Commands) {
    //paddel left
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10., 100.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-700., 0., 0.),
                ..default()
            },
            ..default()
        },
        Player {
            score: 0,
            side: Side::Left,
        },
        Collider,
    ));

    // paddel right
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10., 100.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(700., 0., 0.),
                ..default()
            },
            ..default()
        },
        Player {
            score: 0,
            side: Side::Right,
        },
        Collider,
    ));
}
//spawn ball
pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..100);
    let direction: f32;

    if num > 50 {
        direction = 3000.;
    } else {
        direction = -3000.;
    }

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Ball { speed: 400. },
        Velocity(Vec2::new(direction, 0.).normalize()),
        Collider,
    ));
}
//spawn scoreboard
pub fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Score: 0 | 0",
            TextStyle {
                font: default(),
                font_size: 60.,
                color: Color::WHITE,
            },
        ),
        transform: Transform::from_translation(Vec3::new(0., 450., 0.)),
        ..default()
    });
}

pub fn spawn_recources(mut commands: Commands) {
    //init all recources
    commands.insert_resource(ScoreBoard {
        score_left: 0,
        score_right: 0,
    });
    commands.insert_resource(GameRules {
        max_rounds: 3,
        max_score: 5,
    });
    commands.insert_resource(GameState {
        current_round: 0,
        current_winning_player: None,
    });
}

pub fn player_movment(
    mut query: Query<(&mut Transform, &Player)>,
    key_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let time_delta = time.delta_seconds();
    for (mut tranform, player) in query.iter_mut() {
        let mut direction_y: f32 = 0.;
        if player.side == Side::Left {
            if key_input.pressed(KeyCode::A) {
                direction_y += 200.;
            }
            if key_input.pressed(KeyCode::D) {
                direction_y -= 200.;
            }
        } else {
            if key_input.pressed(KeyCode::Up) {
                direction_y += 200.;
            }
            if key_input.pressed(KeyCode::Down) {
                direction_y -= 200.;
            }
        }
        let new_translation = tranform.translation.y + direction_y * time_delta;
        tranform.translation.y = new_translation.clamp(-540., 540.);
    }
}

pub fn ballmovment(mut query: Query<(&mut Transform, &mut Velocity, &Ball)>, time: Res<Time>) {
 //move ball
    
    for (mut transform, velocity, ball) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_seconds() * &ball.speed;
        transform.translation.y += velocity.0.y* time.delta_seconds() * &ball.speed;
    }
    
}   
