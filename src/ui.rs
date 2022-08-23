use std::ops::Add;
use bevy::ecs::schedule::ShouldRun::No;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::math::{vec2, Vec3Swizzles};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::Texture;
use bevy::utils::tracing::event;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::Collider;
use crate::element::Element;
use crate::{GameHelper, MixerRecipeIden};
use crate::registry::Registry;

const TAVERN_LEVEL : f32 = 10.0;
const UI_LELVEL : f32 = 20.0;
const SLOT_LEVEL : f32 = 30.0;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DragInfo>()
            .add_event::<DropElementEvent>()
            .add_event::<UpdateSlotEvent>()
            .add_startup_system(add_slots)
            .add_system(render_slots)
            .add_system(render_dragging)
            .add_system(drag_item)
            .add_system(check_for_mixer_craft)
            .add_system(test_system)
            //.add_system(on_drop_element.after(drag_item))
            .add_system_to_stage(CoreStage::PostUpdate, handle_slot_events);
    }
}

//==================================================================================================
//                          Slot Events
//==================================================================================================

#[derive(Debug)]
pub struct UpdateSlotEvent(u32, Option<Element>);

#[derive(Debug)]
pub struct DropElementEvent(Vec2, Element);

pub fn handle_slot_events (
    mut slot_query : Query<(&mut Slot, &Transform, &Sprite)>,
    mut element_drop_event : EventReader<DropElementEvent>,
    mut update_slot_event : EventReader<UpdateSlotEvent>
) {
    for event in element_drop_event.iter() {
        for (mut slot, transform, sprite) in slot_query.iter_mut() {
            let rect = Slot::generate_rect(transform, sprite);
            if rect.is_within(event.0) && slot.can_change {
                slot.element = Some(event.1.clone())
            }
        }
    }

    for event in update_slot_event.iter() {
        for (mut slot, _, _) in slot_query.iter_mut() {
            if slot.index == event.0 {
                slot.element = event.1.clone();
            }
        };
    }
}



//==================================================================================================
//                          DragInfo Resource
//==================================================================================================

pub struct DragInfo {
    pub currently_dragging : Option<Element>,
    pub should_change_sprite : bool,
    pub sprite_size : f32
}

impl Default for DragInfo {
    fn default() -> Self {
        DragInfo {
            currently_dragging : None,
            should_change_sprite : false,
            sprite_size : 16.0
        }
    }
}

//==================================================================================================
//                          Rect
//==================================================================================================

pub struct Rect {
    pub x1 : f32,
    pub y1 : f32,
    pub x2 : f32,
    pub y2 : f32,
}

impl Rect {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub fn is_within(&self, point : Vec2) -> bool {
        self.x1 <= point.x && self.x2 >= point.x && self.y1 >= point.y && self.y2 <= point.y
    }

    pub fn draw_rect(&self, lines : &mut ResMut<DebugLines>, color : Color) {
        let point1 = Vec3::new(self.x1, self.y1, 0.0);
        let point2 = Vec3::new(self.x2, self.y1, 0.0);
        let point3 = Vec3::new(self.x1, self.y2, 0.0);
        let point4 = Vec3::new(self.x2, self.y2, 0.0);
        lines.line_colored(point1, point2, 0.0, color);
        lines.line_colored(point2, point4, 0.0, color);
        lines.line_colored(point4, point3, 0.0, color);
        lines.line_colored(point3, point1, 0.0, color);
    }
}

//==================================================================================================
//                          Slot
//==================================================================================================

#[derive(Component)]
pub struct DragEntity;

#[derive(Component)]
pub struct MixerSlot1;

#[derive(Component)]
pub struct MixerSlot2;

#[derive(Component)]
pub struct FurnaceSlot1;

#[derive(Component)]
pub struct FurnaceSlot2;

#[derive(Component)]
pub struct SlicerSlot;

#[derive(Component)]
pub struct Slot{
    pub element : Option<Element>,
    pub index : u32,
    pub can_change : bool
}

impl Slot {
    pub fn with_index(index : u32) -> Slot {
        Slot {
            index,
            ..default()
        }
    }
}

impl Default for Slot {
    fn default() -> Self {
        Slot {
            index : 0,
            element: None,
            can_change : false
        }
    }
}

impl Slot {
    pub fn generate_rect(transform : &Transform, sprite : &Sprite) -> Rect {
        if let Some(size) = sprite.custom_size {
            Rect::new(
                transform.translation.x - size.x/2.0,
                transform.translation.y + size.y/2.0,
                transform.translation.x + size.x/2.0,
                transform.translation.y - size.y/2.0,
            )
        } else {
            Rect::new(
                transform.translation.x - 8.0,
                transform.translation.y + 8.0,
                transform.translation.x + 8.0,
                transform.translation.y - 8.0,
            )
        }
    }
}

fn test_system (
    mut slot_update : EventWriter<UpdateSlotEvent>,
    keys: Res<Input<KeyCode>>
) {
    if keys.just_pressed(KeyCode::A) {
        slot_update.send(UpdateSlotEvent(0, Some(Element::FIRE_PEPPER)));
        slot_update.send(UpdateSlotEvent(1, Some(Element::YETI_WATER)));
    }
}

fn check_for_mixer_craft(
    mut slot_1_q : Query<&mut Slot, (With<MixerSlot1>, Without<MixerSlot2>)>,
    mut slot_2_q : Query<&mut Slot, (With<MixerSlot2>, Without<MixerSlot1>)>,
    registy : Res<Registry>,
    mut slot_update : EventWriter<UpdateSlotEvent>
) {
    let mut slot_1 = slot_1_q.single_mut();
    let mut slot_2 = slot_2_q.single_mut();

    if slot_1.element.is_some() && slot_2.element.is_some() {
        let element_1 = slot_1.element.as_ref().unwrap().clone();
        let element_2 = slot_2.element.as_ref().unwrap().clone();

        let iden = MixerRecipeIden::new(element_1, element_2);

        let recipe = registy.mixer_recipe_registry.get(&iden);
        if recipe.is_some() {
            slot_update.send(UpdateSlotEvent(10, Some(recipe.as_ref().unwrap().result.clone())))
        } else {

        }

        slot_1.element = None;
        slot_2.element = None;
    }
}

fn render_slots(
    mut slot : Query<(&Slot, &mut Handle<Image>, &mut Visibility)>,
    asset_server: Res<AssetServer>
) {
    for (s, mut handle, mut visibility) in slot.iter_mut() {
        if s.element.is_some() {
            visibility.is_visible = true;
            let sprite = asset_server.load(s.element.as_ref().unwrap().sprite_file_path().as_str());
            *handle = sprite;
        } else {
            visibility.is_visible = false;
        }
    }
}

fn render_dragging (
    mut drag_entity : Query<(&mut Transform, &mut Handle<Image>, &mut Visibility), With<DragEntity>>,
    mut drag_info : ResMut<DragInfo>,
    game_helper : Res<GameHelper>,
    asset_server: Res<AssetServer>
) {
    let (mut transform, mut handle, mut visibility) = drag_entity.single_mut();

    if drag_info.currently_dragging.is_some() {
        if drag_info.should_change_sprite {
            *handle = asset_server.load(drag_info.currently_dragging.as_ref().unwrap().sprite_file_path().as_str());
            drag_info.should_change_sprite = false;
        }

        visibility.is_visible = true;
        transform.translation = game_helper.mouse_world_pos().extend(SLOT_LEVEL + 1.0);
    } else {
        visibility.is_visible = false;
    }
}

fn drag_item(
    mut slot_query : Query<(&mut Slot, &Transform, &Sprite)>,
    buttons: Res<Input<MouseButton>>,
    mut lines : ResMut<DebugLines>,
    game_helper : Res<GameHelper>,
    mut drag_info : ResMut<DragInfo>,
    mut drop_element_event : EventWriter<DropElementEvent>
) {
    for (mut slot, transform, sprite) in slot_query.iter_mut() {
        let rect = Slot::generate_rect(transform, sprite);
        rect.draw_rect(&mut lines, Color::RED);
        //draw_box(&mut lines, transform.translation, width, height, Color::RED);

        let is_within = rect.is_within(game_helper.mouse_world_pos());

        if is_within && buttons.just_pressed(MouseButton::Right) && slot.element.is_some() && slot.can_change {
            slot.element = None;
        }

        if is_within && buttons.just_pressed(MouseButton::Left) && drag_info.currently_dragging.is_none() && slot.element.is_some() && !slot.can_change {
            drag_info.currently_dragging = Some(slot.element.as_ref().unwrap().clone());
            drag_info.should_change_sprite = true;
        }

        if buttons.just_released(MouseButton::Left) && drag_info.currently_dragging.is_some() {
            drop_element_event.send(DropElementEvent(game_helper.mouse_world_pos(),drag_info.currently_dragging.as_ref().unwrap().clone()));
            drag_info.currently_dragging = None;
        }
    }
}

pub fn on_drop_element(
    mut slot_query : Query<(&mut Slot, &Transform, &Sprite)>,
    mut element_drop_event : EventReader<DropElementEvent>
) {
    for event in element_drop_event.iter() {
        for (mut slot, transform, sprite) in slot_query.iter_mut() {
            let rect = Slot::generate_rect(transform, sprite);
            if rect.is_within(event.0) {
                slot.element = Some(event.1.clone())
            }
        }
    }
}

pub fn add_slots(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite : Sprite {
                custom_size : Some(Vec2::splat(160.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, SLOT_LEVEL),
            visibility : Visibility {
                is_visible : false
            },
            ..default()
        })
        .insert(DragEntity)
        .insert(Name::new("Drag Entity"));

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("sprites/proto_kitchen_recipe.png"),
        transform: Transform::from_xyz(0.0, 0.0, UI_LELVEL),
        ..default()
    });

    let mut current_slots_taken = add_slot_array(&mut commands, -512.0, 200.0, 3, 4, 128.0);
    setup_mixer_slots(&mut commands, &mut current_slots_taken)
}

fn add_slot_array(commands: &mut Commands, x : f32, y : f32, width : u32, height : u32, slot_size : f32) -> u32{
    for hy in 0..height {
        for wx in 0..width {
            let pos = Vec2::new(x + slot_size * wx as f32, y - slot_size * hy as f32);
            commands.spawn_bundle(SpriteBundle{
                    transform : Transform::from_xyz(pos.x, pos.y, SLOT_LEVEL),
                    sprite : Sprite {
                        custom_size : Some(Vec2::splat(slot_size)),
                        ..default()
                    },
                    ..default()
                })
                .insert(Slot::with_index(wx + hy * width));
        }
    }
    width * height
}

fn setup_mixer_slots(commands: &mut Commands, slots_taken : &mut u32) {
    let pos_x = -64.0;
    let pos_y = 88.0;

    let slot_pos_1 = Vec3::new(pos_x, pos_y, SLOT_LEVEL);
    let slot_pos_2 = Vec3::new(pos_x + 128.0, pos_y  , SLOT_LEVEL);

    commands.spawn_bundle(SpriteBundle {
        transform : Transform::from_translation(slot_pos_1),
        sprite : Sprite {
            custom_size: Some(Vec2::splat(128.0)),
            ..default()
        },
        ..default()
    })
        .insert(Slot{element : None, can_change: true, index: slots_taken.clone()})
        .insert(MixerSlot1);

    commands.spawn_bundle(SpriteBundle {
        transform : Transform::from_translation(slot_pos_2),
        sprite : Sprite {
            custom_size: Some(Vec2::splat(128.0)),
            ..default()
        },
        ..default()
    })
        .insert(Slot{element : None, can_change: true, index: slots_taken.clone() + 1})
        .insert(MixerSlot2);
}




