use std::collections::linked_list::IntoIter;
use bevy::prelude::*;
use crate::element::Element;
use crate::game::GameStatus::QuestComplete;
use crate::npc::{Npc, NpcKind, Say};
use crate::npc::NpcKind::Squee;
use crate::quest::{CraftingTable, Quest};
use crate::ui::{ElementCraftedEvent, InsertElementEvent, LoadMixerEvent, LoadSlicerEvent, RefreshSlotsEvent, UI_LEVEL, UiData};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Game>()
            .add_startup_system(create_npcs)
            .add_startup_system(setup_elements)
            .add_startup_system(setup_crafting_tables)
            .add_system_to_stage(CoreStage::PostUpdate, check_if_quest_completed)
            .add_system(make_visible)
            .add_system(give_next_quest);
    }
}

#[derive(PartialEq, Eq)]
pub enum GameStatus {
    QuestInProgress,
    QuestComplete,
    AllQuestsComplete,
}

pub struct Game {
    npcs: Vec<Entity>,
    pub pages: Vec<Entity>,
    pub slicer_ent: Option<Entity>,
    pub mixer_ent: Option<Entity>,
    pub furnace_ent: Option<Entity>,
    pub npc: NpcKind,
    pub status: GameStatus,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            npcs: vec![],
            pages: vec![],
            slicer_ent: None,
            mixer_ent: None,
            furnace_ent: None,
            npc: NpcKind::Squee,
            status: GameStatus::QuestComplete,
        }
    }
}

impl Game {
    pub fn get_npc(&self) -> Entity {
        let mut i = 0;
        match self.npc {
            NpcKind::Squee => { i = 0; }
            NpcKind::Conrad => { i = 1; }
        }
        self.npcs[i]
    }
}

#[derive(Component)]
struct MakeVisible;

fn make_visible(
    mut query: Query<(Entity, &mut Visibility), With<MakeVisible>>,
    mut commands: Commands,
) {
    for (entity, mut visability) in &mut query {
        visability.is_visible = true;
        commands.entity(entity).remove::<MakeVisible>();
    }
}

fn setup_crafting_tables(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
) {
    let slicer = commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(16. * 8.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 264., UI_LEVEL),
        texture: asset_server.load("sprites/slicer.png"),
        visibility: Visibility {
            is_visible: false
        },
        ..default()
    })
        .insert(Name::new("Slicer"))
        .id();

    let furnace = commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(16. * 8., 32. * 8.)),
            ..default()
        },
        transform: Transform::from_xyz(0., -152., UI_LEVEL),
        texture: asset_server.load("sprites/furnace.png"),
        // visibility: Default::default(),
        ..default()
    })
        .insert(Name::new("Furnace"))
        .id();

    let mixer = commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(32. * 8., 16. * 8.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 88., UI_LEVEL),
        texture: asset_server.load("sprites/mixer.png"),
        visibility: Visibility {
            is_visible: false
        },
        ..default()
    })
        .insert(Name::new("Mixer"))
        .id();

    game.furnace_ent = Some(furnace);
    game.mixer_ent = Some(mixer);
    game.slicer_ent = Some(slicer);
}

fn setup_elements(
    mut ui_data: ResMut<UiData>,
    mut slot_refresh: EventWriter<RefreshSlotsEvent>,
) {
    ui_data.unsafe_add(Element::YETI_WATER.clone());
    ui_data.unsafe_add(Element::FROZEN_DRAGON_SCALE.clone());

    slot_refresh.send(RefreshSlotsEvent)
}

fn create_npcs(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    let squee_entity = commands
        .spawn()
        .insert(
            Npc {
                kind: NpcKind::Squee,
                name: "Squee the Thumbless".to_string(),
                sprite: asset_server.load("sprites/squee.png"),
                sprite_path: "sprites/squee.png".to_string(),
                talking_anims: vec![
                    asset_server.load("sprites/squee_talk1.png"),
                    asset_server.load("sprites/squee_talk2.png"),
                ],
                talking_index: 0,
                // voice: asset_server.load("voice/goblin_voice.png"),
            }
        )
        .insert(Name::new("Squee Entity"))
        .id();

    let sir_conrad = commands
        .spawn()
        .insert(
            Npc {
                kind: NpcKind::Conrad,
                name: "Sir Conrad".to_string(),
                sprite: asset_server.load("sprites/knight.png"),
                sprite_path: "sprites/knight.png".to_string(),
                talking_anims: vec![

                ],
                talking_index: 0,
            }
        )
        .insert(Name::new("Sir Conrad Entity"))
        .id();

    game.npcs.push(squee_entity);
    game.npcs.push(sir_conrad);
}

pub fn give_next_quest(mut commands: Commands, mut game: ResMut<Game>, mut quest_iter: ResMut<IntoIter<Quest<'static>>>, mut current_quest: ResMut<Quest<'static>>) {
    if game.status == GameStatus::QuestComplete {

        // change game status
        game.status = GameStatus::QuestInProgress;

        // update next quest
        if let Some(q) = quest_iter.next() {
            *current_quest = q;
        }
        //println!("\nNEW QUEST: {:?}", *current_quest);

        match game.npc {
            NpcKind::Squee => {
                let squee = game.get_npc();
                // respond differently depending on the quest
                // DEBUG
                if current_quest.result == Element::LEGEND_DAIRY {
                    commands.entity(squee).insert(Say::new(
                        "Debug Text. Combine the two elements,\n\
                        thanks xD"
                    ));
                } else if current_quest.result == Element::GLACIER_ICE {
                    commands.entity(squee).insert(Say::new(
                        "Try using the furnace to make some ice will ya? I heard ice in the oven makes it real cold."
                    ));
                } else if current_quest.result == Element::SHAVED_ICE {
                    commands.entity(squee).insert(Say::new(
                        "I need Ice Ice ICE! Try cutting some of that glacier, will ya?"
                    ));
                } else if current_quest.result == Element::UTTER_ICE_CREAM {
                    commands.entity(squee).insert(Say::new(
                        "Squeeeee neeeeeds ice creeeeem! \n...\n\
                        Try mixing some of those ingredients up!."
                    ));
                } else {
                    commands.entity(squee).insert(Say::new(
                        "I don't have a response for the current quest."
                    ));
                }
                // add more quests below ...
            }
            NpcKind::Conrad => {
                let conrad = game.get_npc();
                if current_quest.result == Element::GLACIER_ICE {
                    commands.entity(conrad).insert(Say::new(
                        "The king needs ice, fast!"
                    ));
                } else {
                    commands.entity(conrad).insert(Say::new(
                        "I don't have a response for the current quest."
                    ));
                }
            }
        }
    }
}

fn check_if_quest_completed(
    mut commands: Commands,
    mut current_quest: Res<Quest<'static>>,
    mut game: ResMut<Game>,
    mut combine_event: EventReader<ElementCraftedEvent>,
    mut mixer_unlock: EventWriter<LoadMixerEvent>,
    mut slicer_unlock: EventWriter<LoadSlicerEvent>,
    mut reward_writer: EventWriter<InsertElementEvent>,
) {
    for combination in combine_event.iter() {
        // if quest is complete
        if combination.0 == current_quest.result {
            // unlock npc
            let npc = current_quest.npc.clone();
            match npc {
                NpcKind::Squee => {
                    game.npc = NpcKind::Squee;
                    println!("change npc to squee");
                }
                NpcKind::Conrad => {
                    game.npc = NpcKind::Conrad;
                    println!("change npc to conrad");
                }
            }
            // unlock crafting table
            if let Some(craft) = current_quest.crafting_table.clone() {
                match craft {
                    CraftingTable::Mixer => {
                        mixer_unlock.send(LoadMixerEvent);
                        commands.entity(game.mixer_ent.unwrap()).insert(MakeVisible);
                    }
                    CraftingTable::Slicer => {
                        slicer_unlock.send(LoadSlicerEvent);
                        commands.entity(game.slicer_ent.unwrap()).insert(MakeVisible);
                    }
                    CraftingTable::Furnace => {
                        // default unlocked
                    }
                }
            }

            // rewards
            if let Some(rewards) = current_quest.rewards {
                for r in rewards.iter() {
                    reward_writer.send(InsertElementEvent(r.clone()));
                }
            }

            game.status = QuestComplete;
        }
    }
}