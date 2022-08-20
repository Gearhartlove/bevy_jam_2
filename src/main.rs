mod mixer;
mod registry;
mod element;
mod mixer;
mod furnace;
mod slicer;

use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};
use crate::mixer::MixerPlugin;
use crate::AppState::{Game, Setup};
use crate::registry::{MixerRecipeIden, RegistryPlugin};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum AppState {
    Game,
    Setup
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("323E40").unwrap()))
        .add_state(Game)
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            title: "Fantastical Kitchen".to_string(),
            cursor_visible: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::hex("323E40").unwrap())) // sets background color
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprite
        .add_plugins(DefaultPlugins)
        .add_plugin(RegistryPlugin)
        .add_plugin(WorldInspectorPlugin::new()) // debugging window
        .add_plugin(MixerPlugin)
        .add_state(AppState::Game)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}