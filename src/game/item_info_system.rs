use bevy::prelude::*;
use crate::game::{AssetStorage, Item, TextureId};
use crate::Mouse;
use crate::positioning::Coords;

#[derive(Component, Debug)]
pub struct ItemInfo {
    pub name: String,
    pub description: String,
    pub is_showing: bool,
} // Marker Components

pub fn item_info_system(
    mut commands: Commands,
    mouse: Res<Mouse>,
    new_hover_query: Query<(Entity, &Item, &Transform, &Coords), Without<ItemInfo>>,
    old_hover_query: Query<(Entity, &Item, &Transform, &Coords), With<ItemInfo>>,
) {
    // Add and remove Item Info Component
    for (entity, item, transform, coords) in new_hover_query.iter() {
        let is_mouse_over_item = mouse.position.x > transform.translation.x - coords.dimens.x as f32 * 0.5
            && mouse.position.x < transform.translation.x + coords.dimens.x as f32 * 0.5
            && mouse.position.y > transform.translation.y - coords.dimens.y as f32 * 0.5
            && mouse.position.y < transform.translation.y + coords.dimens.y as f32 * 0.5;


        if is_mouse_over_item {
            commands.entity(entity).insert(ItemInfo {
                name: item.name.clone(),
                description: item.description.clone(),
                is_showing: false,
            });
        }
    }
    for (entity, _, transform, coords) in old_hover_query.iter() {
        let is_mouse_over_item = mouse.position.x > transform.translation.x - coords.dimens.x as f32 * 0.5
            && mouse.position.x < transform.translation.x + coords.dimens.x as f32 * 0.5
            && mouse.position.y > transform.translation.y - coords.dimens.y as f32 * 0.5
            && mouse.position.y < transform.translation.y + coords.dimens.y as f32 * 0.5;

        if !is_mouse_over_item {
            commands.entity(entity).remove::<ItemInfo>();
        }
    }
}

pub fn display_item_info_system(
    mut commands: Commands,
    mouse: Res<Mouse>,
    assets: Res<AssetStorage>,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut ItemInfo)>,
) {
    for (entity, mut item_info) in query.iter_mut() {
        if !item_info.is_showing {
            debug!("ItemInfo: {:?}", item_info);

            let name_text = Text::from_section(
                item_info.name.clone(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::BLACK,
                },
            );
            let description_text = Text::from_section(
                item_info.description.clone(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::BLACK,
                },
            );


            // this isn't working
            commands.spawn_bundle(
                SpriteBundle {
                    sprite: Sprite {
                        ..default()
                    },
                    texture: assets.texture(&TextureId::Croissant),
                    transform: Transform::from_xyz(mouse.position.x, mouse.position.y, 10.0),
                    ..default()
                }
            ).insert(Name::new("ItemInfo"))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: name_text,
                        transform: Transform::from_xyz(0.0, 5.0, 0.0),
                        ..default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: description_text,
                        transform: Transform::from_xyz(0.0, -5.0, 0.0),
                        ..default()
                    });
                });

            // second attempt, trying to use a button instead of a sprite
            // commands.spawn_bundle(ButtonBundle {
            //     style: Style {
            //         size: Size::new(Val::Px(64.0), Val::Px(64.0)),
            //         // center button
            //         margin: UiRect::all(Val::Auto),
            //         // horizontally center child text
            //         justify_content: JustifyContent::Center,
            //         // vertically center child text
            //         align_items: AlignItems::Center,
            //         ..default()
            //     },
            //     color: bevy::prelude::UiColor(Color::WHITE),
            //     ..default()
            // }).insert(Name::new("ItemInfo"))
            //     .with_children(|parent| {
            //         parent.spawn_bundle(TextBundle {
            //             text: name_text,
            //             transform: Transform::from_xyz(0.0, 5.0, 0.0),
            //             ..default()
            //         });
            //         parent.spawn_bundle(TextBundle {
            //             text: description_text,
            //             transform: Transform::from_xyz(0.0, -5.0, 0.0),
            //             ..default()
            //         });
            //     });


            item_info.is_showing = true;
        }
    }
}