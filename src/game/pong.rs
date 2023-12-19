use bevy::{
    ecs::query,
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        MaterialMesh2dBundle,
    },
    window::{CursorGrabMode, PresentMode, WindowLevel, WindowTheme},
};
use std::{thread, time};
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
    raduis: f32,
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

//screen heights
const SCREEN_WIDTH: f32 = 1920.;
const SCREEN_HEIGHT: f32 = 1080.;

//paddel dimensions
const PADDEL_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 100.;

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
        direction = 300.;
    } else {
        direction = -300.;
    }

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Ball {
            speed: 300.,
            raduis: 10.,
        },
        Velocity(Vec2::new(direction, 300.).normalize()),
        Collider,
    ));
}
//spawn scoreboard
pub fn spawn_scoreboard(mut commands: Commands) {
   let score=  commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score:",
                 TextStyle {
                    font: default(),
                    color: Color::ANTIQUE_WHITE,
                    font_size: 60.
                 },
            ),
            TextSection::from_style(TextStyle {
                font: default(),
                color: Color::ANTIQUE_WHITE,
                font_size: 60.
            }),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Start,
            ..default()
        })
    );
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
                direction_y += 500.;
            }
            if key_input.pressed(KeyCode::D) {
                direction_y -= 500.;
            }
        } else {
            if key_input.pressed(KeyCode::Up) {
                direction_y += 500.;
            }
            if key_input.pressed(KeyCode::Down) {
                direction_y -= 500.;
            }
        }
        let new_translation = tranform.translation.y + direction_y * time_delta;
        let clamp = SCREEN_HEIGHT / 2. - PADDEL_WIDTH / 2.;
        tranform.translation.y = new_translation.clamp(-clamp, clamp);
    }
}

pub fn ballmovment(mut query: Query<(&mut Transform, &mut Velocity, &Ball)>, time: Res<Time>) {
    //move ball

    for (mut transform, velocity, ball) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_seconds() * &ball.speed;
        transform.translation.y += velocity.0.y * time.delta_seconds() * &ball.speed;
    }
}

pub fn update_ball_direction(mut query: Query<(&Transform, &mut Ball, &mut Velocity)>) {
    let (mut ball_transform, mut ball, mut ball_velocity) = query.single_mut();
    let half_ball_size = ball.raduis / 2.;
    let x_min = -SCREEN_WIDTH / 2. + half_ball_size;
    let y_min = -SCREEN_HEIGHT / 2. + half_ball_size;
    let x_max = SCREEN_WIDTH / 2. - half_ball_size;
    let y_max = SCREEN_HEIGHT / 2. - half_ball_size;

    let trans = ball_transform.translation;
    if trans.y < y_min || trans.y > y_max {
        ball_velocity.0.y *= -1.0;
    }
}

pub fn collision_detection(
    mut ball_query: Query<(&mut Ball, &mut Velocity, &Transform)>,
    paddel_query: Query<(&Player, &Transform)>,
) {
    let (mut ball, mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = Vec2::new(10., 10.);

    for (_player, player_tranform) in &paddel_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            player_tranform.translation,
            Vec2::new(PADDEL_WIDTH, PADDLE_HEIGHT),
        );
        if let Some(collision) = collision {
            let mut new_velocity = ball_velocity.0;
            match collision {
                Collision::Top => {
                    ball.speed += 100.0;
                    new_velocity.y *= -1.0;
                }
                Collision::Bottom => {
                    ball.speed += 100.0;
                    new_velocity.y *= -1.0;
                }
                Collision::Left => {
                    ball.speed += 100.0;
                    new_velocity.x *= -1.0;
                }
                Collision::Right => {
                    ball.speed += 100.0;
                    new_velocity.x *= -1.0;
                }
                _ => new_velocity *= -1.0,
            }

            ball_velocity.0 = new_velocity;
        }
    }
}

pub fn check_for_goal(mut query: Query<(&mut Transform, &mut Ball)>, mut score_board: ResMut<ScoreBoard>) {
    let (mut ball_transform, mut ball) = query.single_mut();
    let half_ball_size = ball.raduis / 2.;
    let x_min = -SCREEN_WIDTH / 2. + half_ball_size;
    let x_max = SCREEN_WIDTH / 2. - half_ball_size;
    let trans = ball_transform.translation;
    if x_min > trans.x {
        score_board.score_right += 1;
        ball_transform.translation = Vec3::new(0.,0.,0.);
        ball.speed = 300.;
        thread::sleep(time::Duration::from_secs(5));

    }
    if x_max < trans.x {
        score_board.score_left += 1;
        ball_transform.translation = Vec3::new(0.,0.,0.);
        ball.speed = 300.;
        thread::sleep(time::Duration::from_secs(5));
    }
}

pub fn update_scoreboard(mut query: Query<&mut Text>, score_board: Res<ScoreBoard>) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{} : {}", score_board.score_left, score_board.score_right);
}
