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
use crate::registry::{MixerRecipeIden, Registry, RegistryPlugin};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum AppState {
    Game,
    Setup,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("323E40").unwrap()))
        .add_state(Game)
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            title: "Fantastical Kitchen".to_string(),
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(RegistryPlugin)
        .add_plugin(WorldInspectorPlugin::new()) // debugging window
        //.add_plugin(MixerPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_game_background)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_game_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("sprites/proto_kitchen_recipe.png"),
        ..default()
    });
}

#[derive(Component)]
struct Page;