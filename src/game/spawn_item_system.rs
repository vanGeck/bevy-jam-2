use bevy::prelude::*;

use crate::game::{AssetStorage, CleanupOnGameplayEnd, FallingItem, Silhouette};
use crate::game::items::Item;
use crate::mouse::MouseInteractive;
use crate::positioning::{Coords, GridData};
use crate::positioning::{Depth, Dimens, Pos};

/// Broadcast this as an event to spawn an item.
#[derive(Debug)]
pub struct SpawnItemEvent {
    item: Item,
    coords: Coords,
    /// If it spawns as an animated FallingItem, where does it appear?
    ///
    /// Set to to None for any items that are present at the start of the game. They will spawn
    /// in the inventory without any animations.
    source: Option<Vec2>,
    combine: bool,
}

impl SpawnItemEvent {
    pub fn new(item: Item, coords: Coords, source: Vec2, combine: bool) -> Self {
        SpawnItemEvent {
            item,
            coords,
            source: Some(source),
            combine,
        }
    }
    /// Use this for items that already exist in the backpack at the start of the game.
    pub fn without_anim(item: Item, coords: Coords) -> Self {
        SpawnItemEvent {
            item,
            coords,
            source: None,
            combine: false,
        }
    }
}

pub fn spawn_item(
    mut commands: Commands,
    mut events: EventReader<SpawnItemEvent>,
    assets: Res<AssetStorage>,
    grid: Res<GridData>,
) {
    for SpawnItemEvent {
        item,
        coords,
        source,
        combine,
    } in events.iter()
    {
        trace!("Received SpawnItemEvent( {:?}, {:?} )", item, coords);
        if let Some(source) = source {
            // Spawn the animating item.
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(coords.dimens.as_vec2()),
                        ..default()
                    },
                    texture: assets.texture(&item.texture_id),
                    transform: Transform::from_xyz(source.x, source.y, Depth::FloatingItem.z()),
                    ..Default::default()
                })
                .insert(Name::new("FallingItem"))
                .insert(FallingItem::new(
                    *coords,
                    *source,
                    coords.pos.as_vec2() + grid.offset,
                    if *combine { 0.75 } else { 1.25 },
                ))
                .insert(CleanupOnGameplayEnd);
        }
        // Spawn the silhouette.
        let mut builder = commands.spawn();
        builder
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(coords.dimens.as_vec2()),
                    ..default()
                },
                texture: assets.texture(&item.texture_id),
                transform: Transform::from_xyz(
                    grid.offset.x + coords.pos.x as f32 + coords.dimens.x as f32 * 0.5,
                    grid.offset.y + coords.pos.y as f32 + coords.dimens.y as f32 * 0.5,
                    Depth::Item.z(),
                ),
                ..Default::default()
            })
            .insert(Name::new(item.name.clone()))
            .insert(item.clone())
            .insert(*coords)
            .insert(MouseInteractive::new(coords.dimens.as_vec2(), true))
            .insert(CleanupOnGameplayEnd);
        if source.is_some() {
            builder.insert(Silhouette);
        }
    }
}

pub fn animate_falling_item(
    mut commands: Commands,
    time: Res<Time>,
    mut query_falling: Query<(Entity, &mut FallingItem, &mut Transform)>,
    query_cleanup: Query<(Entity, &Coords), With<Silhouette>>,
) {
    for (entity, mut item, mut transform) in query_falling.iter_mut() {
        item.timer.tick(time.delta());
        if item.timer.finished() {
            commands.entity(entity).despawn_recursive();
            if let Some((silhouette_entity, _)) = query_cleanup
                .iter()
                .find(|(_, coords)| **coords == item.coords)
            {
                commands.entity(silhouette_entity).remove::<Silhouette>();
            }
        } else {
            let progress = item.timer.percent().powi(2);
            let delta_total = item.target - item.source;
            let delta_current = delta_total * progress;
            let current_pos = delta_current + item.source;
            transform.translation.x = current_pos.x + item.coords.dimens.x as f32 * 0.5;
            transform.translation.y = current_pos.y + item.coords.dimens.y as f32 * 0.5;
            transform.scale.x = 1. + (1. - progress);
            transform.scale.y = 1. + (1. - progress);
        }
    }
}

pub fn find_free_space(
    grid: &GridData,
    dimens: Dimens,
    items_query: &Query<&Coords, With<Item>>, // is there any way to call this function without this query? it forces you to have the exact same query in whichever query you're calling this function from. - Jacques
    same_tick_items: &[Coords],               // Pass this an emtpy vec if not multiple spawn
) -> Option<Coords> {
    for y in 0..grid.inventory.dimens.y {
        for x in 0..grid.inventory.dimens.x {
            let coords = Coords {
                pos: Pos::new(x, y),
                dimens,
            };

            let overlap_conflict = items_query.iter().any(|item| coords.overlaps(item))
                || same_tick_items.iter().any(|item| coords.overlaps(item));
            let bound_conflict = !grid.inventory.encloses(&coords);
            if !overlap_conflict && !bound_conflict {
                return Some(coords);
            }
        }
    }
    None
}
