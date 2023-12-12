use bevy::prelude::*;

#[derive(Resource)]
struct MenuData {
    node: Entity,
}

pub fn menu_setup(mut commands: Commands) {}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.node).despawn_recursive();
}

pub fn menu() {}
