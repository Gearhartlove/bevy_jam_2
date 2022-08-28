mod squee;

use bevy::ecs::system::Command;
use bevy::math::Vec2Swizzles;
use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy_inspector_egui::egui::FontSelection::Style;
use bevy_prototype_debug_lines::DebugLines;
use imagesize::size;
use crate::{AppState, GameHelper};
use crate::element::Element;
use crate::game::{GameManager, GameStatus};
use crate::game::GameStatus::QuestComplete;
use crate::quest::Quest;
use crate::ui::{NPC_LEVEL, Rect, Slot};

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NpcClickEvent>()
            .init_resource::<NPCData>()
            .add_startup_system(setup_npc_assets)
            .add_system(click_npc)
            .add_system(dialogue);
    }
}

#[derive(Component)]
pub struct Npc {
    pub kind: NpcKind,
    pub name: String,
    pub sprite: Handle<Image>,
    pub sprite_path: String,
    pub talking_anims: Vec<Handle<Image>>,
    pub talking_index: usize,
    // pub color: Color,
    // pub voice: Handle<Audio>,
}

impl Npc {
    pub fn talk_frame(&mut self) -> usize {
        self.talking_index += 1;
        let i = &self.talking_index % 2;
        return i
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum NpcKind {
    Squee,
    Conrad,
    Pumkinhead,
    Gordon,
}

//==================================================================================================
//                  Setup
//==================================================================================================

fn setup_npc_assets(
    mut commands : Commands,
    asset_server : Res<AssetServer>,
    mut game : ResMut<GameManager>
) {
    //Squee
    let squee = Npc {
        kind: NpcKind::Squee,
        name: "Squee the Thumbless".to_string(),
        sprite: asset_server.load("sprites/squee.png"),
        sprite_path: "sprites/squee.png".to_string(),
        talking_anims: vec![
            asset_server.load("sprites/squee_talk1.png"),
            asset_server.load("sprites/squee_talk2.png"),
        ],
        talking_index: 0,
    };

    let conrad1 = Npc {
        kind: NpcKind::Conrad,
        name: "Sir Conrad".to_string(),
        sprite: asset_server.load("sprites/sir_conrad.png"),
        sprite_path: "sprites/knight.png".to_string(),
        talking_anims: vec![
            asset_server.load("sprites/sir_conrad_talk_1.png"),
            asset_server.load("sprites/sir_conrad_talk_2.png"),
        ],
        talking_index: 0,
    };

    let pumpkinhead = Npc {
        kind: NpcKind::Pumkinhead,
        name: "Pumpkinhead".to_string(),
        sprite: asset_server.load("sprites/pumpkinhead.png"),
        sprite_path: "sprites/knight.png".to_string(),
        talking_anims: vec![
            asset_server.load("sprites/pumpkinhead_talk_1.png"),
            asset_server.load("sprites/pumpkinhead_talk_2.png"),
        ],
        talking_index: 0,
    };

    let conrad2 = Npc {
        kind: NpcKind::Conrad,
        name: "Sir Conrad".to_string(),
        sprite: asset_server.load("sprites/sir_conrad.png"),
        sprite_path: "sprites/knight.png".to_string(),
        talking_anims: vec![
            asset_server.load("sprites/sir_conrad_talk_1.png"),
            asset_server.load("sprites/sir_conrad_talk_2.png"),
        ],
        talking_index: 0,
    };

    let gordon = Npc {
        kind: NpcKind::Gordon,
        name: "Sir Conrad".to_string(),
        sprite: asset_server.load("sprites/gordon.png"),
        sprite_path: "sprites/knight.png".to_string(),
        talking_anims: vec![
            asset_server.load("sprites/gordon_talk_1.png"),
            asset_server.load("sprites/gordon_talk_2.png"),
        ],
        talking_index: 0,
    };

    game.npc_data.npcs.push(squee);
    game.npc_data.npcs.push(conrad1);
    game.npc_data.npcs.push(pumpkinhead);
    game.npc_data.npcs.push(conrad2);
    game.npc_data.npcs.push(gordon);

    // NPC Sprite
    let npc_sprite = commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(28. * 8., 38. * 8.)),
            ..default()
        },
        transform: Transform::from_xyz(384., 136., NPC_LEVEL),
        texture: asset_server.load("sprites/squee.png"),
        ..default()
    })
        .insert(NpcSprite)
        .insert(Name::new("NpcSprite"))
        .id();

    // NPC Text Box
    let font = asset_server.load("fonts/pixel_font.ttf");
    // todo: change
    let text_style = TextStyle {
        font,
        font_size: 20.,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Left,
    };

    let npc_text_box = commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("", text_style).with_alignment(text_alignment),
        transform: Transform::from_xyz(206.5, 280., NPC_LEVEL),
        text_2d_bounds: Text2dBounds {
            size: Vec2::new(400., 4000.)
        },
        ..default()
    })
        .insert(NpcText)
        .insert(Name::new("Npc Text")).id();

    game.npc_data.npc_sprite = Some(npc_sprite);
    game.npc_data.npc_dialog_box = Some(npc_text_box);
}

//==================================================================================================
//                  NPC Data
//==================================================================================================


pub struct NPCData {
    npcs : Vec<Npc>,
    current_npc : usize,
    npc_dialog_box : Option<Entity>,
    npc_sprite : Option<Entity>,
}

impl Default for NPCData {
    fn default() -> Self {
        NPCData {
            npcs : Vec::new(),
            current_npc: 0,
            npc_dialog_box : None,
            npc_sprite : None
        }
    }
}

impl NPCData {

    pub fn get_current_npc(&self) -> Option<&Npc> {
        self.npcs.get(self.current_npc)
    }

    pub fn get_current_npc_mut(&mut self) -> Option<&mut Npc> {
        self.npcs.get_mut(self.current_npc)
    }

    pub fn say(&self, commands : &mut Commands, message : &str) {
        if let Some(text_box) = self.npc_dialog_box {
            commands.entity(text_box).insert(Say::new(message));
        };
    }

    pub fn spawn_next_npc(&mut self) {
        self.current_npc += 1;
    }
}

//==================================================================================================
// Everything dialogue related below, inspired from @Inspirateur's 'Undoing' dialogue system in charter.rs
// link: https://github.com/Inspirateur/Undoing/blob/main/src/character.rs
//==================================================================================================

#[derive(Component)]
pub struct Say {
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

fn dialogue(
    mut commands: Commands,
    mut query_text: Query<(Entity, &mut Text, &mut Say), With<NpcText>>,
    mut query_sprite: Query<(&mut Handle<Image>, &mut Sprite),  With<NpcSprite>>,
    time: Res<Time>,
    mut game : ResMut<GameManager>
    // audio: Res<Audio>
) {
    let mut npc = game.npc_data.get_current_npc_mut();

    if let Some(npc) = npc {
        if let Ok((entity, mut text, mut say)) = query_text.get_single_mut() {
            if let Ok((mut sprite_handle, mut sprite)) = query_sprite.get_single_mut() {
                if say.i == 0 {
                    say.start = time.seconds_since_startup();
                    // change sprite picture
                    *sprite_handle = npc.sprite.clone();
                    // change sprite scaling
                    let change_sprite_size = |width: f32, height: f32, mut sprite: &mut Sprite| {
                        sprite.custom_size = Some(Vec2::new(28. * 8., 38. * 8.,));
                    };
                }

                // compute the new i
                let now = time.seconds_since_startup();
                let mut new_i = say.compute_i(now);
                // if we finished
                if say.i >= say.text.len() {
                    // and 1 sec has passed

                    if now - say.duration - say.start > 1. {
                        commands.entity(entity).remove::<Say>();

                        // change sprite back to default sprite
                        *sprite_handle = npc.sprite.clone();
                    }
                }
                // if not finished
                else if new_i != say.i {
                    // there's new characters to say
                    new_i = new_i.min(say.text.len());

                    // sprite talking animation
                    if new_i % 6 == 0 {
                        let frame = npc.talk_frame();
                        *sprite_handle = npc.talking_anims[frame].clone();
                    }

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

pub struct NpcClickEvent;

fn click_npc(
    game_helper: Res<GameHelper>,
    mut writer: EventWriter<NpcClickEvent>,
    mut lines : ResMut<DebugLines>,
    mut query: Query<(&GlobalTransform, &Sprite), With<NpcSprite>>,
    mouse : Res<Input<MouseButton>>,
) {
    if let Ok((transform, sprite)) = query.get_single_mut() {
        let rect = Slot::generate_rect(transform, sprite);

        rect.draw_rect(&mut lines, Color::RED);
        if rect.is_within(game_helper.mouse_world_pos()) && mouse.just_pressed(MouseButton::Left) {
            writer.send(NpcClickEvent);
        };
    }
}