use super::{components::Item, PhysLayer};
use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, RigidBody};

pub fn spawn_item(commands: &mut Commands, item: Item, texture: Handle<Image>) {
    let item_id = commands.spawn().id();

    commands
        .entity(item_id)
        .insert(Name::new(item.clone().name))
        .insert(item.clone())
        .insert(RigidBody::Sensor)
        .insert(CollisionLayers::new(
            PhysLayer::Draggables,
            PhysLayer::World,
        ))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(
                (item.coords.dimens.x * 10) as f32,
                (item.coords.dimens.y * 10) as f32,
                1.,
            ), // Item dimens * 10 there's probably a better way
            border_radius: None,
        })
        .insert_bundle(SpriteBundle {
            texture,
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        });
}
