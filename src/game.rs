use std::collections::linked_list::IntoIter;
use bevy::prelude::*;
use crate::element::Element;
use crate::game::GameStatus::QuestComplete;
use crate::npc::{Npc, NpcKind, Say};
use crate::quest::Quest;
use crate::ui::ElementCraftedEvent;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Game>()
            .add_startup_system(create_npcs)
            .add_system_to_stage(CoreStage::PostUpdate, check_if_quest_completed)
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
    pub npc: NpcKind,
    pub status: GameStatus,

}

impl Default for Game {
    fn default() -> Self {
        Game {
            npcs: vec![],
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

fn create_npcs(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    let squee_entity = commands
        .spawn()
        .insert(
            Npc {
                kind: NpcKind::Squee,
                name: "Squee the Thumbless".to_string(),
                sprite: asset_server.load("sprites/goblin.png"),
                sprite_path: "sprites/goblin.png".to_string()
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
                sprite_path: "sprites/knight.png".to_string()
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
        *current_quest = quest_iter.next().unwrap();
        println!("\nCURRENT QUEST: {:?}", *current_quest);

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
                }
                else if current_quest.result == Element::GLACIER_ICE {
                    commands.entity(squee).insert(Say::new(
                        "Try using the furnace to make some \n\
                    will ya? I heard ice in the oven makes \n\
                    it real cold."
                    ));
                } else if current_quest.result == Element::SHAVED_ICE {
                    commands.entity(squee).insert(Say::new(
                        "I need Ice Ice ICE! Try cutting some of that \n\
                        glacier, will ya?"
                    ));
                } else if current_quest.result == Element::UTTER_ICE_CREAM {
                    commands.entity(squee).insert(Say::new(
                        "Squeeeee neeeeeds ice creeeeem! \n... \n\
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
    mut current_quest: Res<Quest<'static>>,
    mut game: ResMut<Game>,
    mut combine_event: EventReader<ElementCraftedEvent>
) {
    for combine in combine_event.iter() {
        println!("comparing: \n{:?} \n{:?}", combine.0, current_quest.result.clone());
        if combine.0 == current_quest.result {
            println!("changing game state");
            game.status = QuestComplete;
        }
    }
}