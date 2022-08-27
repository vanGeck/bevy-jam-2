use bevy::prelude::*;

use crate::game::{EquipmentSlot, Item};
use crate::mouse::MouseInteractive;
use crate::Mouse;

/// === Components ===

/// This marker component is added to entities with Item Components that are currently being moused over.
#[derive(Component)]
pub struct MouseOver {}

/// This component is added to the entity spawned to display the item info.
#[derive(Component, Debug)]
pub struct MouseOverItemInfo {}

/// === Systems ===

pub fn update_mouse_over_item_info_system(
    mut commands: Commands,
    new_mouse_over_items_query: Query<(Entity, &MouseInteractive, &Item), Without<MouseOver>>,
    old_mouse_over_items_query: Query<(Entity, &MouseInteractive, &Item), With<MouseOver>>,
    item_info_query: Query<Entity, With<MouseOverItemInfo>>,
    asset_server: Res<AssetServer>,
    mouse: Res<Mouse>,
) {
    // Add new item info
    for (item_entity, mouse_interaction, item) in new_mouse_over_items_query.iter() {
        if mouse_interaction.hovered {
            commands.entity(item_entity).insert(MouseOver {});
            commands
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            top: Val::Px(mouse.screen_pos_inverted.y),
                            left: Val::Px(mouse.screen_pos_inverted.x + 32.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("MouseOverItemInfo"))
                .insert(MouseOverItemInfo {})
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Auto, Val::Auto),
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                top: Val::Px(0.0),
                                left: Val::Px(0.0),
                                ..default()
                            },
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::FlexStart,
                            flex_direction: FlexDirection::ColumnReverse,
                            ..default()
                        },
                        image: asset_server.load("textures/MyPanel2.png").into(),
                        ..default()
                    }).with_children(|parent| {
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
                                    position_type: PositionType::Relative,
                                    position: UiRect {
                                        top: Val::Px(0.0),
                                        left: Val::Px(0.0),
                                        ..default()
                                    },
                                    margin: UiRect {
                                        top: Val::Px(8.0),
                                        left: Val::Px(8.0),
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
                                    position_type: PositionType::Relative,
                                    position: UiRect {
                                        top: Val::Px(0.0),
                                        left: Val::Px(0.0),
                                        ..default()
                                    },
                                    margin: UiRect {
                                        top: Val::Px(8.0),
                                        left: Val::Px(8.0),
                                        ..default()
                                    },
                                    ..default()
                                }),
                        );
                        // Wearable
                        if let Some((slot, _)) = item.wearable.clone() {
                            let slot_name: String;
                            match slot {
                                EquipmentSlot::Armour => {
                                    slot_name = "Armour".to_string();
                                }
                                EquipmentSlot::Helmet => {
                                    slot_name = "Helmet".to_string();
                                }
                                EquipmentSlot::Necklace => {
                                    slot_name = "Necklace".to_string();
                                }
                                EquipmentSlot::Shield => {
                                    slot_name = "Shield".to_string();
                                }
                                EquipmentSlot::Weapon => {
                                    slot_name = "Weapon".to_string();
                                }
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
                            if stat_bonus.combat_prof > 0 {
                                stats.push_str(&*format!(
                                    "Combat Proficiency: {} | ",
                                    stat_bonus.combat_prof
                                ));
                            }
                            if stat_bonus.damage > 0 {
                                stats.push_str(&*format!("Damage: {} | ", stat_bonus.damage));
                            }
                            if stat_bonus.damage_res > 0 {
                                stats.push_str(&*format!(
                                    "Damage Resistance: {} | ",
                                    stat_bonus.damage_res
                                ));
                            }
                            if stat_bonus.max_hp > 0 {
                                stats.push_str(&*format!("Max HP: {} | ", stat_bonus.combat_prof));
                            }

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
                });
        }
    }
    // Remove old item info
    for (item_entity, mouse_interaction, _) in old_mouse_over_items_query.iter() {
        if !mouse_interaction.hovered {
            commands.entity(item_entity).remove::<MouseOver>();
            for item_info_entity in item_info_query.iter() {
                commands.entity(item_info_entity).despawn_recursive()
            }
        }
    }
}

pub fn update_mouse_over_item_info_style_position_system(
    mouse: Res<Mouse>,
    mut mouse_over_item_info_query: Query<&mut Style, With<MouseOverItemInfo>>,
) {
    for mut style in mouse_over_item_info_query.iter_mut() {
        style.position = UiRect {
            top: Val::Px(mouse.screen_pos_inverted.y),
            left: Val::Px(mouse.screen_pos_inverted.x + 32.0),
            ..default()
        }
    }
}

// References
// 1. https://docs.rs/bevy/latest/bevy/ui/struct.Style.html


// Old Code

// pub fn check_mouse_over_item_system(
//     mut commands: Commands,
//     new_hover_query: Query<(Entity, &MouseInteractive), Without<MouseOver>>,
//     old_hover_query: Query<(Entity, &MouseInteractive), With<MouseOver>>,
//     mut mouse_over_event_writer: EventWriter<MouseOverEvent>,
// ) {
//     // Add and remove Item Info Component
//     for (entity, interactive) in new_hover_query.iter() {
//         if interactive.hovered {
//             // debug!("new_hover_query - is_mouse_over_item: {:?}", is_mouse_over_item);
//             commands
//                 .entity(entity)
//                 .insert(MouseOver { displayed: false }); // This isn't completing before the event is sent (sometimes)
//             mouse_over_event_writer.send(MouseOverEvent {
//                 state: MouseOverState::Started,
//             })
//         }
//     }
//     for (entity, interactive) in old_hover_query.iter() {
//         if !interactive.hovered {
//             // debug!("old_hover_query - is_mouse_over_item: {:?}", is_mouse_over_item);
//             commands.entity(entity).remove::<MouseOver>();
//             mouse_over_event_writer.send(MouseOverEvent {
//                 state: MouseOverState::Ended,
//             })
//         }
//     }
// }

// pub fn update_mouse_over_item_info_system(
//     mut commands: Commands,
//     mouse: Res<Mouse>,
//     asset_server: Res<AssetServer>,
//     mut mouse_over_items: Query<(&Item, &mut MouseOver)>,
//     mouse_over_item_info_query: Query<Entity, With<MouseOverItemInfo>>,
//     mut mouse_over_events: EventReader<MouseOverEvent>,
// ) {
//     for (item, mut mouse_over) in mouse_over_items.iter_mut() {
//         if !mouse_over.displayed {
//             commands
//                 .spawn_bundle(NodeBundle {
//                     style: Style {
//                         position_type: PositionType::Absolute,
//                         position: UiRect {
//                             top: Val::Px(mouse.screen_pos_inverted.y),
//                             left: Val::Px(mouse.screen_pos_inverted.x),
//                             ..default()
//                         },
//                         ..default()
//                     },
//                     ..default()
//                 })
//                 .insert(Name::new("MouseOverItemInfo"))
//                 .insert(MouseOverItemInfo {})
//                 .with_children(|parent| {
//                     // Name
//                     parent.spawn_bundle(
//                         TextBundle::from_section(
//                             item.name.clone(),
//                             TextStyle {
//                                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                                 font_size: 20.0,
//                                 color: Color::WHITE,
//                             },
//                         )
//                             .with_style(Style {
//                                 position_type: PositionType::Absolute,
//                                 position: UiRect {
//                                     top: Val::Px(-24.0),
//                                     left: Val::Px(0.0),
//                                     ..default()
//                                 },
//                                 ..default()
//                             }),
//                     );
//                     // Description
//                     parent.spawn_bundle(
//                         TextBundle::from_section(
//                             item.description.clone(),
//                             TextStyle {
//                                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                                 font_size: 20.0,
//                                 color: Color::WHITE,
//                             },
//                         )
//                             .with_style(Style {
//                                 position_type: PositionType::Absolute,
//                                 position: UiRect {
//                                     top: Val::Px(-12.0),
//                                     left: Val::Px(0.0),
//                                     ..default()
//                                 },
//                                 ..default()
//                             }),
//                     );
//                     // Wearable
//                     if let Some((slot, _)) = item.wearable.clone() {
//                         let slot_name: String;
//                         match slot {
//                             EquipmentSlot::Armour => {
//                                 slot_name = "Armour".to_string();
//                             }
//                             EquipmentSlot::Helmet => {
//                                 slot_name = "Helmet".to_string();
//                             }
//                             EquipmentSlot::Necklace => {
//                                 slot_name = "Necklace".to_string();
//                             }
//                             EquipmentSlot::Shield => {
//                                 slot_name = "Shield".to_string();
//                             }
//                             EquipmentSlot::Weapon => {
//                                 slot_name = "Weapon".to_string();
//                             }
//                         }
//                         parent.spawn_bundle(
//                             TextBundle::from_section(
//                                 slot_name,
//                                 TextStyle {
//                                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                                     font_size: 20.0,
//                                     color: Color::WHITE,
//                                 },
//                             )
//                                 .with_style(Style {
//                                     position_type: PositionType::Absolute,
//                                     position: UiRect {
//                                         top: Val::Px(0.0),
//                                         left: Val::Px(0.0),
//                                         ..default()
//                                     },
//                                     ..default()
//                                 }),
//                         );
//                     }
//                     // Stat Bonuses
//                     if let Some(stat_bonus) = item.stat_bonuses {
//                         let mut stats: String = format!("| Stats: ");
//                         if stat_bonus.combat_prof > 0 {
//                             stats.push_str(&*format!(
//                                 "Combat Proficiency: {} | ",
//                                 stat_bonus.combat_prof
//                             ));
//                         }
//                         if stat_bonus.damage > 0 {
//                             stats.push_str(&*format!("Damage: {} | ", stat_bonus.damage));
//                         }
//                         if stat_bonus.damage_res > 0 {
//                             stats.push_str(&*format!(
//                                 "Damage Resistance: {} | ",
//                                 stat_bonus.damage_res
//                             ));
//                         }
//                         if stat_bonus.max_hp > 0 {
//                             stats.push_str(&*format!("Max HP: {} | ", stat_bonus.combat_prof));
//                         }
//
//                         parent.spawn_bundle(
//                             TextBundle::from_section(
//                                 stats,
//                                 TextStyle {
//                                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                                     font_size: 20.0,
//                                     color: Color::WHITE,
//                                 },
//                             )
//                                 .with_style(Style {
//                                     position_type: PositionType::Absolute,
//                                     position: UiRect {
//                                         top: Val::Px(12.0),
//                                         left: Val::Px(0.0),
//                                         ..default()
//                                     },
//                                     ..default()
//                                 }),
//                         );
//                     }
//                 });
//         }
//         mouse_over.displayed = true;
//     }
//     for event in mouse_over_events.iter() {
//         // debug!("update_mouse_over_item_info_system, state: {:?}", event.state);
//
//         match event.state {
//             MouseOverState::Started => {
//                 // Do nothing, had the spawning code in here before but it wasn't working as I expected.
//                 // It appears the component wasn't added to the entity before the event was received.
//             }
//             MouseOverState::Ended => {
//                 for entity in mouse_over_item_info_query.iter() {
//                     commands.entity(entity).despawn_recursive();
//                 }
//             }
//         }
//     }
// }
