use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy::utils::HashMap;
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
            .add_startup_system(start_gameflow)
            .add_system_to_stage(CoreStage::PostUpdate, update_gameflow);
    }
}

//==================================================================================================
//                  GameFlow
//==================================================================================================

pub struct Gameflow {
    segments: Vec<Box<dyn Segment + Send + Sync>>,
    current: i32,
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
    pub insert_element_event : Option<InsertElementEvent>
}

impl Default for EventCaller {
    fn default() -> Self {
        EventCaller {
            insert_element_event : None
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
) {
    let i = gameflow.current as usize;
    let mut current = gameflow.segments.get_mut(i).unwrap();
    let mut event_caller = EventCaller::default();

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

    if let Some(event) = event_caller.insert_element_event {
        insert_element_event.send(event)
    }
}

impl Default for Gameflow {
    fn default() -> Self {
        let mut game_flow = Gameflow {
            segments: vec![],
            current: 0,
        };

        game_flow
            // chapter 1
            .add_segment(NpcDialogueSegment::new(
                vec![
                    "one".to_string(),
                    "two".to_string(),
                    "three".to_string(),
                ]
            ))

            .add_segment(CraftingSegment::new(Element::UTTER_ICE_CREAM.clone())
                .with_hint("Hint 1")
                .with_hint("Hint 2")
                .with_hint("Hint 3")
                .with_comment(&Element::SHAVED_ICE, "How Cold!"))

            .add_segment(NpcDialogueSegment::new(
                vec![
                    "Good Job! Now on to the next thing...".to_string()
                ]
            ))

            .add_segment(NpcDialogueSegment::new(
                vec![
                    "Good Job! Now on to the next thing...".to_string()
                ]
            ));;

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
    phrases: Vec<String>,
    phrase_index: usize,
}

impl NpcDialogueSegment {
    pub fn new(vec: Vec<String>) -> Self {
        Self {
            phrases: vec,
            phrase_index: 0,
        }
    }

    pub fn get_next_phrase(&mut self) -> String {
        self.phrase_index += 1;
        println!("index: {}", self.phrase_index);
        let index = self.phrase_index;
        if let Some(s) = self.phrases.get(index) {
            return s.clone();
        } else {
            return String::from("Out of bounds indexing npc phrases.");
        }
    }
}

impl Segment for NpcDialogueSegment {
    fn is_complete(&self) -> bool {
        self.phrase_index + 1 == self.phrases.len()
    }

    fn on_npc_click(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller
    ) {
        let text = self.get_next_phrase();
        game.npc_data.say(commands, text.as_str())
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
            game.npc_data.say(commands, comment.as_str())
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
        game.npc_data.say(commands, text.as_str());
        self.current_hint += 1;
    }
}

//==================================================================================================
//                    Crafting Segment
//==================================================================================================