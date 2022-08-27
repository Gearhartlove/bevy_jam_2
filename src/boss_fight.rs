use bevy::prelude::*;
use crate::GameHelper;
use crate::helper::add_scaled_pixel_asset;

pub struct BossFightPlugin;

impl Plugin for BossFightPlugin {
    fn build(&self, app: &mut App) {
        //app.add_startup_system(setup_boss_fight);
    }
}

//=================================================================================================
//                              Setup
//=================================================================================================

pub fn setup_boss_fight(mut commands: Commands, asset_server : Res<AssetServer> ) {
    let parent = add_scaled_pixel_asset(&mut commands, &asset_server, "sprites/boss_fight_ui.png", 56, 76, SpriteBundle {
        transform : Transform::from_xyz(0.0, 0.0, 60.0),
        ..default()
    }).insert(Name::new("Boss Fight Menu")).id();
}

//=================================================================================================
//                              Timer
//=================================================================================================

#[derive(Component)]
pub struct BossTimer {
    timer : Timer,
}