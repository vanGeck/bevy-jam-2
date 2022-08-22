use bevy::prelude::*;

use crate::config::config_grid::GridConfig;
use crate::game::{AssetStorage, CleanupOnGameplayEnd, Item, SpriteType};
use crate::mouse::Mouse;
use crate::positioning::coordinates::Coordinates;
use crate::positioning::depth::Depth;
use crate::positioning::position::Position;

/// Broadcast this event when completing a dragging operation.
/// The entity that is being dragged still has the BeingDragged component.
/// The Pos is the target position that the item is moved towards.
#[derive(Debug)]
pub struct DragEvent(Position);

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
    cursor_delta: Position,
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
    asset_server: Res<AssetServer>,
    input: Res<Input<MouseButton>>,
    mut query_mouse: Query<&mut Mouse>,
    query: Query<(&Coordinates, Entity, &Item)>,
) {
    let mut mouse = query_mouse.single_mut();
    if mouse.is_dragging || !input.just_pressed(MouseButton::Left) {
        return;
    }
    let clicked_cell = Position::from(mouse.position);
    for (coords, entity, item) in query.iter() {
        if coords.overlaps_pos(&clicked_cell) {
            commands.entity(entity).insert(BeingDragged);
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1., 1., 1., 0.5),
                        custom_size: Some(coords.dimensions.as_vec2()),
                        ..default()
                    },
                    texture: asset_server.load(std::path::PathBuf::new().join("textures/").join(&item.sprite_path)),
                    transform: Transform::from_xyz(
                        coords.position.x as f32 + coords.dimensions.x as f32 * 0.5,
                        coords.position.y as f32 + coords.dimensions.y as f32 * 0.5,
                        Depth::FloatingItem.z(),
                    ),
                    ..Default::default()
                })
                .insert(coords.clone())
                .insert(DragGhost {
                    cursor_delta: coords.position - clicked_cell,
                    ..default()
                })
                .insert(CleanupOnGameplayEnd);
            mouse.is_dragging = true;
        }
    }
}

/// Move the item ghost with the mouse, but in discrete increments, always snapping to the grid.
pub fn set_ghost_position(
    mut query_mouse: Query<&mut Mouse>,
    mut query: Query<(&mut Transform, &mut Coordinates, &DragGhost)>,
) {
    let mouse = query_mouse.single_mut();
    if let Ok((mut transform, mut coords, ghost)) = query.get_single_mut() {
        coords.position = Position::from(mouse.position) + ghost.cursor_delta;
        transform.translation.x = coords.position.x as f32 + coords.dimensions.x as f32 * 0.5;
        transform.translation.y = coords.position.y as f32 + coords.dimensions.y as f32 * 0.5;
    }
}

/// Apply a dark scrim to the item that is being dragged.
pub fn apply_scrim_to_being_dragged(
    mut query: Query<(&mut Sprite, Option<&BeingDragged>), With<Item>>,
) {
    for (mut sprite, being_dragged) in query.iter_mut() {
        sprite.color = if let Some(_) = being_dragged {
            Color::rgba(0.5, 0.5, 0.5, 0.5)
        } else {
            Color::rgb(1., 1., 1.)
        };
    }
}

/// Checks if the dragging move would be valid. If not, tints the ghost red.
pub fn check_ghost_placement_validity(
    grid: Res<GridConfig>,
    mut query_ghost: Query<(&mut DragGhost, &mut Sprite, &Coordinates)>,
    query_items: Query<&Coordinates, (With<Item>, Without<BeingDragged>)>,
) {
    if let Ok((mut ghost, mut sprite, coords)) = query_ghost.get_single_mut() {
        let conflicts_with_item = query_items.iter().any(|item| coords.overlaps(item));
        if !conflicts_with_item
            && (grid.equipment.encloses(coords) || grid.crafting.encloses(coords))
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
    mut query_mouse: Query<&mut Mouse>,
    input: Res<Input<MouseButton>>,
    query_ghost: Query<&Coordinates, With<DragGhost>>,
) {
    let mut mouse = query_mouse.single_mut();
    if !mouse.is_dragging || !input.just_released(MouseButton::Left) {
        return;
    }
    mouse.is_dragging = false;
    let ghost_coords = query_ghost.single();
    writer.send(DragEvent(ghost_coords.position));
}

pub fn process_drag_event(
    mut commands: Commands,
    mut events: EventReader<DragEvent>,
    query_ghost: Query<(Entity, &DragGhost)>,
    mut query_item: Query<(Entity, &mut Transform, &mut Coordinates), With<BeingDragged>>,
) {
    for event in events.iter() {
        debug!("Received drag item event: {:?}", event);
        let DragEvent(end) = event;
        if let Ok((entity, mut transform, mut coords)) = query_item.get_single_mut() {
            let (ghost_entity, ghost) = query_ghost.single();
            commands.entity(ghost_entity).despawn_recursive();
            commands.entity(entity).remove::<BeingDragged>();
            if ghost.placement_valid {
                coords.position.x = end.x;
                coords.position.y = end.y;
                transform.translation.x = coords.position.x as f32 + coords.dimensions.x as f32 * 0.5;
                transform.translation.y = coords.position.y as f32 + coords.dimensions.y as f32 * 0.5;
            }
        }
    }
}
