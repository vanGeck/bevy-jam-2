use super::{components::Grid, components::Shape, AssetHandles, PhysLayer};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use heron::{CollisionLayers, CollisionShape, PhysicsLayer, RigidBody};

pub fn create_items_system(
    mut commands: Commands,
    assets: Res<AssetHandles>,
    mut query: Query<&mut Grid>,
) {
    let mut shape = commands.spawn().id();

    commands
        .entity(shape)
        .insert(Name::new("Shape_Croissant"))
        .insert(Shape {
            width: 3,
            height: 2,
            // This truth table is the shape of the croissant sprite.
            occupied: vec![vec![true, true, true], vec![true, false, true]],
        })
        .insert(RigidBody::Sensor)
        .insert(CollisionLayers::new(
            PhysLayer::Draggables,
            PhysLayer::World,
        ))
        // Collider dimensions match texture dimensions (halved)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(48., 32., 1.),
            border_radius: None,
        })
        .insert_bundle(SpriteBundle {
            texture: assets.three_x_two_croissant.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        });
}
