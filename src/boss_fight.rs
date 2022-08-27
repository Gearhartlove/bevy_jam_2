use bevy::prelude::*;

pub struct BossFightPlugin;

impl Plugin for BossFightPlugin {
    fn build(&self, app: &mut App) {

    }
}

//=================================================================================================
//                              Setup
//=================================================================================================

pub fn setup_boss_fight( commands : Commands, asset_server : Res<AssetServer> ) {

}

//=================================================================================================
//                              Timer
//=================================================================================================

#[derive(Component)]
pub struct BossTimer {

}