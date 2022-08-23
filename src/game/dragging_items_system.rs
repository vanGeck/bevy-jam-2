use bevy::prelude::*;

use crate::config::config_grid::GridConfig;
use crate::config::data_recipes::RecipesData;
use crate::game::items::Item;
use crate::game::{AssetStorage, CleanupOnGameplayEnd};
use crate::mouse::Mouse;
use crate::positioning::Coords;
use crate::positioning::Depth;
use crate::positioning::Pos;
use super::combining_items_system::*;

/// === Events ===

/// Broadcast this event when completing a dragging operation.
/// The entity that is being dragged still has the BeingDragged component.
/// The Pos is the target position that the item is moved towards.
#[derive(Debug)]
pub struct DragEndedEvent(Pos);

/// === Components ===

// TODO: Use this??
#[derive(Component)]
pub struct Draggable;

/// This marker component is added to items that are currently being dragged.
#[derive(Component)]
pub struct BeingDragged;

/// Marker component for the entity that spawns when dragging an item.
/// The original item stays in its place, the ghost indicates where the item will end up.
#[derive(Component, Default)]
pub struct DragGhost {
    /// Difference between where the cursor is and where the DragGhost's bottom-left corner is.
    /// Accounts for cases where the player didn't start the dragging on the bottom-left corner.
    cursor_delta: Pos,
    placement_valid: bool,
}

/// === Systems ===

/// TODO: There's no logic separating normal clicks from drag initiation.
///       Needs to be included down the road.
///
/// Handles initiating a dragging operation.
/// When an item starts being dragged;
///     - That item is marked with the BeingDragged component.
///     - A ghost item is spawned.
///     - The mouse is tagged as being in the middle of a dragging operation.
pub fn check_drag_begin(
    mut commands: Commands,
    assets: Res<AssetStorage>,
    input: Res<Input<MouseButton>>,
    mut query_mouse: Query<&mut Mouse>,
    query: Query<(&Coords, Entity, &Item)>,
) {
    let mut mouse = query_mouse.single_mut();
    if mouse.is_dragging {
        return;
    }
    let hovered_over_cell = Pos::from(mouse.position);
    if !input.just_pressed(MouseButton::Left) {
        mouse.can_drag = query
            .iter()
            .any(|(coords, _, _)| coords.overlaps_pos(&hovered_over_cell));
        return;
    }
    for (coords, entity, item) in query.iter() {
        if coords.overlaps_pos(&hovered_over_cell) {
            commands.entity(entity).insert(BeingDragged);
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1., 1., 1., 0.5),
                        custom_size: Some(coords.dimens.as_vec2()),
                        ..default()
                    },
                    texture: assets.texture(&item.texture_id),
                    // Can someone please explain the math going on here to me? - Jacques
                    transform: Transform::from_xyz(
                        coords.pos.x as f32 + coords.dimens.x as f32 * 0.5,
                        coords.pos.y as f32 + coords.dimens.y as f32 * 0.5,
                        Depth::FloatingItem.z(),
                    ),
                    ..Default::default()
                })
                .insert(*coords)
                .insert(DragGhost {
                    cursor_delta: coords.pos - hovered_over_cell,
                    ..default()
                })
                .insert(CleanupOnGameplayEnd);
            mouse.is_dragging = true;
        }
    }
}

/// Move the item ghost with the mouse, but in discrete increments, always snapping to the grid.
pub fn update_dragged_ghost_item_position(
    mut query_mouse: Query<&mut Mouse>,
    mut query: Query<(&mut Transform, &mut Coords, &DragGhost)>,
) {
    let mouse = query_mouse.single_mut();
    if let Ok((mut transform, mut coords, ghost)) = query.get_single_mut() {
        coords.pos = Pos::from(mouse.position) + ghost.cursor_delta;
        // Again, can someone please explain the math here to me? I am guessing it's to do with the snapping. - Jacques
        transform.translation.x = coords.pos.x as f32 + coords.dimens.x as f32 * 0.5;
        transform.translation.y = coords.pos.y as f32 + coords.dimens.y as f32 * 0.5;
    }
}

// Jacques: I don't know if this needs to be run on every frame as it's own system,
// perhaps we can move this logic to the check drag begin and end systems?
/// Apply a dark tint to the item that is being dragged.
pub fn update_dragged_item_tint(
    mut query: Query<(&mut Sprite, Option<&BeingDragged>), With<Item>>,
) {
    for (mut sprite, being_dragged) in query.iter_mut() {
        sprite.color = if being_dragged.is_some() {
            Color::rgba(0.5, 0.5, 0.5, 0.5)
        } else {
            Color::rgb(1., 1., 1.)
        };
    }
}

/// Checks if the dragging move would be valid. If not, tints the ghost red.
pub fn update_dragged_ghost_item_validity(
    grid: Res<GridConfig>,
    mut query_ghost: Query<(&mut DragGhost, &mut Sprite, &Coords)>, // the ghost of the item that we are dragging
    query_possible_overlapped_items: Query<(&Coords, &Item), Without<BeingDragged>>, // other items in the grid
    query_being_dragged_item: Query<&Item, With<BeingDragged>>, // this is the item that we are dragging
    recipesData: Res<RecipesData>,
) {
    if let Ok((mut ghost, mut ghost_sprite, ghost_coords)) = query_ghost.get_single_mut() {
        let is_inside_a_grid = grid.inventory.encloses(ghost_coords) ||
            grid.crafting.encloses(ghost_coords);

        if !is_inside_a_grid {
            ghost.placement_valid = false;
            ghost_sprite.color = Color::rgba(1., 0., 0., 0.5);
            return;
        }

        let possible_overlapped_item = query_possible_overlapped_items.iter().find(|(overlapped_item_coords, _)| ghost_coords.overlaps(overlapped_item_coords));
        if let Some((_, overlapped_item)) = possible_overlapped_item {
            if let Ok(being_dragged_item) = query_being_dragged_item.get_single() {
                let overlapped_item_id = &overlapped_item.id;
                let being_dragged_item_id = &being_dragged_item.id;
                // Check if they can combine, else they can't place the item.

                let can_combine = is_valid_recipe(&recipesData, being_dragged_item_id.clone(), overlapped_item_id.clone());

                if let Some(recipe) = can_combine {
                    // change to a different color, add Combine Component, remove the two items in that system, spawn a new item that is the result
                } else {
                    ghost.placement_valid = false;
                    ghost_sprite.color = Color::rgba(1., 0., 0., 0.5);
                }
            }
        }

        // Good to move
        ghost.placement_valid = true;
        ghost_sprite.color = Color::rgba(1., 1., 1., 0.5);
    }
}

/// Check if the dragging operation should be concluded. If so;
/// - Delete the ghost entity.
/// - Mark the mouse as no longer in the middle of a drag operation.
/// - Broadcast a DragEvent.
pub fn check_drag_end(
    mut writer: EventWriter<DragEndedEvent>,
    mut query_mouse: Query<&mut Mouse>,
    input: Res<Input<MouseButton>>,
    query_ghost: Query<&Coords, With<DragGhost>>,
) {
    let mut mouse = query_mouse.single_mut();
    if !mouse.is_dragging || !input.just_released(MouseButton::Left) {
        return;
    }
    mouse.is_dragging = false;
    let ghost_coords = query_ghost.single();
    writer.send(DragEndedEvent(ghost_coords.pos));
}

pub fn process_drag_ended_event(
    mut commands: Commands,
    mut events: EventReader<DragEndedEvent>,
    query_ghost: Query<(Entity, &DragGhost)>,
    mut query_item: Query<(Entity, &mut Transform, &mut Coords), With<BeingDragged>>,
) {
    for event in events.iter() {
        debug!("Received drag ended event: {:?}", event);
        let end_pos = event.0;
        if let Ok((entity, mut transform, mut coords)) = query_item.get_single_mut() {
            let (ghost_entity, ghost) = query_ghost.single();
            commands.entity(ghost_entity).despawn_recursive();
            commands.entity(entity).remove::<BeingDragged>();
            if ghost.placement_valid {
                coords.pos.x = end_pos.x;
                coords.pos.y = end_pos.y;
                transform.translation.x = coords.pos.x as f32 + coords.dimens.x as f32 * 0.5;
                transform.translation.y = coords.pos.y as f32 + coords.dimens.y as f32 * 0.5;
            }
        }
    }
}
