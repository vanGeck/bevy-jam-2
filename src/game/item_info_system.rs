use bevy::input::mouse::mouse_button_input_system;
use bevy::prelude::*;
use bevy::utils::tracing::field::debug;
use crate::game::{AssetStorage, EquipmentSlot, Item, TextureId};
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
            // debug!("new_hover_query - is_mouse_over_item: {:?}", is_mouse_over_item);
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
            // debug!("old_hover_query - is_mouse_over_item: {:?}", is_mouse_over_item);
            commands.entity(entity).remove::<MouseOver>();
            mouse_over_event_writer.send(MouseOverEvent { state: MouseOverState::Ended })
        }
    }
}

pub fn update_mouse_over_item_info_system(
    mut commands: Commands,
    mouse: Res<Mouse>,
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
                    debug!("there are mouse_over_items to process");
                    commands.spawn_bundle(
                        NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    top: Val::Px(mouse.screen_pos_inverted.y),
                                    left: Val::Px(mouse.screen_pos_inverted.x),
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        }
                    )
                        .insert(Name::new("MouseOverItemInfo"))
                        .insert(MouseOverItemInfo {})
                        .with_children(|parent| {
                            // Name
                            parent.spawn_bundle(
                                TextBundle::from_section(
                                    item.name.clone(),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                )
                                    .with_style(Style {
                                        position_type: PositionType::Absolute,
                                        position: UiRect {
                                            top: Val::Px(-24.0),
                                            left: Val::Px(0.0),
                                            ..default()
                                        },
                                        ..default()
                                    }),
                            );
                            // Description
                            parent.spawn_bundle(
                                TextBundle::from_section(
                                    item.description.clone(),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                )
                                    .with_style(Style {
                                        position_type: PositionType::Absolute,
                                        position: UiRect {
                                            top: Val::Px(-12.0),
                                            left: Val::Px(0.0),
                                            ..default()
                                        },
                                        ..default()
                                    })
                            );
                            // Wearable
                            if let Some((slot, _)) = item.wearable.clone() {
                                let slot_name: String;
                                match slot {
                                    EquipmentSlot::Armour => { slot_name = "Armour".to_string(); }
                                    EquipmentSlot::Helmet => { slot_name = "Helmet".to_string(); }
                                    EquipmentSlot::Necklace => { slot_name = "Necklace".to_string(); }
                                    EquipmentSlot::Shield => { slot_name = "Shield".to_string(); }
                                    EquipmentSlot::Weapon => { slot_name = "Weapon".to_string(); }
                                }
                                parent.spawn_bundle(
                                    TextBundle::from_section(
                                        slot_name,
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                        .with_style(Style {
                                            position_type: PositionType::Absolute,
                                            position: UiRect {
                                                top: Val::Px(0.0),
                                                left: Val::Px(0.0),
                                                ..default()
                                            },
                                            ..default()
                                        }),
                                );
                            }
                            // Stat Bonuses
                            if let Some(stat_bonus) = item.stat_bonuses {
                                let mut stats: String = format!("| Stats: ");
                                if stat_bonus.combat_prof > 0 { stats.push_str(&*format!("Combat Proficiency: {} | ", stat_bonus.combat_prof)); }
                                if stat_bonus.damage > 0 { stats.push_str(&*format!("Damage: {} | ", stat_bonus.damage)); }
                                if stat_bonus.damage_res > 0 { stats.push_str(&*format!("Damage Resistance: {} | ", stat_bonus.damage_res)); }
                                if stat_bonus.max_hp > 0 { stats.push_str(&*format!("Max HP: {} | ", stat_bonus.combat_prof)); }

                                parent.spawn_bundle(
                                    TextBundle::from_section(
                                        stats,
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                        .with_style(Style {
                                            position_type: PositionType::Absolute,
                                            position: UiRect {
                                                top: Val::Px(12.0),
                                                left: Val::Px(0.0),
                                                ..default()
                                            },
                                            ..default()
                                        }),
                                );
                            }
                        });
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

pub fn update_mouse_over_item_info_position_system() {}

// Backup

/*
// Name
                    commands.entity(mouse_over_info_entity).insert_bundle(
                        spawn_bundle(
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
                    );

                    // Description
                    commands.entity(mouse_over_info_entity).insert_bundle(spawn_bundle(
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
                    ));

                    // Wearable
                    if let Some((slot, _)) = item.wearable.clone() {
                        let slot_name: String;
                        match slot {
                            EquipmentSlot::Armour => { slot_name = "Armour".to_string(); }
                            EquipmentSlot::Helmet => { slot_name = "Helmet".to_string(); }
                            EquipmentSlot::Necklace => { slot_name = "Necklace".to_string(); }
                            EquipmentSlot::Shield => { slot_name = "Shield".to_string(); }
                            EquipmentSlot::Weapon => { slot_name = "Weapon".to_string(); }
                        }
                        commands.entity(mouse_over_info_entity).insert_bundle(spawn_bundle(
                            TextBundle::from_section(
                                slot_name,
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                },
                            )
                                .with_style(Style {
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        top: Val::Px(mouse.screen_pos_inverted.y - 24.0),
                                        left: Val::Px(mouse.screen_pos_inverted.x),
                                        ..default()
                                    },
                                    ..default()
                                }),
                        ));
                    }

                    // Stat Bonuses
                    if let Some(stat_bonus) = item.stat_bonuses {
                        let mut stats: String = format!("| Stats: ");
                        if stat_bonus.combat_prof > 0 { stats.push_str(&*format!("Combat Proficiency: {} | ", stat_bonus.combat_prof)); }
                        if stat_bonus.damage > 0 { stats.push_str(&*format!("Damage: {} | ", stat_bonus.damage)); }
                        if stat_bonus.damage_res > 0 { stats.push_str(&*format!("Damage Resistance: {} | ", stat_bonus.damage_res)); }
                        if stat_bonus.max_hp > 0 { stats.push_str(&*format!("Max HP: {} | ", stat_bonus.combat_prof)); }

                        commands.entity(mouse_over_info_entity).insert_bundle(spawn_bundle(
                            TextBundle::from_section(
                                stats,
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                },
                            )
                                .with_style(Style {
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        top: Val::Px(mouse.screen_pos_inverted.y - 32.0),
                                        left: Val::Px(mouse.screen_pos_inverted.x),
                                        ..default()
                                    },
                                    ..default()
                                }),
                        ))
                    }
 */

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

// 4th attempt with sprite

/*
// I tried to child the text to a sprite but it didn't work

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
 */