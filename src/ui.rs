use std::ops::Add;
use bevy::ecs::schedule::ShouldRun::No;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::math::{vec2, Vec3Swizzles};
use bevy::prelude::*;
use bevy::reflect::Array;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::Texture;
use bevy::text::Text2dSize;
use bevy::utils::tracing::event;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::Collider;
use crate::element::Element;
use crate::{GameHelper, MixerRecipeIden};
use crate::registry::{FurnaceRecipeIden, Registry};

const TAVERN_LEVEL : f32 = 10.0;
const UI_LEVEL: f32 = 20.0;
const SLOT_LEVEL : f32 = 30.0;
const TEXT_LEVEL : f32 = 40.0;
const TOP_LEVEL : f32 = 50.0;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiData>()
            .add_event::<DropElementEvent>()
            .add_event::<UpdateSlotEvent>()
            .add_event::<SlotEnteredEvent>()
            .add_event::<SlotLeftEvent>()
            .add_event::<RefreshSlotsEvent>()
            .add_event::<ElementCraftedEvent>()
            .add_event::<CraftFailedEvent>()
            .add_event::<CraftRepeatedEvent>()
            .add_event::<ElementInfoEvent>()
            .add_event::<LoadMixerEvent>()
            .add_event::<LoadSlicerEvent>()
            .add_event::<InsertElementEvent>()
            .add_event::<PageUpEvent>()
            .add_event::<PageDownEvent>()
            .add_startup_system(setup_ui)
            .add_system(render_slots)
            .add_system(render_dragging)
            .add_system(drag_item)
            .add_system(check_for_mixer_craft)
            .add_system(check_for_furnace_craft)
            .add_system(check_for_slicer_craft)
            .add_system(test_system)
            //.add_system(on_drop_element.after(drag_item))
            .add_system_to_stage(CoreStage::PostUpdate, on_load_mixer)
            .add_system_to_stage(CoreStage::PostUpdate, on_load_slicer)
            .add_system_to_stage(CoreStage::PostUpdate, handle_slot_events)
            .add_system_to_stage(CoreStage::PostUpdate, hide_name)
            .add_system_to_stage(CoreStage::PostUpdate, show_name.after(hide_name))
            .add_system_to_stage(CoreStage::Last, refresh_slots)
        ;
    }
}

//==================================================================================================
//                          Slot Events
//==================================================================================================

#[derive(Debug)]
pub struct UpdateSlotEvent(u32, Option<Element>);

#[derive(Debug)]
pub struct DropElementEvent(Vec2, Element);

#[derive(Debug)]
pub struct SlotEnteredEvent(u32);

#[derive(Debug)]
pub struct SlotLeftEvent(u32);

#[derive(Debug)]
pub struct RefreshSlotsEvent;

#[derive(Debug)]
pub struct ElementCraftedEvent(pub Element);

#[derive(Debug)]
pub struct CraftFailedEvent(CraftType);

#[derive(Debug)]
pub struct CraftRepeatedEvent(CraftType);

#[derive(Debug)]
pub struct ElementInfoEvent(Element);

#[derive(Debug)]
pub struct LoadMixerEvent;

#[derive(Debug)]
pub struct LoadSlicerEvent;

#[derive(Debug)]
pub struct InsertElementEvent(Element);

#[derive(Debug)]
pub struct PageUpEvent;

#[derive(Debug)]
pub struct PageDownEvent;

#[derive(Debug)]
pub enum CraftType {
    SLICER,
    MIXER,
    FURNACE
}

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
//                          Event Reactors
//==================================================================================================

fn on_load_mixer(
    mut commands: Commands,
    mut ui_info : ResMut<UiData>,
    mut load_mixer_event : EventReader<LoadMixerEvent>
) {
    if !load_mixer_event.is_empty() {
        setup_mixer_slots(&mut commands, &mut ui_info.amount_of_slots_indices);
        load_mixer_event.clear()
    }
}

fn on_load_slicer(
    mut commands: Commands,
    mut ui_info : ResMut<UiData>,
    mut load_slicer_event: EventReader<LoadSlicerEvent>
) {
    if !load_slicer_event.is_empty() {
        setup_slicer_slot(&mut commands, &mut ui_info.amount_of_slots_indices);
        load_slicer_event.clear()
    }
}

fn on_insert_element(
    mut ui_info : ResMut<UiData>,
    mut insert_element_event : EventReader<InsertElementEvent>,
    mut refresh_slots : EventWriter<RefreshSlotsEvent>,
) {
    for event in insert_element_event.iter() {
        ui_info.add_element(event.0.clone());
        refresh_slots.send(RefreshSlotsEvent)
    }
}

fn on_page_up (
    mut page_up_event : EventReader<PageUpEvent>,
    mut ui_data : ResMut<UiData>
) {
    if ui_data.can_move_up() {

    }
}

//==================================================================================================
//                          DragInfo Resource
//==================================================================================================

pub struct UiData {
    pub currently_dragging : Option<Element>,
    pub should_change_sprite : bool,
    pub sprite_size : f32,
    pub last_slot_hovered: u32,
    known_elements : Vec<Element>,
    pub amount_of_slots_indices: u32,
    number_of_pages : u32,
    pub current_page : u32
}

impl UiData {
    pub fn can_move_up(&self) -> bool {
        self.current_page != 0
    }

    pub fn can_move_down(&self) -> bool {
        self.current_page != self.number_of_pages
    }

    pub fn add_element(&mut self, element : Element) {
        if !self.known_elements.contains(&element) {
            self.known_elements.push(element);
            let element_amount = self.known_elements.len();
            self.number_of_pages = ((element_amount - (element_amount % 12)) / 12) as u32
        }
    }

    pub fn unsafe_add(&mut self, element : Element) {
        self.known_elements.push(element)
    }

    pub fn number_of_pages(&self) -> u32 {
        self.number_of_pages
    }

    pub fn known_elements(&self) -> &Vec<Element> {
        &self.known_elements
    }
}

impl Default for UiData {
    fn default() -> Self {
        UiData {
            currently_dragging : None,
            should_change_sprite : false,
            sprite_size : 16.0,
            last_slot_hovered: u32::MAX,
            known_elements : Vec::new(),
            amount_of_slots_indices: 0,
            number_of_pages: 0,
            current_page : 0,
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
pub struct TitleText;

#[derive(Component)]
pub struct ToolSlot;

#[derive(Component)]
pub struct PageUp;

#[derive(Component)]
pub struct PageDown;

#[derive(Component)]
pub struct Slot {
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

//==================================================================================================
//                          Systems
//==================================================================================================

fn test_system (
    keys: Res<Input<KeyCode>>,
    mut ui_data : ResMut<UiData>,
    mut slot_refresh : EventWriter<RefreshSlotsEvent>,
    mut load_mixer : EventWriter<LoadMixerEvent>,
    mut load_slicer : EventWriter<LoadSlicerEvent>,
) {
    if keys.just_pressed(KeyCode::A) {
        ui_data.unsafe_add(Element::FIRE_PEPPER.clone());
        ui_data.unsafe_add(Element::YETI_WATER.clone());

        slot_refresh.send(RefreshSlotsEvent)
    }

    if keys.just_pressed(KeyCode::M) {
        load_mixer.send(LoadMixerEvent)
    }

    if keys.just_pressed(KeyCode::S) {
        load_slicer.send(LoadSlicerEvent)
    }
}

fn check_for_mixer_craft(
    mut slot_1_q : Query<&mut Slot, (With<MixerSlot1>, Without<MixerSlot2>)>,
    mut slot_2_q : Query<&mut Slot, (With<MixerSlot2>, Without<MixerSlot1>)>,
    registy : Res<Registry>,
    mut ui_data : ResMut<UiData>,
    mut refresh_slots : EventWriter<RefreshSlotsEvent>,
    mut element_crafted_event : EventWriter<ElementCraftedEvent>,
    mut craft_failed_event : EventWriter<CraftFailedEvent>,
    mut craft_repeated_event: EventWriter<CraftRepeatedEvent>
) {
    let mut slot_1 = slot_1_q.get_single_mut();
    let mut slot_2 = slot_2_q.get_single_mut();

    if let Ok(mut slot_1) = slot_1 {
        if let Ok(mut slot_2) = slot_2 {
            if slot_1.element.is_some() && slot_2.element.is_some() {
                let element_1 = slot_1.element.as_ref().unwrap().clone();
                let element_2 = slot_2.element.as_ref().unwrap().clone();

                let iden = MixerRecipeIden::new(element_1, element_2);

                let recipe = registy.mixer_recipe_registry.get(&iden);
                if recipe.is_some() {
                    let element = recipe.as_ref().unwrap().result.clone();
                    if !ui_data.known_elements.contains(&element) {
                        element_crafted_event.send(ElementCraftedEvent(element.clone()));
                        ui_data.add_element(element);
                        refresh_slots.send(RefreshSlotsEvent);
                    } else {
                        craft_repeated_event.send(CraftRepeatedEvent(CraftType::MIXER))
                    }
                } else {
                    craft_failed_event.send(CraftFailedEvent(CraftType::MIXER))
                }

                slot_1.element = None;
                slot_2.element = None;
            }
        }
    }
}

fn check_for_furnace_craft(
    mut slot_1_q : Query<&mut Slot, (With<FurnaceSlot1>, Without<FurnaceSlot2>)>,
    mut slot_2_q : Query<&mut Slot, (With<FurnaceSlot2>, Without<FurnaceSlot1>)>,
    registy : Res<Registry>,
    mut ui_data : ResMut<UiData>,
    mut refresh_slots : EventWriter<RefreshSlotsEvent>,
    mut element_crafted_event : EventWriter<ElementCraftedEvent>,
    mut craft_failed_event : EventWriter<CraftFailedEvent>,
    mut craft_repeated_event: EventWriter<CraftRepeatedEvent>
) {
    let mut slot_2 = slot_1_q.single_mut();
    let mut slot_1 = slot_2_q.single_mut();

    if slot_1.element.is_some() && slot_2.element.is_some() {
        let element_1 = slot_1.element.as_ref().unwrap().clone();
        let element_2 = slot_2.element.as_ref().unwrap().clone();

        let iden = FurnaceRecipeIden::new(element_1, element_2);

        let recipe = registy.furnace_recipe_registry.get(&iden);
        if recipe.is_some() {
            let element = recipe.as_ref().unwrap().result.clone();
            if !ui_data.known_elements.contains(&element) {
                element_crafted_event.send(ElementCraftedEvent(element.clone()));
                ui_data.add_element(element);
                refresh_slots.send(RefreshSlotsEvent);
            } else {
                //Add the you already have this response
                craft_repeated_event.send(CraftRepeatedEvent(CraftType::FURNACE))
            }
        } else {
            //Add error responce
            craft_failed_event.send(CraftFailedEvent(CraftType::FURNACE))
        }

        slot_1.element = None;
        slot_2.element = None;
    }
}

fn check_for_slicer_craft (
    mut slot_q : Query<&mut Slot, With<SlicerSlot>>,
    registry: Res<Registry>,
    mut ui_data : ResMut<UiData>,
    mut refresh_slots : EventWriter<RefreshSlotsEvent>,
    mut element_crafted_event : EventWriter<ElementCraftedEvent>,
    mut craft_failed_event : EventWriter<CraftFailedEvent>,
    mut craft_repeated_event: EventWriter<CraftRepeatedEvent>
) {
    let mut slot = slot_q.get_single_mut();

    if let Ok(mut slot) = slot {
        if slot.element.is_some() {
            let element = slot.element.as_ref().unwrap().clone();
            let recipe = registry.slicer_recipe_registry.get(&element);
            if let Some(recipe) = recipe {
                let result = recipe.result.clone();
                if !ui_data.known_elements.contains(&result) {
                    element_crafted_event.send(ElementCraftedEvent(result.clone()));
                    ui_data.add_element(element);
                    refresh_slots.send(RefreshSlotsEvent)
                } else {
                    // Add "already have that" response
                    craft_repeated_event.send(CraftRepeatedEvent(CraftType::SLICER))
                }
            } else {
                //Add not a recipe response
                craft_failed_event.send(CraftFailedEvent(CraftType::SLICER))
            }

            slot.element = None;
        }
    }
}

fn render_slots(
    mut slot : Query<(&Slot, &mut Handle<Image>, &mut Visibility)>,
    asset_server: Res<AssetServer>,
    ui_data : Res<UiData>
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
    mut drag_info : ResMut<UiData>,
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
    mut drag_info : ResMut<UiData>,
    mut drop_element_event : EventWriter<DropElementEvent>,
    mut entered_slot_event : EventWriter<SlotEnteredEvent>,
    mut left_slot_event : EventWriter<SlotLeftEvent>,
    mut element_info_event : EventWriter<ElementInfoEvent>
) {
    let mut is_in_slots = false;

    for (mut slot, transform, sprite) in slot_query.iter_mut() {
        let rect = Slot::generate_rect(transform, sprite);
        rect.draw_rect(&mut lines, Color::RED);
        //draw_box(&mut lines, transform.translation, width, height, Color::RED);

        let is_within = rect.is_within(game_helper.mouse_world_pos());
        if is_within {
            is_in_slots = true;
        }

        if is_within && drag_info.last_slot_hovered != slot.index {
            left_slot_event.send(SlotLeftEvent(drag_info.last_slot_hovered));
            entered_slot_event.send(SlotEnteredEvent(slot.index));
            drag_info.last_slot_hovered = slot.index
        }

        if is_within && buttons.just_pressed(MouseButton::Right) && slot.element.is_some() && slot.can_change {
            slot.element = None;
        }

        if is_within && buttons.just_pressed(MouseButton::Right) && slot.element.is_some() && !slot.can_change {
            element_info_event.send(ElementInfoEvent(slot.element.as_ref().unwrap().clone()));
            println!("INFO PLEASE!")
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

    if !is_in_slots && drag_info.last_slot_hovered != u32::MAX {
        left_slot_event.send(SlotLeftEvent(drag_info.last_slot_hovered));
        drag_info.last_slot_hovered = u32::MAX;
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

fn show_name(
    mut slot_query : Query<&Slot>,
    mut title : Query<(&mut Text, &mut Visibility), With<TitleText>>,
    mut slot_entered_event : EventReader<SlotEnteredEvent>
) {
    let (mut text, mut visibility) = title.single_mut();

    for event in slot_entered_event.iter() {
        for slot in slot_query.iter_mut() {
            if slot.index == event.0 && slot.element.is_some() {
                visibility.is_visible = true;
                if let Some(slot) = &slot.element {
                    text.sections.first_mut().unwrap().value = slot.name.to_string();
                }
            }
        }
    }
}

fn hide_name (
    mut title : Query<&mut Visibility, With<TitleText>>,
    mut slot_left_event : EventReader<SlotLeftEvent>
) {
    let mut title = title.single_mut();
    for event in slot_left_event.iter() {
        title.is_visible = false;
    }
}

fn refresh_slots (
    mut slot_query : Query<&mut Slot, Without<ToolSlot>>,
    ui_manager : Res<UiData>,
    mut refresh_event : EventReader<RefreshSlotsEvent>
) {
    if !refresh_event.is_empty() {
        for mut slot in slot_query.iter_mut() {
            let element = ui_manager.known_elements.get(slot.index as usize);
            if let Some(element) = element {
                slot.element = Some(element.to_owned());
            }
        }
        refresh_event.clear()
    }
}

//==================================================================================================
//                          Setup
//==================================================================================================

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, mut ui_info : ResMut<UiData>) {
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
        texture: asset_server.load("sprites/proto_kitchen_bg2.png"),
        transform: Transform::from_xyz(0.0, 0.0, UI_LEVEL),
        ..default()
    });

    let font = asset_server.load("fonts/pixel_font.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("Test", text_style),
        transform: Transform::from_xyz(-606.0, -292.0, TEXT_LEVEL),
        text_2d_size : Text2dSize {
            size: Vec2::new(256.0, 128.0)
        },
        visibility: Visibility { is_visible: false },
        ..default()
    }).insert(Name::new("Item Hover Text")).insert(TitleText);

    let mut current_slots_taken = add_slot_array(&mut commands, -512.0, 200.0, 3, 4, 128.0);
    setup_furnace_slots(&mut commands, &mut current_slots_taken);

    ui_info.amount_of_slots_indices = current_slots_taken;

    crate::helper::add_scaled_pixel_asset(&mut commands, &asset_server, "sprites/hor_x.png",45, 28, SpriteBundle {
        transform: Transform::from_xyz(8.0, 88.0, TOP_LEVEL),
        visibility: Visibility { is_visible: false },
        ..default()
    }).insert(Name::new("X"));

    crate::helper::add_scaled_pixel_asset(&mut commands, &asset_server, "sprites/page_down.png", 9, 9, SpriteBundle{
        transform: Transform::from_xyz(-188.0, -284.0, TOP_LEVEL),
        ..default()
    }).insert(Name::new("Page Down"));
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

fn setup_mixer_slots(commands: &mut Commands, mut slots_taken : &mut u32) {
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
        .insert(MixerSlot1)
        .insert(ToolSlot);

    commands.spawn_bundle(SpriteBundle {
        transform : Transform::from_translation(slot_pos_2),
        sprite : Sprite {
            custom_size: Some(Vec2::splat(128.0)),
            ..default()
        },
        ..default()
    })
        .insert(Slot{element : None, can_change: true, index: slots_taken.clone() + 1})
        .insert(MixerSlot2)
        .insert(ToolSlot);

    *slots_taken += 2;
}

fn setup_furnace_slots(commands: &mut Commands, mut slots_taken : &mut u32) {
    let pos_x = 0.0;
    let pos_y = -88.0;

    let slot_pos_1 = Vec3::new(pos_x, pos_y, SLOT_LEVEL);
    let slot_pos_2 = Vec3::new(pos_x, pos_y - 128.0 , SLOT_LEVEL);

    commands.spawn_bundle(SpriteBundle {
        transform : Transform::from_translation(slot_pos_1),
        sprite : Sprite {
            custom_size: Some(Vec2::splat(128.0)),
            ..default()
        },
        ..default()
    })
        .insert(Slot{element : None, can_change: true, index: slots_taken.clone()})
        .insert(FurnaceSlot1)
        .insert(ToolSlot)
        .insert(Name::new("Furnace Slot"));

    commands.spawn_bundle(SpriteBundle {
        transform : Transform::from_translation(slot_pos_2),
        sprite : Sprite {
            custom_size: Some(Vec2::splat(128.0)),
            ..default()
        },
        ..default()
    })
        .insert(Slot{element : None, can_change: true, index: slots_taken.clone() + 1})
        .insert(FurnaceSlot2)
        .insert(ToolSlot);

    *slots_taken += 2;
}

fn setup_slicer_slot(commands: &mut Commands, mut slots_taken : &mut u32) {
    commands.spawn_bundle(SpriteBundle {
        transform : Transform::from_translation(Vec3::new(0.0, 264.0, SLOT_LEVEL)),
        sprite : Sprite {
            custom_size: Some(Vec2::splat(128.0)),
            ..default()
        },
        ..default()
    })
        .insert(Slot{element : None, can_change: true, index: slots_taken.clone()})
        .insert(SlicerSlot)
        .insert(ToolSlot)
        .insert(Name::new("Slicer Slot"));

    *slots_taken += 1;
}