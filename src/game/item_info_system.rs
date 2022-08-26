use bevy::input::mouse::mouse_button_input_system;
use bevy::prelude::*;
use bevy::utils::tracing::field::debug;
use crate::game::{AssetStorage, Item, TextureId};
use crate::Mouse;
use crate::positioning::Coords;

/// === Enums ===
#[derive(Debug)]
pub enum MouseOverState {
    Started,
    Ended,
}

/// === Events ===

/// This event is broadcast when the mouse starts or stops hovering over an entity with an Item Component.
#[derive(Debug)]
pub struct MouseOverEvent {
    pub state: MouseOverState,
}

/// === Components ===

/// This marker component is added to entities with Item Components that are currently being moused over.
#[derive(Component)]
pub struct MouseOver {}

/// This component is added to the entity spawned to display the item info.
#[derive(Component, Debug)]
pub struct MouseOverItemInfo {
    // pub name: String,
    // pub description: String,
    // pub is_showing: bool,
}

/// === Systems ===
pub fn check_mouse_over_item_system(
    mut commands: Commands,
    mouse: Res<Mouse>,
    new_hover_query: Query<(Entity, &Item, &Transform, &Coords), Without<MouseOver>>,
    old_hover_query: Query<(Entity, &Item, &Transform, &Coords), With<MouseOver>>,
    mut mouse_over_event_writer: EventWriter<MouseOverEvent>,
) {
    // Add and remove Item Info Component
    for (entity, _, transform, coords) in new_hover_query.iter() {
        let is_mouse_over_item = mouse.position.x > transform.translation.x - coords.dimens.x as f32 * 0.5
            && mouse.position.x < transform.translation.x + coords.dimens.x as f32 * 0.5
            && mouse.position.y > transform.translation.y - coords.dimens.y as f32 * 0.5
            && mouse.position.y < transform.translation.y + coords.dimens.y as f32 * 0.5;

        if is_mouse_over_item {
            debug!("new_hover_query - is_mouse_over_item: {:?}", is_mouse_over_item);
            commands.entity(entity).insert(MouseOver {});
            mouse_over_event_writer.send(MouseOverEvent { state: MouseOverState::Started })
        }
    }
    for (entity, _, transform, coords) in old_hover_query.iter() {
        let is_mouse_over_item = mouse.position.x > transform.translation.x - coords.dimens.x as f32 * 0.5
            && mouse.position.x < transform.translation.x + coords.dimens.x as f32 * 0.5
            && mouse.position.y > transform.translation.y - coords.dimens.y as f32 * 0.5
            && mouse.position.y < transform.translation.y + coords.dimens.y as f32 * 0.5;

        if !is_mouse_over_item {
            debug!("old_hover_query - is_mouse_over_item: {:?}", is_mouse_over_item);
            commands.entity(entity).remove::<MouseOver>();
            mouse_over_event_writer.send(MouseOverEvent { state: MouseOverState::Ended })
        }
    }
}

pub fn update_mouse_over_item_info_system(
    mut commands: Commands,
    mouse: Res<Mouse>,
    windows: Res<Windows>,
    assets: Res<AssetStorage>,
    asset_server: Res<AssetServer>,
    mouse_over_items: Query<&Item, With<MouseOver>>,
    mouse_over_item_info_query: Query<Entity, With<MouseOverItemInfo>>,
    mut mouse_over_events: EventReader<MouseOverEvent>,
) {
    for event in mouse_over_events.iter() {
        debug!("update_mouse_over_item_info_system, state: {:?}", event.state);
        match event.state {
            MouseOverState::Started => {
                for item in mouse_over_items.iter() {
                    commands.spawn_bundle(
                        TextBundle::from_section(
                            item.name.clone(),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                        )
                            .with_style(Style {
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    top: Val::Px(mouse.screen_pos_inverted.y + 12.0),
                                    left: Val::Px(mouse.screen_pos_inverted.x),
                                    ..default()
                                },
                                ..default()
                            }),
                    )
                        .insert(Name::new("MouseOverItemInfo"))
                        .insert(MouseOverItemInfo {});

                    commands.spawn_bundle(
                        TextBundle::from_section(
                            item.description.clone(),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                        )
                            .with_style(Style {
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    top: Val::Px(mouse.screen_pos_inverted.y - 12.0),
                                    left: Val::Px(mouse.screen_pos_inverted.x),
                                    ..default()
                                },
                                ..default()
                            }),
                    )
                        .insert(Name::new("MouseOverItemInfo"))
                        .insert(MouseOverItemInfo {});


                    // commands.spawn_bundle(
                    //     NodeBundle {
                    //         style: Style {
                    //             size: Size::new(Val::Percent(10.0), Val::Percent(10.0)),
                    //             ..default()
                    //         },
                    //         transform: Transform::from_xyz(mouse.position.x, mouse.position.y, 5.0),
                    //         ..default()
                    //     }
                    // )
                    // .insert(Name::new("MouseOverItemInfo"))
                    //     .insert(MouseOverItemInfo {});
                    // .with_children(|parent| {
                    //     parent.spawn_bundle(
                    //         SpriteBundle {
                    //             sprite: Sprite {
                    //                 ..default()
                    //             },
                    //             texture: assets.texture(&TextureId::Croissant),
                    //             transform: Transform::from_xyz(mouse.position.x, mouse.position.y, 10.0),
                    //             ..default()
                    //         }
                    //     )
                    // .with_children(|parent| {
                    //     parent.spawn_bundle(TextBundle {
                    //         text: Text::from_section(
                    //             item.name.clone(),
                    //             TextStyle {
                    //                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    //                 font_size: 24.0,
                    //                 color: Color::BLACK,
                    //             },
                    //         ),
                    //         transform: Transform::from_xyz(0.0, 5.0, 0.0),
                    //         ..default()
                    //     });
                    //     parent.spawn_bundle(TextBundle {
                    //         text: Text::from_section(
                    //             item.description.clone(),
                    //             TextStyle {
                    //                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    //                 font_size: 24.0,
                    //                 color: Color::BLACK,
                    //             },
                    //         ),
                    //         transform: Transform::from_xyz(0.0, -5.0, 0.0),
                    //         ..default()
                    //     });
                    // });
                    // });
                }
            }
            MouseOverState::Ended => {
                for entity in mouse_over_item_info_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}


// Trash

// this isn't working
// commands.spawn_bundle(
//     SpriteBundle {
//         sprite: Sprite {
//             ..default()
//         },
//         texture: assets.texture(&TextureId::Croissant),
//         transform: Transform::from_xyz(mouse.position.x, mouse.position.y, 10.0),
//         ..default()
//     }
// ).insert(Name::new("ItemInfo"))
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

// third attempt
// commands.spawn_bundle(
//     NodeBundle {
//         style: Style {
//             size: Size::new(Val::Percent(10.0), Val::Percent(10.0)),
//             ..default()
//         },
//         transform: Transform::from_xyz(mouse.position.x, mouse.position.y, 5.0),
//         ..default()
//     }
// )
//     .insert(Name::new("ItemInfo"));
// .with_children(|parent| {
//     parent.spawn_bundle(
//         SpriteBundle {
//             sprite: Sprite {
//                 ..default()
//             },
//             texture: assets.texture(&TextureId::Croissant),
//             transform: Transform::from_xyz(mouse.position.x, mouse.position.y, 10.0),
//             ..default()
//         }
//     )
//         .with_children(|parent| {
//             parent.spawn_bundle(TextBundle {
//                 text: name_text,
//                 transform: Transform::from_xyz(0.0, 5.0, 0.0),
//                 ..default()
//             });
//             parent.spawn_bundle(TextBundle {
//                 text: description_text,
//                 transform: Transform::from_xyz(0.0, -5.0, 0.0),
//                 ..default()
//             });
//         });
// });