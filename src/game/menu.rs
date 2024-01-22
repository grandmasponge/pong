use bevy::{prelude::*, app::AppExit};

use crate::AppState;

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub enum ButtonCommands {
    Play,
    Quit,
}

pub fn menu(mut commands: Commands) {


    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: Color::BLACK,
        ..default()
    };

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..default()   
        },
        Menu
    ))
    .with_children(|parent| {
        parent
        .spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
         })
         .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section("PONG 2", TextStyle {
                    font_size: 100.0,
                    color: Color::WHITE.into(),
                    ..default()
                })
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                })
            );
         }); 
         parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: Color::WHITE.into(),
                ..default()
            },
            ButtonCommands::Play
         ))
         .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Play", button_text_style.clone()
                )
            );
         });  
         parent.spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: Color::WHITE.into(),
                ..default()
            },
            ButtonCommands::Quit
         )).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "quit", button_text_style.clone()
                )
            );
         });
         });
         
}


pub fn menu_action(interaction_query: Query<(&Interaction, &ButtonCommands)>, mut next_state: ResMut<NextState<AppState>>, mut app_exit: EventWriter<AppExit>) {
    for (interaction, command) in &interaction_query {
           if *interaction == Interaction::Pressed {
               match command {
                   ButtonCommands::Play => {
                      next_state.set(AppState::InGame);
                   }
                   ButtonCommands::Quit => {
                       app_exit.send(AppExit);
                   }
               }
           }
    }
}

pub fn despawn_screen(to_despawn: Query<Entity, With<Menu>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}