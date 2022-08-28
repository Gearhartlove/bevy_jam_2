use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy::utils::HashMap;
use crate::element::Element;
use crate::game::GameManager;
use crate::npc::{Npc, NpcClickEvent, NpcKind, NpcSprite, NpcText, Say};
use crate::ui::{ElementCraftedEvent, NPC_LEVEL};

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

fn start_gameflow(
    mut gameflow: ResMut<Gameflow>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<GameManager>,
) {
    let i = gameflow.current as usize;
    let current = gameflow.segments.get_mut(i).unwrap();
    current.on_segment_start(&mut commands, &asset_server, &mut game);
}

fn update_gameflow(
    mut gameflow: ResMut<Gameflow>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<GameManager>,
    mut on_npc_click: EventReader<NpcClickEvent>,
    mut on_item_craft: EventReader<ElementCraftedEvent>,
) {
    let i = gameflow.current as usize;
    let mut current = gameflow.segments.get_mut(i).unwrap();
    if current.is_complete() {
        current.on_segment_end();
        gameflow.advance();
        let i = gameflow.current as usize;
        let current = gameflow.segments.get_mut(i).unwrap();
        current.on_segment_start(&mut commands, &asset_server, &mut game);
    } else {
        // npc clicking
        for event in on_npc_click.iter() {
            current.on_npc_click(&game, &mut commands);
        }
        for event in on_item_craft.iter() {
            current.on_item_crafted(&mut commands, &mut game, event.0.clone());
        }
    }
}

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
        mut commands: &mut Commands,
        game: &mut ResMut<GameManager>,
        element: Element,
    ) {}

    fn on_npc_click(
        &mut self,
        game: &ResMut<GameManager>,
        mut commands: &mut Commands,
    ) {}

    fn on_npc_drop(&self) {}

    fn on_segment_start(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
    ) {}

    fn on_segment_end(&self) {}
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
        game: &ResMut<GameManager>,
        mut commands: &mut Commands,
    ) {
        let text = self.get_next_phrase();
        commands.entity(game.get_npc()).insert(Say::new(
            text
        ));
    }

    fn on_segment_start(
        &mut self,
        mut commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        mut game: &mut ResMut<GameManager>,
    ) {
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
                }
            )
            .insert(Name::new("Squeoooooe Entity"))
            .id();

        // npc
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(28. * 8., 38. * 8.)),
                ..default()
            },
            transform: Transform::from_xyz(384., 136., NPC_LEVEL),
            texture: asset_server.load("sprites/squee.png"),
            ..default()
        })
            .insert(NpcSprite)
            .insert(Name::new("NpcSprite"));

        // text bubble
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

        commands.spawn_bundle(Text2dBundle {
            text: Text::from_section("", text_style).with_alignment(text_alignment),
            transform: Transform::from_xyz(206.5, 280., NPC_LEVEL),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(400., 4000.)
            },
            ..default()
        })
            .insert(NpcText)
            .insert(Name::new("Npc Text"));

        game.npcs.push(squee_entity);

        // have npc say something on start
        let index = self.phrase_index;

        commands.entity(game.get_npc()).insert(Say::new(
            self.phrases[index].clone()
        ));
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

    fn on_item_crafted(&mut self, commands: &mut Commands, game : &mut ResMut<GameManager>, element: Element) {
        if element == self.goal {
            self.is_thing_crafted = true;
        }
        if let Some(comment) = self.comments.get(&element) {
            commands.entity(game.get_npc()).insert(Say::new(
                comment
            ));
        }
    }

    fn on_npc_click(&mut self, game: &ResMut<GameManager>, commands: &mut Commands) {
        println!("On Click");
        if self.current_hint >= self.hints.len() {
            self.current_hint = 0
        }
        let text = self.hints.get(self.current_hint).unwrap();
        commands.entity(game.get_npc()).insert(Say::new(
            text
        ));
        self.current_hint += 1;
    }

    fn on_segment_start(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>) {}
}

//==================================================================================================
//                    Crafting Segment
//==================================================================================================