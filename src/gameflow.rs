use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy_rapier2d::na::SliceRange;
use crate::element::Element;
use crate::game::{GameManager, GameStatus};
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
        for event in on_item_craft {
            current.on_item_crafted()
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
            .add_segment(SqueeHelloPlayer::default())
            .add_segment(SqueeTutorialCrafting::default());

        return game_flow;
    }
}

// ########################################################################

trait Segment {
    fn is_complete(&self) -> bool;

    fn on_item_crafted(
        &self,
        mut commands: &mut Commands,
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
        mut commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        mut game: &mut ResMut<GameManager>,
    );

    fn on_segment_end(&self) {}
}

// ################################################################################################################################################
// SqueeHelloPlayer
// ################################################################################################################################################
struct SqueeHelloPlayer {
    phrases: Vec<String>,
    phrase_index: usize,
}

impl SqueeHelloPlayer {
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

impl Default for SqueeHelloPlayer {
    fn default() -> Self {
        SqueeHelloPlayer {
            phrases: vec![
                "Yo wassup!".to_string(),
                "Nice Job clicking me!".to_string(),
                "You are Awesome!".to_string(),
                "You are dope!".to_string(),
                "You are MOGIS!".to_string(),
            ],
            phrase_index: 0,
        }
    }
}

impl Segment for SqueeHelloPlayer {
    fn is_complete(&self) -> bool {
        self.phrase_index == (self.phrases.len() + 1)
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

// ################################################################################################################################################
// SqueeCraftingTutorial
// ################################################################################################################################################
struct SqueeTutorialCrafting {

}

impl Default for SqueeTutorialCrafting {
    fn default() -> Self {
        todo!()
    }
}

impl Segment for SqueeTutorialCrafting {
    fn is_complete(&self) -> bool {
        todo!()
    }

    fn on_item_crafted(&self, commands: &mut Commands, element: Element) {

    }

    fn on_segment_start(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>) {
        todo!()
    }
}