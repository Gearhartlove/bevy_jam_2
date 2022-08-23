use std::collections::linked_list::IntoIter;
use std::collections::LinkedList;
use bevy::prelude::*;
use crate::AppState;
use crate::element::Element;
use crate::npc::NpcPlugin;

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        let quests = setup_quests();
        let current_quest: Quest = Quest::GLACIER_ICE_QUEST;
        app
            .insert_resource(quests)
            .insert_resource(current_quest)
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(advance_current_quest))
            // .add_system_to_stage(CoreStage::PostUpdate, is_quest_complete);
        ;
    }
}

#[derive(Debug, Clone)]
pub struct Quest<'r> {
    text: &'static str,
    result: Element,
    reward: Option<&'r[Element]>,
    crafting_table: Option<CraftingTable>,
    pub npc: &'static str,
}

#[derive(Debug, Clone)]
enum CraftingTable {
    Furnace,
    Mixer,
    Slicer,
}

impl<'r> Quest<'r> {
    // #####################################################################
    // New quest chapter
    // #####################################################################
    // Note default unlocked ingredients: Frost Bottle, Yeti Water

    pub const GLACIER_ICE_QUEST: Quest<'r> = {
        Quest::new(
            Element::GLACIER_ICE, // Result
            None,  // Reward
            Some(CraftingTable::Slicer), // Crafting Table Reward
            "I need some glacier ice, can you get me some?",// Quest Text
            NpcPlugin::GOBLIN_NPC, // npc
        )
    };

    pub const SHAVED_ICE_QUEST: Quest<'r> = {
        Quest::new(
            Element::SHAVED_ICE, // Result
            Some(&[Element::LEGEND_DAIRY]),  // Reward
            Some(CraftingTable::Mixer), // Crafting Table Reward
            "I need some shaved ice. Get me some :)",// Quest Text
            NpcPlugin::GOBLIN_NPC, // npc
        )
    };

    pub const UTTER_ICE_CREAM_QUEST: Quest<'r> = {
        Quest::new(
            Element::UTTER_ICE_CREAM, // Result
            Some(&[Element::GRIFFON_EGGS]),  // Reward
            None, // Crafting Table Reward
            "MAKE ME ICE CREAM!",// Quest Text
            NpcPlugin::GOBLIN_NPC, // npc
        )
    };
    // #####################################################################
    // New quest chapter
    // #####################################################################

    const fn new(result: Element, reward: Option<&'r[Element]>, crafting_table: Option<CraftingTable>, text: &'static str, npc: &'static str) -> Self {
        Self {
            result,
            reward,
            crafting_table,
            text,
            npc
        }
    }
}

fn setup_quests() -> IntoIter<Quest<'static>> {
    let mut ll: LinkedList<Quest> = LinkedList::new();
    //ll.push_back(Quest::GLACIER_ICE_QUEST); // already started in the 'build' function
    ll.push_back(Quest::SHAVED_ICE_QUEST);
    ll.push_back(Quest::UTTER_ICE_CREAM_QUEST);

    return ll.into_iter();
}

pub fn advance_current_quest(mut current_quest: ResMut<Quest<'static>>, mut quest_iter: ResMut<IntoIter<Quest<'static>>>) {
    *current_quest = quest_iter.next().unwrap();
    println!("{:?}", *current_quest)
}

// temporary; todo: change to brook's event name
struct CraftingEvent;

fn is_quest_complete(mut crafting_events: ResMut<Events<CraftingEvent>>) -> bool {
    // return self.result == *created_ingredient;
    unimplemented!()
}