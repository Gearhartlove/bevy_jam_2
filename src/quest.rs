use std::collections::linked_list::IntoIter;
use std::collections::LinkedList;
use bevy::prelude::*;
use crate::AppState;
use crate::element::Element;
use crate::npc::{Npc, NpcKind, NpcPlugin};

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        let quests = setup_quests();
        let current_quest: Quest = Quest::GLACIER_ICE_QUEST;
        app
            .insert_resource(quests)
            .insert_resource(current_quest)
        // .add_system_to_stage(CoreStage::PostUpdate, is_quest_complete);
        ;
    }
}

#[derive(Debug, Clone)]
pub struct Quest<'r> {
    pub result: Element,
    pub rewards: Option<&'r [Element]>,
    pub crafting_table: Option<CraftingTable>,
    pub npc: NpcKind,
}

#[derive(Debug, Clone)]
pub enum CraftingTable {
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
            NpcKind::Squee, // Npc
        )
    };

    pub const SHAVED_ICE_QUEST: Quest<'r> = {
        Quest::new(
            Element::SHAVED_ICE, // Result
            Some(&[Element::LEGEND_DAIRY]),  // Reward
            Some(CraftingTable::Mixer), // Crafting Table Reward
            NpcKind::Squee, // Npc
        )
    };

    pub const UTTER_ICE_CREAM_QUEST: Quest<'r> = {
        Quest::new(
            Element::UTTER_ICE_CREAM, // Result
            // Some(&[Element::GRIFFON_EGGS, Element::FIRE_PEPPER]),  // Reward
            None, // Reward
            None, // Crafting Table Reward
            NpcKind::Squee, // Npc
        )
    };
    // #####################################################################
    // New quest chapter
    // #####################################################################

    const fn new(
        result: Element,
        reward: Option<&'r [Element]>,
        crafting_table: Option<CraftingTable>,
        npc: NpcKind,
    ) -> Self {
        Self {
            result,
            rewards: reward,
            crafting_table,
            npc
        }
    }
}

fn setup_quests() -> IntoIter<Quest<'static>> {
    let mut ll: LinkedList<Quest> = LinkedList::new();
    ll.push_back(Quest::GLACIER_ICE_QUEST);
    ll.push_back(Quest::SHAVED_ICE_QUEST);
    ll.push_back(Quest::UTTER_ICE_CREAM_QUEST);

    return ll.into_iter();
}