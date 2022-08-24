use bevy::prelude::*;
use bevy_inspector_egui::egui::FontSelection::Style;
use crate::AppState;
use crate::element::Element;
use crate::game::{Game, GameStatus};
use crate::game::GameStatus::QuestComplete;
use crate::quest::Quest;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_dialogue))
            .add_system(dialogue);
    }
}

#[derive(Component)]
pub struct Npc {
    pub kind: NpcKind,
    pub name: String,
    pub sprite: Handle<Image>,
    // pub voice: Handle<Audio>,
}

impl Npc {
    pub const GOBLIN_NPC: &'static str = "sprites/goblin.png";
}

#[derive(PartialEq, Eq)]
pub enum NpcKind {
    Squee,
}

//==================================================================================================
// Everything dialogue related below, inspired from @Inspirateur's 'Undoing' dialogue system in charter.rs
// link: https://github.com/Inspirateur/Undoing/blob/main/src/character.rs
//==================================================================================================

#[derive(Component)]
pub struct Say {
    // npc: String,
    text: String,
    i: usize,
    start: f64,
    duration: f64,
}

impl Say {
    const CHAR_SEC: f64 = 0.04;

    pub fn new(text: impl ToString) -> Self {
        Say {
            text: text.to_string(),
            i: 0,
            start: -1.,
            // gets the total duration of the text talking
            duration: text
                .to_string()
                .chars()
                .map(Say::char_duration)
                .fold(0., |acc, x| acc + x)
                * Say::CHAR_SEC, // todo: tweek
        }
    }

    fn char_duration(char: char) -> f64 {
        match char {
            ' ' => 1.5,
            ',' => 3.,
            '.' | '!' | '?' => 6.,
            _ => 1.,
        }
    }

    // todo: rename, find use cases
    pub fn compute_i(&self, now: f64) -> usize {
        let delta = now - self.start;
        let mut count = 0.;
        let mut new_i = 0;
        for char in self.text.chars() {
            count += Say::CHAR_SEC * Say::char_duration(char);
            new_i += 1;
            if count > delta {
                break;
            }
        }
        new_i
    }
}

#[derive(Component)]
pub struct NpcText;

#[derive(Component)]
pub struct NpcSprite;

/// Spawns the sprite and the text box for the npc
fn setup_dialogue(mut commands: Commands, asset_server: Res<AssetServer>, current_quest: Res<Quest<'static>>, game: Res<Game>) {
    let npc_file_path = current_quest;

    // npc
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(128.)),
            ..default()
        },
        transform: Transform::from_xyz(384., -128., 1.),
        texture: asset_server.load("sprites/empty.png"), // why?
        ..default()
    })
        .insert(NpcSprite)
        .insert(Name::new("NpcSprite"));

    // text bubble
    let font = asset_server.load("fonts/JetBrainsMono.ttf");
    // todo: change
    let text_style = TextStyle {
        font,
        font_size: 20.,
        color: Color::WHITE,
    };
    // todo: change?
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Center,
    };

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("", text_style).with_alignment(text_alignment),
        transform: Transform::from_xyz(384., 93., 0.),
        ..default()
    })
        .insert(NpcText)
        .insert(Name::new("Npc Text"));
}

fn dialogue(
    mut commands: Commands,
    mut query: Query<(Entity, &Npc, &mut Say)>,
    mut query_text: Query<&mut Text, With<NpcText>>,
    mut query_sprite: Query<&mut Handle<Image>, With<NpcSprite>>,
    time: Res<Time>,
    // audio: Res<Audio>
) {
    if let Ok((entity, npc, mut say)) = query.get_single_mut() {
        if let Ok(mut text) = query_text.get_single_mut() {
            if let Ok(mut sprite) = query_sprite.get_single_mut() {
                if say.i == 0 {
                    println!("finished");
                    say.start = time.seconds_since_startup();
                    *sprite = npc.sprite.clone();
                }
                // compute the new i
                let now = time.seconds_since_startup();
                let mut new_i = say.compute_i(now);
                // if we finished
                if say.i >= say.text.len() {
                    // and 1 sec has passed
                    if now - say.duration - say.start > 1. {
                        commands.entity(entity).remove::<Say>();
                    }
                }
                // if not finished
                else if new_i != say.i {
                    // there's new characters to say
                    new_i = new_i.min(say.text.len());
                    // magic line that updates the code by making the old text box equal to the new
                    // sliced text box
                    text.sections[0].value = say.text[0..new_i].to_string();
                    // if i..new_i is not only spaces, produce a sound
                    // if say.text[say.i..new_i].trim().len() > 0 {
                    // audio.play(character.voice.clone());
                    // }
                    say.i = new_i;
                }
            }
        }
    }
}

