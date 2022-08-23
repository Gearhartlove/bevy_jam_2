use bevy::prelude::*;
use crate::AppState;
use crate::quest::Quest;

pub struct NpcPlugin;

impl NpcPlugin {
    pub const GOBLIN_NPC: &'static str = "sprites/goblin.png";
}

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_npc));
    }
}

fn spawn_npc(mut commands: Commands, asset_server: Res<AssetServer>, current_quest: Res<Quest<'static>>) {
    let npc_file_path = current_quest.npc;
    commands.spawn_bundle(SpriteBundle{
        sprite: Sprite {
            custom_size: Some(Vec2::splat(128.)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(384., -128., 1.),
            ..default()
        },
        texture: asset_server.load(npc_file_path),
        visibility: Default::default(),
        computed_visibility: Default::default(),
        ..default()
    })
        .insert(NpcRenderer)
        .insert(Name::new("Goblin"))
    ;
}

#[derive(Component)]
struct NpcRenderer;