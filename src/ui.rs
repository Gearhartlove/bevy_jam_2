use std::ops::Add;
use bevy::ecs::schedule::ShouldRun::No;
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::math::{vec2, Vec3Swizzles};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::Texture;
use bevy::utils::tracing::event;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::Collider;
use crate::GameHelper;
use crate::registry::Registry;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DragInfo>()
            .add_event::<DropElementEvent>()
            .add_startup_system(add_slots)
            .add_system(render_slots)
            .add_system(render_dragging)
            .add_system(drag_item)
            .add_system(on_drop_element.after(drag_item));
    }
}

//==================================================================================================
//                          DragElement Event
//==================================================================================================

#[derive(Debug)]
pub struct DropElementEvent(Vec2, String);

//==================================================================================================
//                          DragInfo Resource
//==================================================================================================

pub struct DragInfo {
    pub currently_dragging : Option<String>,
    pub should_change_sprite : bool,

}

impl Default for DragInfo {
    fn default() -> Self {
        DragInfo {
            currently_dragging : None,
            should_change_sprite : false
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

#[derive(Component, Default)]
pub struct Slot{
    element : Option<String>,
    can_change : bool
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

fn render_slots(
    mut slot : Query<(&Slot, &mut Handle<Image>, &mut Visibility)>,
    asset_server: Res<AssetServer>,
    registry : Res<Registry>
) {
    for (s, mut handle, mut visibility) in slot.iter_mut() {
        if s.element.is_some() && registry.element_registry.contains_key(s.element.as_ref().unwrap().as_str()) {
            let element_data = registry.element_registry.get(s.element.as_ref().unwrap().as_str()).unwrap();
            visibility.is_visible = true;
            let sprite = asset_server.load(element_data.sprite.as_str());
            *handle = sprite;
        } else {
            visibility.is_visible = false;
        }
    }
}

fn render_dragging (
    mut drag_entity : Query<(&mut Transform, &mut Handle<Image>, &mut Visibility), With<DragEntity>>,
    registry : Res<Registry>,
    drag_info : Res<DragInfo>,
    game_helper : Res<GameHelper>,
    asset_server: Res<AssetServer>
) {
    let (mut transform, mut handle, mut visibility) = drag_entity.single_mut();

    if drag_info.currently_dragging.is_some() {
        if drag_info.should_change_sprite {
            let element = registry.element_registry.get(drag_info.currently_dragging.as_ref().unwrap());
            if let Some(element) = element {
                *handle = asset_server.load(element.sprite.as_str())
            }
        }

        visibility.is_visible = true;
        transform.translation = game_helper.mouse_world_pos().extend(0.0);
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
            transform : Transform::from_xyz(0.0, 0.0, 0.0),
            sprite : Sprite {
                custom_size : Some(Vec2::splat(160.0)),
                ..default()
            },
            ..default()
        })
        .insert(Slot{element : Some("fire_pepper".to_string()), can_change: false})
        .insert(Name::new("Pepper"));

    commands
        .spawn_bundle(SpriteBundle{
            transform : Transform::from_xyz(200.0, 0.0, 0.0),
            sprite : Sprite {
                custom_size : Some(Vec2::splat(160.0)),
                ..default()
            },
            ..default()
        })
        .insert(Slot{element : None, can_change: true})
        .insert(Name::new("Test"));;

    commands
        .spawn_bundle(SpriteBundle {
            texture : asset_server.load("sprites/fire_pepper.png"),
            sprite : Sprite {
                custom_size : Some(Vec2::splat(160.0)),
                ..default()
            },
            visibility : Visibility {
                is_visible : false
            },
            ..default()
        })
        .insert(DragEntity)
        .insert(Name::new("Drag Entity"));
}



