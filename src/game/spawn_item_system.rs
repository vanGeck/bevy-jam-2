use bevy::prelude::*;

use crate::game::items::Item;
use crate::game::{AssetStorage, CleanupOnGameplayEnd};
use crate::mouse::MouseInteractive;
use crate::positioning::{Coords, GridData};
use crate::positioning::{Depth, Dimens, Pos};

/// Broadcast this as an event to spawn an item.
#[derive(Debug)]
pub struct SpawnItemEvent {
    item: Item,
    coords: Coords,
}

impl SpawnItemEvent {
    pub fn new(item: Item, coords: Coords) -> Self {
        SpawnItemEvent { item, coords }
    }
}

pub fn spawn_item(
    mut commands: Commands,
    mut events: EventReader<SpawnItemEvent>,
    assets: Res<AssetStorage>,
    grid: Res<GridData>,
) {
    for SpawnItemEvent { item, coords } in events.iter() {
        trace!("Received SpawnItemEvent( {:?}, {:?} )", item, coords);

        commands
            .spawn_bundle(SpriteBundle {
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
    }
}

pub fn find_free_space(
    grid: &GridData,
    dimens: Dimens,
    items_query: &Query<&Coords, With<Item>>, // is there any way to call this function without this query? it forces you to have the exact same query in whichever query you're calling this function from. - Jacques
    same_tick_items: &[Coords], // Pass this an emtpy vec if not multiple spawn
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
