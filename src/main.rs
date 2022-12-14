mod registry;
mod element;
mod mixer;
mod furnace;
mod slicer;
mod ui;
mod helper;
mod quest;
mod npc;
mod game;
mod page;
mod boss_fight;
mod gameflow;
mod audio;

use std::time::Duration;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_inspector_egui::{WorldInspectorPlugin};
use crate::AppState::{Game};
use crate::registry::{MixerRecipeIden, RegistryPlugin};
use crate::ui::UiPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use crate::boss_fight::BossFightPlugin;
use crate::game::GamePlugin;
use crate::gameflow::GameflowPlugin;
use crate::helper::{GameHelper, HelperPlugin};
use crate::npc::NpcPlugin;
use crate::page::PagePlugin;
use crate::quest::{QuestPlugin};
use crate::audio::AudioPlugin;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum AppState {
    Game,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("183f39").unwrap()))
        .add_state(Game)
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            title: "Fantastical Kitchen".to_string(),
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        // .add_plugin(GamePlugin)
        .add_plugin(GameflowPlugin)
        .add_plugin(RegistryPlugin)
        .add_plugin(HelperPlugin)
        .add_plugin(UiPlugin)
        //.add_plugin(WorldInspectorPlugin::new()) // debugging window
        .add_plugin(QuestPlugin)
        .add_plugin(NpcPlugin)
        .add_plugin(PagePlugin)
        .add_plugin(BossFightPlugin)
        .add_plugin(AudioPlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}