use bevy::prelude::*;
use bevy::text::{Text2dBounds, Text2dSize};

use crate::config::data_layout::LayoutData;
use crate::game::{AssetStorage, EquipmentSlot, FontId, Item, TextureId};
use crate::Mouse;
use crate::mouse::MouseInteractive;
use crate::positioning::Depth;

/// === Components ===

/// This marker component is added to entities with Item Components that are currently being moused over.
#[derive(Component)]
pub struct MousedOver;

/// This component is added to the entity spawned to display the item info.
#[derive(Component, Debug)]
pub struct TooltipBg;

#[derive(Component, Debug)]
pub struct TooltipName;

#[derive(Component, Debug)]
pub struct TooltipDescription;

#[derive(Component, Debug)]
pub struct TooltipWearable;

#[derive(Component, Debug)]
pub struct TooltipStats;

/// === Systems ===

/// Adds and removes MouseOver components to items that are being hovered over.
/// Also deletes the tooltip if an item is not hovered over anymore.
pub fn update_mouse_over_item_info_system(
    mut commands: Commands,
    new_mouse_over_items_query: Query<(Entity, &MouseInteractive, &Item), Without<MousedOver>>,
    old_mouse_over_items_query: Query<(Entity, &MouseInteractive, &Item), With<MousedOver>>,
    item_info_query: Query<Entity, With<TooltipBg>>,
    assets: Res<AssetStorage>,
    mouse: Res<Mouse>,
    layout: Res<LayoutData>,
) {
    // Add new item info
    for (item_entity, mouse_interaction, item) in new_mouse_over_items_query.iter() {
        if mouse_interaction.hovered {
            commands.entity(item_entity).insert(MousedOver);

            // Spawn the container with the sprite background:
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(0.)), //Will be adjusted later.
                        ..default()
                    },
                    texture: assets.texture(&TextureId::TooltipBackground),
                    transform: Transform::from_xyz(
                        mouse.position.x,
                        mouse.position.y,  //Will be adjusted later.
                        Depth::Cursor.z(), // Relative to parent grid.
                    ),
                    ..default()
                })
                .insert(Name::new("MouseOverItemInfo"))
                .insert(TooltipBg)
                .with_children(|parent| {
                    let text_alignment = TextAlignment {
                        horizontal: HorizontalAlign::Left,
                        vertical: VerticalAlign::Center,
                    };
                    let text_bounds = Vec2::new(5., 3.);
                    // Spawn the name text:
                    parent
                        .spawn_bundle(Text2dBundle {
                            // Default text, will probably never be seen:
                            text: Text::from_section(
                                &item.name,
                                TextStyle {
                                    font: assets.font(&FontId::FiraSansBold),
                                    font_size: 80.0,
                                    color: Color::ANTIQUE_WHITE,
                                },
                            )
                                .with_alignment(text_alignment),
                            // The max size that it should fit in:
                            text_2d_bounds: Text2dBounds {
                                size: Vec2::new(
                                    text_bounds.x * layout.text_factor,
                                    text_bounds.y * layout.text_factor,
                                ),
                            },
                            transform: Transform::from_translation(Vec3::new(0., 0., 1.0))
                                .with_scale(Vec3::new(
                                    1. / layout.text_factor,
                                    1. / layout.text_factor,
                                    1.,
                                )),
                            ..default()
                        })
                        .insert(TooltipName);
                    // Spawn the description text:
                    parent
                        .spawn_bundle(Text2dBundle {
                            // Default text, will probably never be seen:
                            text: Text::from_section(
                                &item.description,
                                TextStyle {
                                    font: assets.font(&FontId::FiraSansItalic),
                                    font_size: 60.0,
                                    color: Color::ANTIQUE_WHITE,
                                },
                            )
                                .with_alignment(text_alignment),
                            // The max size that it should fit in:
                            text_2d_bounds: Text2dBounds {
                                size: Vec2::new(
                                    text_bounds.x * layout.text_factor,
                                    text_bounds.y * layout.text_factor,
                                ),
                            },
                            transform: Transform::from_translation(Vec3::new(0., 0., 1.0))
                                .with_scale(Vec3::new(
                                    1. / layout.text_factor,
                                    1. / layout.text_factor,
                                    1.,
                                )),
                            ..default()
                        })
                        .insert(TooltipDescription);
                    // If applicable, spawn the wearable text:
                    if let Some(slot) = item.wearable {
                        let slot_name = match slot {
                            EquipmentSlot::Armour => "Armour".to_string(),
                            EquipmentSlot::Shield => "Shield".to_string(),
                            EquipmentSlot::Weapon => "Weapon".to_string(),
                        };
                        parent
                            .spawn_bundle(Text2dBundle {
                                // Default text, will probably never be seen:
                                text: Text::from_section(
                                    &slot_name,
                                    TextStyle {
                                        font: assets.font(&FontId::FiraSansMedium),
                                        font_size: 60.0,
                                        color: Color::ANTIQUE_WHITE,
                                    },
                                )
                                    .with_alignment(text_alignment),
                                // The max size that it should fit in:
                                text_2d_bounds: Text2dBounds {
                                    size: Vec2::new(
                                        text_bounds.x * layout.text_factor,
                                        text_bounds.y * layout.text_factor,
                                    ),
                                },
                                transform: Transform::from_translation(Vec3::new(0., 0., 1.0))
                                    .with_scale(Vec3::new(
                                        1. / layout.text_factor,
                                        1. / layout.text_factor,
                                        1.,
                                    )),
                                ..default()
                            })
                            .insert(TooltipWearable);
                    }
                    // If applicable, spawn Stat Bonuses text:
                    let stats_text = item
                        .stat_bonuses
                        .map(|stat_bonus| {
                            let mut stats: String = format!("Stats:\n");
                            let mut stats_present = false;
                            if stat_bonus.proficiency > 0 {
                                stats_present = true;
                                stats.push_str(&*format!(
                                    "    | Combat Proficiency: {}\n",
                                    stat_bonus.proficiency
                                ));
                            }
                            if stat_bonus.damage_bonus > 0 {
                                stats_present = true;
                                stats.push_str(&*format!("    | Damage: {}\n", stat_bonus.damage_bonus));
                            }
                            if stat_bonus.damage_res > 0 {
                                stats_present = true;
                                stats.push_str(&*format!(
                                    "    | Damage Resistance: {}\n",
                                    stat_bonus.damage_res
                                ));
                            }
                            if stat_bonus.max_health > 0 {
                                stats_present = true;
                                stats.push_str(&*format!("    | Max HP: {}\n", stat_bonus.proficiency));
                            }
                            if stats_present {
                                Some(stats)
                            } else {
                                None
                            }
                        })
                        .flatten();
                    if let Some(stat_bonus) = stats_text {
                        parent
                            .spawn_bundle(Text2dBundle {
                                // Default text, will probably never be seen:
                                text: Text::from_section(
                                    stat_bonus,
                                    TextStyle {
                                        font: assets.font(&FontId::FiraSansMedium),
                                        font_size: 60.0,
                                        color: Color::ANTIQUE_WHITE,
                                    },
                                )
                                    .with_alignment(text_alignment),
                                // The max size that it should fit in:
                                text_2d_bounds: Text2dBounds {
                                    size: Vec2::new(
                                        text_bounds.x * layout.text_factor,
                                        text_bounds.y * layout.text_factor,
                                    ),
                                },
                                transform: Transform::from_translation(Vec3::new(0., 0., 1.0))
                                    .with_scale(Vec3::new(
                                        1. / layout.text_factor,
                                        1. / layout.text_factor,
                                        1.,
                                    )),
                                ..default()
                            })
                            .insert(TooltipStats);
                    }
                });
        }
    }
    // Remove old item info
    for (item_entity, mouse_interaction, _) in old_mouse_over_items_query.iter() {
        if !mouse_interaction.hovered {
            commands.entity(item_entity).remove::<MousedOver>();
            for item_info_entity in item_info_query.iter() {
                commands.entity(item_info_entity).despawn_recursive()
            }
        }
    }
}

/// Update the position of the tooptip to move with the mouse and maybe flip around to the other
/// side of the cursor if the tooltip would otherwise go off the screen.
pub fn update_mouse_over_item_info_style_position_system(
    layout: Res<LayoutData>,
    mouse: Res<Mouse>,
    mut queries: ParamSet<(
        Query<(&mut Sprite, &mut Transform), With<TooltipBg>>,
        Query<(&Text2dSize, &mut Transform), With<TooltipName>>,
        Query<(&Text2dSize, &mut Transform), With<TooltipDescription>>,
        Query<(&Text2dSize, &mut Transform), With<TooltipWearable>>,
        Query<(&Text2dSize, &mut Transform), With<TooltipStats>>,
    )>,
) {
    if queries.p0().get_single().is_err() {
        return;
    }
    let size_name = queries.p1().single().0.size / layout.text_factor;
    let size_description = queries.p2().single().0.size / layout.text_factor;
    let size_wearable = queries
        .p3()
        .get_single()
        .map(|(size, _)| size.size)
        .unwrap_or(Vec2::splat(0.))
        / layout.text_factor;
    let size_stats = queries
        .p4()
        .get_single()
        .map(|(size, _)| size.size)
        .unwrap_or(Vec2::splat(0.))
        / layout.text_factor;
    let padding = 0.25;
    let container_size = Vec2::new(
        padding * 2.
            + size_name
            .x
            .max(size_description.x)
            .max(size_wearable.x)
            .max(size_stats.x),
        padding * 2. + size_stats.y + size_wearable.y + size_description.y + size_name.y,
    );
    let anchor = container_size * -0.5;

    if let Ok((_, mut transform)) = queries.p4().get_single_mut() {
        transform.translation.x = anchor.x + padding;
        transform.translation.y = anchor.y + padding + size_stats.y * 0.5;
    }
    if let Ok((_, mut transform)) = queries.p3().get_single_mut() {
        transform.translation.x = anchor.x + padding;
        transform.translation.y = anchor.y + padding + size_stats.y + size_wearable.y * 0.5;
    }
    if let Ok((_, mut transform)) = queries.p2().get_single_mut() {
        transform.translation.x = anchor.x + padding;
        transform.translation.y =
            anchor.y + padding + size_stats.y + size_wearable.y + size_description.y * 0.5;
    }
    if let Ok((_, mut transform)) = queries.p1().get_single_mut() {
        transform.translation.x = anchor.x + padding;
        transform.translation.y = anchor.y
            + padding
            + size_stats.y
            + size_wearable.y
            + size_description.y
            + size_name.y * 0.5;
    }

    let enough_room_on_right = layout.screen_dimens.x - mouse.position.x > container_size.x + 2. * padding;
    let enough_room_on_bottom = layout.screen_dimens.y - mouse.position.y > container_size.y + 2. * padding;
    if let Ok((mut sprite, mut transform)) = queries.p0().get_single_mut() {
        sprite.custom_size = Some(container_size);
        transform.translation.x = if enough_room_on_right {
            mouse.position.x + padding - anchor.x
        } else {
            mouse.position.x - padding - anchor.x - container_size.x
        };
        transform.translation.y = if enough_room_on_bottom {
            mouse.position.y - padding + anchor.y + container_size.y
        } else {
            mouse.position.y + padding + anchor.y - container_size.y
        };
    }
}
