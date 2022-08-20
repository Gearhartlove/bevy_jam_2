mod mixer;

use bevy::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum AppState {
    Game
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("323E40").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}