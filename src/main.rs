mod mixer;

use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};
use crate::AppState::Game;
use crate::mixer::MixerPlugin;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum AppState {
    Game,
    Store,
    Loading
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("323E40").unwrap())) // sets background color
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprite
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new()) // debugging window
        .add_plugin(MixerPlugin)
        .add_state(AppState::Loading)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}