use bevy::prelude::*;
use crate::AppState;

pub struct MixerPlugin;

impl Plugin for MixerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/mixer.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -250., 0.),
            ..default()
        })
        .insert(Name::new("Mixer"));
}