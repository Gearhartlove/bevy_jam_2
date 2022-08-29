use std::time::Duration;
use bevy::ecs::schedule::ShouldRun::No;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy::utils::tracing::event;
use bevy_inspector_egui::egui::{DragValue, Ui};
use bevy_inspector_egui::{Context, Inspectable, RegisterInspectable};
use bevy_prototype_debug_lines::DebugLines;
use crate::element::Element;
use crate::GameHelper;
use crate::helper::add_scaled_pixel_asset;
use crate::page::MovingTo;
use crate::ui::{Rect, Slot, ToolSlot, UiData};

pub struct BossFightPlugin;

impl Plugin for BossFightPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_inspectable::<Clickable<ToggleBossUIEvent>>()
            .register_inspectable::<Clickable<CheckElementsEvent>>()
            .add_event::<SetupBossFightEvent>()
            .add_event::<ToggleBossUIEvent>()
            .add_event::<ToggleBossTimerEvent>()
            .add_event::<WinGameEvent>()
            .add_event::<LoseGameEvent>()
            .add_event::<CheckElementsEvent>()
            .init_resource::<BossUIData>()
            .add_system(test_system)
            .add_system(tick_clock)
            .add_system(on_click::<ToggleBossUIEvent>)
            .add_system(on_click::<CheckElementsEvent>)
            .add_system_to_stage(CoreStage::PostUpdate, setup_boss_fight)
            .add_system_to_stage(CoreStage::PostUpdate, on_toggle_timer)
            .add_system_to_stage(CoreStage::PostUpdate, on_toggle_boss_ui)
            .add_system_to_stage(CoreStage::PostUpdate, on_check_elements)
        ;
    }
}

//=================================================================================================
//                              Components
//=================================================================================================

#[derive(Component)]
pub struct BossUiSlot;

#[derive(Component, Default)]
pub struct BossToggleButton;

#[derive(Component, Default)]
pub struct BossDoneButton;

#[derive(Component)]
pub struct TimeCounter;

#[derive(Component, Clone)]
pub struct Clickable<T> where T : Sized + Default {
    pub rect : Rect,
    pub event : T
}

impl <T> Default for Clickable<T> where T : Sized + Default {
    fn default() -> Self {
        Clickable {
            rect : Rect::new(-10.0, 10.0, 10.0, -10.0),
            event : T::default()
        }
    }
}

impl <T> Inspectable for Clickable<T> where T : Sized + Default {
    type Attributes = ();

    fn ui(&mut self, ui: &mut Ui, options: Self::Attributes, context: &mut Context) -> bool {
        ui.horizontal(|ui| {
            ui.label("Point 1: ");
            ui.add(DragValue::new(&mut self.rect.x1));
            ui.add(DragValue::new(&mut self.rect.y1))
        });

        ui.horizontal(|ui| {
            ui.label("Point 2: ");
            ui.add(DragValue::new(&mut self.rect.x2));
            ui.add(DragValue::new(&mut self.rect.y2))
        });

        true
    }
}

#[derive(Component)]
pub struct BossTimer {
    timer : Timer,
    is_ticking : bool
}

//=================================================================================================
//                              Bundles
//=================================================================================================

#[derive(Bundle, Clone)]
pub struct ClickableBundle<T> where T : Default + Sized + Send + Sync + 'static {
    pub transform : Transform,
    pub global_transform : GlobalTransform,
    pub clickable : Clickable<T>
}

impl <T> Default for ClickableBundle<T> where T : Default + Sized + Send + Sync + 'static {
    fn default() -> Self {
        ClickableBundle {
            transform : Transform::default(),
            global_transform : GlobalTransform::default(),
            clickable : Clickable::default()
        }
    }
}


//=================================================================================================
//                              Events
//=================================================================================================

#[derive(Default, Debug)]
pub struct SetupBossFightEvent;

#[derive(Default, Debug)]
pub struct ToggleBossUIEvent;

#[derive(Default, Debug)]
pub struct ToggleBossTimerEvent;

#[derive(Default, Debug)]
pub struct CheckElementsEvent;

#[derive(Default, Debug)]
pub struct LoseGameEvent;

#[derive(Default, Debug)]
pub struct WinGameEvent;

//=================================================================================================
//                              Resources
//=================================================================================================

pub struct BossUIData {
    boss_ui_id : Option<Entity>,
    is_out : bool
}

impl Default for BossUIData {
    fn default() -> Self {
        BossUIData {
            boss_ui_id : None,
            is_out : false
        }
    }
}

//=================================================================================================
//                              Setup
//=================================================================================================

pub fn setup_boss_fight (
    mut commands: Commands,
    asset_server : Res<AssetServer>,
    mut setup_boss_fight : EventReader<SetupBossFightEvent>,
    mut ui : ResMut<UiData>,
    mut boss_ui_data : ResMut<BossUIData>
) {
    if !setup_boss_fight.is_empty() && boss_ui_data.boss_ui_id.is_none() {
        let parent = add_scaled_pixel_asset(&mut commands, &asset_server, "sprites/boss_fight_ui.png", 40, 88, SpriteBundle {
            transform : Transform::from_xyz(400.0, -725.0, 25.0),
            ..default()
        })
            .insert(Name::new("Boss Fight Menu"))
            .insert( MovingTo(Transform::from_xyz(400.0, -592.0, 25.0)))
            .id();

        let click = add_scaled_pixel_asset(&mut commands, &asset_server, "sprites/page_up.png", 9, 9, SpriteBundle {
            transform : Transform::from_xyz(-164.0, 288.0, 1.0),
            ..default()
        }).insert(Clickable {
            rect : Rect::new(-38.0, 38.0, 38.0, -38.0),
            event : ToggleBossUIEvent
        }).insert(BossToggleButton).id();

        let done_button = commands.spawn()
            .insert(Transform::from_xyz(0.0, -316.0, 1.0))
            .insert(GlobalTransform::default())
            .insert(Clickable {
                rect : Rect::new(-136.0, 28.0, 136.0, -28.0),
                event : CheckElementsEvent
            })
            .insert(BossDoneButton)
            .id();

        let text_style = TextStyle {
            font: asset_server.load("fonts/pixel_font.ttf"),
            font_size: 65.,
            color: Color::rgb(0.57, 0.42, 0.28),
        };

        let clock_alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        };

        let clock_text = commands.spawn_bundle(Text2dBundle {
            text : Text::from_section(" 9 59", text_style).with_alignment(clock_alignment),
            transform : Transform::from_xyz(0.0, 296.0, 1.0),
            text_2d_bounds : Text2dBounds {
                size : Vec2::new(236.0, 86.0)
            },
            ..default()
        }).insert(BossTimer {
            is_ticking : false,
            timer : Timer::new(Duration::from_secs(600), false)
        }).id();

        commands.entity(parent.clone()).push_children(&[click, clock_text, done_button]);

        add_slot_array(&mut commands, -64.0, 168.0, 2, 4, 128.0, &mut ui.amount_of_slots_indices, &parent);

        boss_ui_data.boss_ui_id = Some(parent);
        setup_boss_fight.clear();
    }
}

fn add_slot_array(commands: &mut Commands, x : f32, y : f32, width : u32, height : u32, slot_size : f32, starting_slot : &mut u32, parent : &Entity) -> u32{
    for hy in 0..height {
        for wx in 0..width {
            let pos = Vec2::new(x + slot_size * wx as f32, y - slot_size * hy as f32);
            let slot = commands.spawn_bundle(SpriteBundle{
                transform : Transform::from_xyz(pos.x, pos.y, 1.0),
                sprite : Sprite {
                    custom_size : Some(Vec2::splat(slot_size)),
                    ..default()
                },
                ..default()
            })
                .insert(Slot::with_index_changable(starting_slot.clone()))
                .insert(BossUiSlot)
                .insert(ToolSlot)
                .id();
            commands.entity(parent.clone()).add_child(slot);
            *starting_slot += 1;
        }
    }
    width * height
}

//=================================================================================================
//                              Event Listeners
//=================================================================================================

pub fn on_toggle_boss_ui(
    mut commands : Commands,
    mut toggle_boss_ui_event : EventReader<ToggleBossUIEvent>,
    mut boss_ui_data : ResMut<BossUIData>,
    mut button : Query<&mut Handle<Image>, With<BossToggleButton>>,
    asset_server : Res<AssetServer>
) {
    if !toggle_boss_ui_event.is_empty() {
        if let Some(entity) = boss_ui_data.boss_ui_id {
            let mut texture = button.single_mut();

            if boss_ui_data.is_out {
                commands.entity(entity).insert(MovingTo(Transform::from_xyz(400.0, -592.0, 25.0)));
                *texture = asset_server.load("sprites/page_up.png")
            } else {
                commands.entity(entity).insert(MovingTo(Transform::from_xyz(400.0, -8.0, 25.0)));
                *texture = asset_server.load("sprites/page_down.png")
            };

            boss_ui_data.is_out = !boss_ui_data.is_out;
        }

        toggle_boss_ui_event.clear();
    }
}

pub fn on_toggle_timer (
    mut toggle_timer_event : EventReader<ToggleBossTimerEvent>,
    mut timer : Query<&mut BossTimer>
) {
    if !toggle_timer_event.is_empty() {
        let mut timer = timer.get_single_mut();
        if let Ok(mut timer) = timer {
            timer.is_ticking = !timer.is_ticking
        }

        toggle_timer_event.clear()
    }
}

pub fn on_check_elements (
    boss_slots : Query<&Slot, With<BossUiSlot>>,
    mut on_check_elements : EventReader<CheckElementsEvent>
) {
    if !on_check_elements.is_empty() {

        let has_pepper_flakes = false;
        let has_chopsticks = false;
        let has_both = false;
        let has_hard_boiled_eggs = false;
        let has_chashu = false;
        let has_noodles = false;
        let has_dried_seaweed = false;
        let has_ice_cube = false;


        for slot in boss_slots.iter() {
            if let Some(element) = &slot.element {
                //has_pepper_flakes = element == Element::PEPPER_FLAKES
            }
        }

        on_check_elements.clear()
    }
}

//=================================================================================================
//                              Systems
//=================================================================================================

pub fn test_system (
    keys : Res<Input<KeyCode>>,
    mut setup_boss_fight_event : EventWriter<SetupBossFightEvent>,
    mut toggle_boss_ui_event : EventWriter<ToggleBossUIEvent>,
    mut toggle_boss_timer_event : EventWriter<ToggleBossTimerEvent>,
) {
    if keys.just_pressed(KeyCode::B) {
        setup_boss_fight_event.send(SetupBossFightEvent)
    }

    if keys.just_pressed(KeyCode::T) {
        toggle_boss_timer_event.send(ToggleBossTimerEvent)
    }

    if keys.just_pressed(KeyCode::P) {
        toggle_boss_ui_event.send(ToggleBossUIEvent)
    }
}

pub fn tick_clock (
    mut clock : Query<(&mut BossTimer, &mut Text)>,
    time : Res<Time>,
    mut lose_game_event : EventWriter<LoseGameEvent>
) {
    let clock = clock.get_single_mut();

    if let Ok((mut timer, mut text)) = clock {
        let duration = timer.timer.elapsed();
        let seconds = 60 - (duration.as_secs() % 60);
        let mut minutes = 9 - ((duration.as_secs() / 60) % 60);
        let is_60 = seconds == 60;
        let text_value = if is_60 {
            minutes += 1;
            format!("{minutes:0>2} 00")
        } else {
            format!("{minutes:0>2} {seconds:0>2}")
        };

        text.sections.get_mut(0).unwrap().value = text_value;

        if timer.timer.finished() {
            println!("YOU LOOSE");
            lose_game_event.send(LoseGameEvent);
        }

        if timer.is_ticking {
            timer.timer.tick(time.delta());
        }
    }
}

pub fn on_click<T> (
    mut lines : ResMut<DebugLines>,
    clickables : Query<(&Clickable<T>, &GlobalTransform), With<Transform>>,
    game_helper : Res<GameHelper>,
    clicks : Res<Input<MouseButton>>,
    mut event_writer : EventWriter<T>
) where T : Default + Send + Sync + 'static {
    for (clickable, transform) in clickables.iter() {
        let trans = transform.translation();
        clickable.rect.draw_rect_with_offset(&mut lines, Color::GREEN, trans.truncate());
        if clicks.just_pressed(MouseButton::Left) && clickable.rect.is_within_with_offset(game_helper.mouse_world_pos(), trans.truncate()) {
            event_writer.send(T::default())
        }
    }
}