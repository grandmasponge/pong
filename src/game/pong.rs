use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Clone, Copy)]
pub enum Side {
    left,
    right
}


#[derive(Component)]
pub struct Ball {
    speed: f32
}

#[derive(Component)]
pub struct Collider;

#[derive(Component, Clone)]
pub struct Player {
    score: usize,
    side: Side
}

#[derive(Resource)]
pub struct ScoreBoard {
    score_left: usize,
    score_right: usize
}


#[derive(Resource, Default)]
pub struct GameState {
   
}

#[derive(Resource)]
pub struct GameRules {
    max_rounds: usize,
    max_score: usize,

}


pub fn spawn_paddels(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    //paddel left
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10., 100.)),
                ..default()
            },
            transform : Transform {
                translation: Vec3::new(-700., 0., 0.),
                ..default()
            },
            ..default()
        },
        Player {
            score: 0,
            side: Side::left
        },
        Collider
    ));

    // paddel right
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10., 100.)),
                ..default()
            },
            transform : Transform {
                translation: Vec3::new(700., 0., 0.),
                ..default()
            },
            ..default()
        },
        Player {
            score: 0,
            side: Side::right
        },
        Collider
    ));

}


pub fn spawn_ball(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,  mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Ball {
            speed: 0.
        },
        Collider,
    ));
}


pub fn start_recources() {

}



pub fn game_over(rules: Res<GameRules>, state: Res<GameState>) {
 
}





pub fn player_movment() {

}


  