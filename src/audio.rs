use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::game::GameManager;
use crate::npc::{Npc, NpcKind, Say};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SayEvent>()
            .init_resource::<AudioManager>()
            .add_plugin(bevy_kira_audio::AudioPlugin)
            .add_audio_channel::<DialogueChannel>()
            .add_startup_system(start_background_audio)
            .add_system(play_dialogue_voice)
            .add_system(stop_dialogue_voice);
    }
}

struct DialogueChannel;

struct AudioManager {
    total_say_duration: f64,
    say_progress: f64,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self {
            total_say_duration: 0.,
            say_progress: 0.,
        }
    }
}

impl AudioManager {}

pub struct SayEvent(pub f64);

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    println!("start bg music");
    audio.play(asset_server.load("sounds/tavern_music.wav")).looped()
        .loop_from(7.0)
        .with_volume(0.05);
}

fn start_boss_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.stop();
    audio.play(asset_server.load("sounds/boss_music.wav")).looped()
        .loop_from(2.0)
        .with_volume(0.05);
}

fn play_dialogue_voice(
    mut say_event: EventReader<SayEvent>,
    game: Res<GameManager>,
    dialogue: Res<AudioChannel<DialogueChannel>>,
    asset_server: Res<AssetServer>,
    audio_manager: ResMut<AudioManager>,
    // say_query: Query<Say>
) {
    // todo: only play once
    for _ in say_event.iter() {
        dialogue.stop();
        match game.npc_data.get_current_npc().unwrap().kind {
            NpcKind::Squee => {
                dialogue.play(asset_server.load("sounds/squee_voice.wav")).looped().with_volume(0.05);
            }
            NpcKind::Conrad => {
            }
            NpcKind::Pumkinhead => {}
            NpcKind::Gordon => {}
        }
    }
}

fn stop_dialogue_voice(
    mut say_event_reader: EventReader<SayEvent>,
    mut audio_manager: ResMut<AudioManager>,
    game: Res<GameManager>,
    dialogue: Res<AudioChannel<DialogueChannel>>,
    time: Res<Time>,
) {
    // if say event, change the Audio Manager Say length and reset the progress
    for say_event in say_event_reader.iter() {
        audio_manager.total_say_duration = say_event.0;
        audio_manager.say_progress = 0.;
    }

    audio_manager.say_progress += time.delta_seconds_f64();

    let difference = audio_manager.total_say_duration - audio_manager.say_progress;

    // debug
    // println!("difference: {} duration: {} progress/100: {}", difference, audio_manager.total_say_duration, audio_manager.say_progress);

    if difference < -0.5 {
        println!("stop");
        dialogue.stop();
    }
}

fn page_turning(

) {

}
