use bevy::prelude::*;
use bevy::text::Text2dBounds;
use crate::element::Element;
use crate::game::Game;
use crate::ui::ElementInfoEvent;

pub struct PagePlugin;

impl PagePlugin {
    pub const ON_SCREEN_POS: Transform = Transform::from_xyz(392., 0., 10.);
    pub const OFF_SCREEN_POS: Transform = Transform::from_xyz(877., 0., 10.);
}

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system_to_stage(CoreStage::PostUpdate, listen_for_right_click)
            .add_system(move_page)
            .add_system(close_info);
    }
}

fn listen_for_right_click(
    game: Res<Game>,
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut element_info_event: EventReader<ElementInfoEvent>,
    mut query_page: Query<&mut Transform, With<Page>>,
    mut query_sprite: Query<(&mut Handle<Image>, &mut Sprite), With<PageItemSprite>>,
    mut query_title: Query<&mut Text, (With<PageTitle>, Without<PageText>)>,
    mut query_text: Query<&mut Text, (With<PageText>, Without<PageTitle>)>,
) {
    for info in element_info_event.iter() {
        let element: &Element = &info.0;

        if let Ok(mut transform) = query_page.get_single_mut() {
            if let Ok((mut handle, mut sprite)) = query_sprite.get_single_mut() {
                if let Ok(mut title) = query_title.get_single_mut() {
                    if let Ok(mut text) = query_text.get_single_mut() {
                        let file_path = format!("sprites/{}.png", element.id);
                        *handle = asset_server.load(&file_path);
                        title.sections[0].value = element.name.to_string();
                        text.sections[0].value = element.desc.to_string();

                        // make page move left
                        commands.entity(game.pages[0]).insert(MovingTo(PagePlugin::ON_SCREEN_POS));
                    }
                }
            }
        }
    }
}

fn close_info(keys: Res<Input<KeyCode>>, game: Res<Game>, mut commands: Commands) {
    if keys.just_pressed(KeyCode::Escape) {
        commands.entity(game.pages[0]).insert(MovingTo(PagePlugin::OFF_SCREEN_POS));
    }
}

#[derive(Component)]
struct MovingTo(Transform);

pub const SIZE: u32 = 64;

fn move_page(
    mut commands: Commands,
    time: Res<Time>,
    game: Res<Game>,
    mut query: Query<(Entity, &mut Transform, &MovingTo)>,
) {
    for (ent, mut transform, moving_to) in query.iter_mut() {
        let mut diff = moving_to.0.translation - transform.translation;
        let mut length = time.delta_seconds() * SIZE as f32 * 20.;
        // page finishes moving
        if length >= diff.length() {
            length = diff.length();
            commands.entity(ent).remove::<MovingTo>();
        }
        if length > 0. {
            diff = length * diff / diff.length();
            transform.translation = transform.translation + diff;
        }
    }
}

#[derive(Component)]
struct PageText;

#[derive(Component)]
struct PageTitle;

#[derive(Component)]
struct Page;

#[derive(Component)]
struct PageItemSprite;

fn setup(mut game: ResMut<Game>, mut commands: Commands, asset_server: Res<AssetServer>) {
    let parent = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(56. * 8., 76. * 8.)),
                ..default()
            },
            // offscreen = 877
            transform: Transform::from_xyz(877., 0., 10.),
            texture: asset_server.load("sprites/page.png"),
            ..default()
        })
        .insert(Page)
        .insert(Name::new("Page"))
        .id();

    game.pages.push(parent);

    let sprite = commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(16. * 8., 16. * 8.)),
            ..default()
        },
        transform: Transform::from_xyz(0., 152., 15.),
        texture: asset_server.load("sprites/empty.png"),
        ..default()
    })
        .insert(PageItemSprite)
        .insert(Name::new("Page Sprite"))
        .id();

    let text_style = TextStyle {
        font: asset_server.load("fonts/pixel_font.ttf"),
        font_size: 23.,
        color: Color::rgb(0.57, 0.42, 0.28),
    };
    let title_style = TextStyle {
        font: asset_server.load("fonts/pixel_font.ttf"),
        font_size: 27.6,
        color: Color::rgb(0.54, 0.23, 0.12),
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Left,
    };
    let title_alignment = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Center,
    };

    let title = commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("", title_style).with_alignment(title_alignment),
        transform: Transform::from_xyz(0., 42.5, 15.),
        ..default()
    })
        .insert(PageTitle)
        .insert(Name::new("Page Title"))
        .id();

    let text = commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("", text_style).with_alignment(text_alignment),
        transform: Transform::from_xyz(-119.7, -1.5, 15.),
        text_2d_bounds: Text2dBounds {
            size: Vec2::new(282., 100.),
        },
        ..default()
    })
        .insert(PageText)
        .insert(Name::new("Page Text"))
        .id();

    commands.entity(parent).push_children(&[title, text, sprite]);
}