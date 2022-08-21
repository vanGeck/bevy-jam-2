use bevy::prelude::*;

use crate::game::{AssetStorage, CleanupOnGameplayEnd, Item, SpriteType};
use crate::input::Mouse;
use crate::positioning::coords::Coords;
use crate::positioning::depth::Depth;
use crate::positioning::pos::Pos;

/// Broadcast this event when completing a dragging operation.
/// The entity that is being dragged still has the BeingDragged component.
#[derive(Debug)]
pub struct DragEvent {
    start: Pos,
    end: Pos,
}

// TODO: Use this??
#[derive(Component)]
pub struct Draggable;

/// This marker component is added to items that are currently being dragged.
#[derive(Component)]
pub struct BeingDragged;

/// Marker component for the entity that spawns when dragging an item.
/// The original item stays in its place, the ghost indicates where the item will end up.
#[derive(Component)]
pub struct DragGhost;

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
    mut mouse: ResMut<Mouse>,
    query: Query<(&Coords, Entity), With<Item>>,
) {
    if mouse.is_dragging || !input.just_pressed(MouseButton::Left) {
        return;
    }
    let cell = Pos::from(mouse.position);
    for (coords, entity) in query.iter() {
        if coords.overlaps_pos(&cell) {
            commands.entity(entity).insert(BeingDragged);
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1., 1., 1., 0.5),
                        custom_size: Some(coords.dimens.as_vec2()),
                        ..default()
                    },
                    texture: assets.texture(&SpriteType::Croissant),
                    transform: Transform::from_xyz(
                        coords.pos.x as f32 + coords.dimens.x as f32 * 0.5,
                        coords.pos.y as f32 + coords.dimens.y as f32 * 0.5,
                        Depth::FloatingItem.z(),
                    )
                    .with_scale(Vec3::new(1.1, 1.1, 1.)),
                    ..Default::default()
                })
                .insert(coords.clone())
                .insert(DragGhost)
                .insert(CleanupOnGameplayEnd);
            mouse.is_dragging = true;
        }
    }
}

// TODO: if it goes out of bounds:
//  - The item ghost just disappears, but dragging continues. If it is let go at that point, the drag simply fails.

/// Move the item ghost with the mouse, but in discrete increments, always snapping to the grid.
pub fn set_ghost_position(
    mouse: Res<Mouse>,
    mut query: Query<(&mut Transform, &mut Coords), With<DragGhost>>,
) {
    if let Ok((mut transform, mut coords)) = query.get_single_mut() {
        coords.pos = Pos::from(mouse.position);
        transform.translation.x = coords.pos.x as f32 + coords.dimens.x as f32 * 0.5;
        transform.translation.y = coords.pos.y as f32 + coords.dimens.y as f32 * 0.5;
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

/// Check if the dragging operation should be concluded. If so;
/// - Delete the ghost entity.
/// - Mark the mouse as no longer in the middle of a drag operation.
/// - Broadcast a DragEvent.
pub fn check_drag_end(
    mut commands: Commands,
    mut writer: EventWriter<DragEvent>,
    mut mouse: ResMut<Mouse>,
    input: Res<Input<MouseButton>>,
    query_item: Query<&Coords, With<BeingDragged>>,
    query_ghost: Query<(Entity, &Coords), With<DragGhost>>,
) {
    if !mouse.is_dragging || !input.just_released(MouseButton::Left) {
        return;
    }
    mouse.is_dragging = false;
    let (ghost_entity, ghost_coords) = query_ghost.single();
    let item_coords = query_item.single();
    writer.send(DragEvent {
        start: item_coords.pos,
        end: ghost_coords.pos,
    });
    commands.entity(ghost_entity).despawn_recursive();
}

pub fn process_drag_event(
    mut commands: Commands,
    mut events: EventReader<DragEvent>,
    mut query_item: Query<(Entity, &mut Transform, &mut Coords), With<BeingDragged>>,
) {
    for event in events.iter() {
        debug!("Received drag item event: {:?}", event);
        if let DragEvent { start, end } = event {
            if let Ok((entity, mut transform, mut coords)) = query_item.get_single_mut() {
                commands.entity(entity).remove::<BeingDragged>();
                coords.pos.x = end.x;
                coords.pos.y = end.y;
                transform.translation.x = coords.pos.x as f32 + coords.dimens.x as f32 * 0.5;
                transform.translation.y = coords.pos.y as f32 + coords.dimens.y as f32 * 0.5;
            }
        }
    }
}
