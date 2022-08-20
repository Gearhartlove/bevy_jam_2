//mod mixer;
mod registry;
mod element;
mod mixer;
mod furnace;
mod slicer;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use bevy::prelude::*;
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
        .add_plugins(DefaultPlugins)
        .add_plugin(RegistryPlugin)
        .add_startup_system(setup_camera)
        .run()
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}