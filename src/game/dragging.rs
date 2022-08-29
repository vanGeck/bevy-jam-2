use bevy::prelude::*;

use crate::game::items::{CraftItem, Item};
use crate::game::{AssetStorage, CleanupOnGameplayEnd, Silhouette};
use crate::mouse::{Mouse, MouseInteractive};
use crate::positioning::Depth;
use crate::positioning::Pos;
use crate::positioning::{Coords, GridData};

/// Broadcast this event when completing a dragging operation.
/// The entity that is being dragged still has the BeingDragged component.
/// The Pos is the target position that the item is moved towards.
#[derive(Debug)]
pub struct DragEvent(Pos);

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
    grid: Res<GridData>,
    mut mouse: ResMut<Mouse>,
    query: Query<(&Coords, Entity, &Item, &MouseInteractive), Without<Silhouette>>,
) {
    if mouse.is_dragging {
        return;
    }
    for (coords, entity, item, interactive) in query.iter() {
        if interactive.clicked && !interactive.shift_clicked && !interactive.ctrl_alt_clicked {
            let hovered_over_cell = Pos::from(mouse.position - grid.offset);
            commands.entity(entity).insert(BeingDragged);
            commands.entity(entity).insert(Silhouette);
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1., 1., 1., 0.5),
                        custom_size: Some(coords.dimens.as_vec2()),
                        ..default()
                    },
                    texture: assets.texture(&item.texture_id),
                    transform: Transform::from_xyz(
                        grid.calc_x(coords),
                        grid.calc_y(coords),
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
pub fn set_ghost_position(
    grid: Res<GridData>,
    mouse: Res<Mouse>,
    mut query: Query<(&mut Transform, &mut Coords, &DragGhost)>,
) {
    if let Ok((mut transform, mut coords, ghost)) = query.get_single_mut() {
        coords.pos = Pos::from(mouse.position - grid.offset) + ghost.cursor_delta;
        transform.translation.x = grid.calc_x(&coords);
        transform.translation.y = grid.calc_y(&coords);
    }
}

/// Checks if the dragging move would be valid. If not, tints the ghost red.
pub fn check_ghost_placement_validity(
    grid: Res<GridData>,
    mut query_ghost: Query<(&mut DragGhost, &mut Sprite, &Coords)>,
    query_items: Query<&Coords, (With<Item>, Without<BeingDragged>)>,
) {
    if let Ok((mut ghost, mut sprite, coords)) = query_ghost.get_single_mut() {
        let conflicts_with_item = query_items.iter().any(|item| coords.overlaps(item));
        if !conflicts_with_item
            && (grid.inventory.encloses(coords) || grid.crafting.encloses(coords))
        {
            ghost.placement_valid = true;
            sprite.color = Color::rgba(1., 1., 1., 0.5);
        } else {
            ghost.placement_valid = false;
            sprite.color = Color::rgba(1., 0., 0., 0.5);
        }
    }
}

/// Check if the dragging operation should be concluded. If so;
/// - Delete the ghost entity.
/// - Mark the mouse as no longer in the middle of a drag operation.
/// - Broadcast a DragEvent.
pub fn check_drag_end(
    mut writer: EventWriter<DragEvent>,
    mut mouse: ResMut<Mouse>,
    input: Res<Input<MouseButton>>,
    query_ghost: Query<&Coords, With<DragGhost>>,
) {
    if !mouse.is_dragging || !input.just_released(MouseButton::Left) {
        return;
    }
    mouse.is_dragging = false;
    let ghost_coords = query_ghost.single();
    writer.send(DragEvent(ghost_coords.pos));
}

pub fn process_drag_event(
    mut commands: Commands,
    grid: Res<GridData>,
    mut events: EventReader<DragEvent>,
    query_ghost: Query<(Entity, &DragGhost)>,
    mut query_item: Query<(Entity, &mut Transform, &mut Coords), With<BeingDragged>>,
) {
    for DragEvent(end) in events.iter() {
        if let Ok((entity, mut transform, mut coords)) = query_item.get_single_mut() {
            let (ghost_entity, ghost) = query_ghost.single();
            commands.entity(ghost_entity).despawn_recursive();
            commands.entity(entity).remove::<BeingDragged>();
            commands.entity(entity).remove::<Silhouette>();
            if ghost.placement_valid {
                coords.pos = *end;
                transform.translation.x = grid.calc_x(&coords);
                transform.translation.y = grid.calc_y(&coords);
                if grid.crafting.encloses(&coords) {
                    commands.entity(entity).insert(CraftItem);
                } else if grid.inventory.encloses(&coords) {
                    commands.entity(entity).remove::<CraftItem>();
                }
            }
        }
    }
}
