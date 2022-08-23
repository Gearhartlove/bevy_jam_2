use std::collections::linked_list::IntoIter;
use bevy::prelude::*;
use crate::element::Element;
use crate::npc::{Npc, NpcKind, Say};
use crate::quest::Quest;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Game>()
            .add_startup_system(create_npcs)
            .add_startup_system(give_next_quest);
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
            NpcKind::Squee => {i = 0;}
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
                // voice: asset_server.load("voice/goblin_voice.png"),
            }
        )
        .insert(Name::new("Squee Entity"))
        .id();

    game.npcs.push(squee_entity);
}

pub fn give_next_quest(mut commands: Commands, mut game: ResMut<Game>, mut quest_iter: ResMut<IntoIter<Quest<'static>>>, mut current_quest: ResMut<Quest<'static>>) {
    if game.status == GameStatus::QuestComplete {
        match game.npc {
            NpcKind::Squee => {
                println!("true");
                let squee = game.get_npc();
                // respond differently depending on the quest
                if current_quest.result == Element::UTTER_ICE_CREAM {
                    commands.entity(squee).insert(Say::new(
                        "Squeeeee neeeeeds ice creeeeem! \n ... \n please."
                    ));
                } else {
                    commands.entity(squee).insert(Say::new(
                        "I don't have a response for the current quest."
                    ));
                }
                // add more quests below ...
            }
            _ => {}
        }

        // change game status
        game.status == GameStatus::QuestInProgress;

        // update next quest
        *current_quest = quest_iter.next().unwrap();
        println!("{:?}", *current_quest)
    }
}