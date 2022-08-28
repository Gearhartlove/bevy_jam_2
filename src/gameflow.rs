use std::collections::VecDeque;
use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy::utils::HashMap;
use bevy::utils::tracing::event;
use crate::audio::SayEvent;
use crate::element::Element;
use crate::game::GameManager;
use crate::npc::{Npc, NpcClickEvent, NpcKind, NpcSprite, NpcText, Say};
use crate::ui::{ElementCraftedEvent, InsertElementEvent, NPC_LEVEL};

pub struct GameflowPlugin;

impl Plugin for GameflowPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Gameflow>()
            .init_resource::<GameManager>()
            //.add_startup_system(start_gameflow)
            .add_system_to_stage(CoreStage::PostUpdate, update_gameflow);
    }
}

//==================================================================================================
//                  GameFlow
//==================================================================================================

pub struct Gameflow {
    segments: Vec<Box<dyn Segment + Send + Sync>>,
    current: u32,
    last : u32
}

impl Gameflow {
    pub fn add_segment<T>(&mut self, segment: T) -> &mut Self where T: Segment + Send + Sync + 'static {
        self.segments.push(Box::new(segment));
        return self;
    }

    pub fn advance(&mut self) {
        self.current += 1;
    }
}

pub struct EventCaller {
    pub insert_element_event : Option<InsertElementEvent>,
    pub say_event: Option<SayEvent>,
}

impl Default for EventCaller {
    fn default() -> Self {
        EventCaller {
            insert_element_event : None,
            say_event: None,
        }
    }
}

fn start_gameflow(
    mut gameflow: ResMut<Gameflow>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<GameManager>,
) {
    let i = gameflow.current as usize;
    let current = gameflow.segments.get_mut(i).unwrap();
    current.on_segment_start(&mut commands, &asset_server, &mut game, &mut EventCaller::default());
}

fn update_gameflow(
    mut gameflow: ResMut<Gameflow>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<GameManager>,

    //Events Listeners
    mut on_npc_click: EventReader<NpcClickEvent>,
    mut on_item_craft: EventReader<ElementCraftedEvent>,

    //Event Writers
    mut insert_element_event : EventWriter<InsertElementEvent>,
    mut say_event_writer : EventWriter<SayEvent>,
) {

    //println!("{} | {}", gameflow.current, gameflow.segments.len());
    let mut event_caller = EventCaller::default();

    let mut should_init = false;

    if gameflow.current != gameflow.last {
        gameflow.last = gameflow.current;
        should_init = true;
    }

    let i = gameflow.current as usize;
    if let Some(mut current) = gameflow.segments.get_mut(i) {
        if should_init {
            current.on_segment_start(&mut commands, &asset_server, &mut game, &mut event_caller)
        }

        for event in on_npc_click.iter() {
            current.on_npc_click(&mut commands, &asset_server, &mut game, &mut event_caller);
        }

        for event in on_item_craft.iter() {
            current.on_item_crafted(&mut commands, &asset_server, &mut game, &mut event_caller, event.0.clone());
        }

        if current.is_complete() {
            current.on_segment_end(&mut commands, &asset_server, &mut game, &mut event_caller);
            gameflow.advance();
        }
    }

    if let Some(event) = event_caller.insert_element_event {
        insert_element_event.send(event)
    }
    if let Some(event) = event_caller.say_event {
        say_event_writer.send(event)
    }
}

impl Default for Gameflow {
    fn default() -> Self {
        let mut game_flow = Gameflow {
            segments: vec![],
            current: 0,
            last: u32::MAX
        };

        game_flow
            // chapter 1
            .add_segment(NpcDialogueSegment::new()
                .add_line("THis is the beginning of the game, welcome!")
                .add_line("Tes Lorem Lorem Lorem Lorem  Lorem t2")
                .add_line("Test3")
            )

            .add_segment(TransitionSegment::new(
                // Exiting Phrases
                vec![
                    "I'm leaving now".to_string(),
                ],
                // Entering Phrases
                vec![
                    "Well Hello There! I am in need of a sandwitch, can you help me out?".to_string(),
                ],
            ))

            .add_segment(CraftingSegment::new(Element::UTTER_ICE_CREAM.clone())
                .with_hint("Hint 1")
                .with_hint("Hint 2")
                .with_hint("Hint 3")
                .with_comment(&Element::SHAVED_ICE, "How Cold!")
            )

            .add_segment(NpcDialogueSegment::new()
                .add_line("Test1")
                .add_line("Test2")
                .add_line("Test3")
            )
        ;

        return game_flow;
    }
}

// ########################################################################

trait Segment {
    fn is_complete(&self) -> bool;

    fn on_item_crafted(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller,
        element : Element
    ) {}

    fn on_npc_click(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller
    ) {}

    fn on_npc_drop(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller,
        element : Element
    ) {}

    fn on_segment_start(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller
    ) {}

    fn on_segment_end(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller
    ) {}
}

// ################################################################################################################################################
// SqueeHelloPlayer
// ################################################################################################################################################

struct NpcDialogueSegment {
    phrases: VecDeque<String>,
}

impl NpcDialogueSegment {
    pub fn new() -> Self {
        Self {
            phrases: VecDeque::new(),
        }
    }

    pub fn do_next_phrase(&mut self, commands : &mut Commands, game : &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        let line = self.phrases.pop_front();
        if let Some(line) = line {
            let duration = game.npc_data.say(commands, line.as_str());
            event_caller.say_event = Some(SayEvent(duration));
        };
    }

    pub fn add_line(mut self, line : &str) -> Self {
        self.phrases.push_back(line.to_string());
        self
    }
}

impl Segment for NpcDialogueSegment {
    fn is_complete(&self) -> bool {
        self.phrases.is_empty()
    }

    fn on_npc_click(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller
    ) {
        self.do_next_phrase(commands, game, event_caller);
    }

    fn on_segment_start(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        self.do_next_phrase(commands, game, event_caller);
    }
}

//==================================================================================================
//                    Crafting Segment
//==================================================================================================

pub struct CraftingSegment {
    goal : Element,
    hints : Vec<String>,
    comments : HashMap<Element, String>,
    is_thing_crafted : bool,
    current_hint : usize
}

impl CraftingSegment {

    pub fn new(element : Element) -> Self {
        CraftingSegment {
            goal : element,
            hints : Vec::new(),
            comments : HashMap::new(),
            is_thing_crafted : false,
            current_hint : 0
        }
    }

    pub fn with_hint(mut self, hint : &str) -> CraftingSegment {
        self.hints.push(hint.to_string());
        self
    }

    pub fn with_comment(mut self, element : &'static Element, comment : &str) -> CraftingSegment {
        self.comments.insert(element.clone(), comment.to_string());
        self
    }
}

impl Segment for CraftingSegment {
    fn is_complete(&self) -> bool {
        self.is_thing_crafted
    }

    fn on_item_crafted (
        &mut self, commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller,
        element: Element
    ) {
        if element == self.goal {
            self.is_thing_crafted = true;
        }
        if let Some(comment) = self.comments.get(&element) {
            let duration = game.npc_data.say(commands, comment.as_str());
            event_caller.say_event = Some(SayEvent(duration));
        }
    }

    fn on_npc_click(
        &mut self, commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller
    ) {
        println!("On Click");
        if self.current_hint >= self.hints.len() {
            self.current_hint = 0
        }
        let text = self.hints.get(self.current_hint).unwrap();
        let duration = game.npc_data.say(commands, text.as_str());
        event_caller.say_event = Some(SayEvent(duration));
        self.current_hint += 1;
    }
}

//==================================================================================================
//                    Transition Segment
//==================================================================================================

pub struct TransitionSegment {
    leaving_phrases: Vec<String>,
    entering_phrases: Vec<String>,
    leaving_index: i32,
    entering_index: i32,
}

impl TransitionSegment {
    fn new(
        leaving_phrases: Vec<String>,
        entering_phrases: Vec<String>,
    ) -> Self {
        Self {
            leaving_phrases,
            entering_phrases,
            leaving_index: -1,
            entering_index: -1,
        }
    }

    pub fn is_old_npc_done(&self) -> bool {
        self.leaving_index >= (self.leaving_phrases.len() as i32) - 1
    }

    pub fn is_new_npc_done(&self) -> bool {
        self.entering_index >= (self.entering_phrases.len() as i32) - 1
    }

    pub fn get_next_phrase(&mut self) -> String {
        let get_phrase = |index: &mut i32, phrases: &Vec<String>| -> String {
            *index += 1;
            let i = *index as usize;
            return if let Some(s) = phrases.get(i as usize) {
                s.clone()
            } else {
                "Index Out of bounds".to_string()
            };
        };

        if self.is_old_npc_done() {
            get_phrase(&mut self.entering_index, &self.entering_phrases)
        } else {
            get_phrase(&mut self.leaving_index, &self.leaving_phrases)
        }
    }
}

impl Segment for TransitionSegment {
    fn is_complete(&self) -> bool {
        self.is_old_npc_done() && self.is_new_npc_done()
    }

    fn on_npc_click(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        if self.is_new_npc_done() {
            return;
        }
        if self.is_old_npc_done() && self.entering_index == -1 { // -1 because the 0 index of the dialogue Vec has not been said
            game.npc_data.spawn_next_npc()
        }
        let phrase = self.get_next_phrase();
        let duration = game.npc_data.say(commands, phrase.as_str());
        event_caller.say_event = Some(SayEvent(duration));
    }
}