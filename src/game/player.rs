use crate::game::assets::AssetHandles;
use crate::game::CleanupOnGameplayEnd;
use crate::{input, CollisionLayers, PhysLayer, Quat, SpriteBundle, Transform};
use bevy::math::Vec3;
use bevy::prelude::{
    Color, Commands, Component, ImageBundle, Query, Res, Sprite, Time, Vec2, With,
};
use heron::{CollisionShape, PhysicsLayer, RigidBody};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct RotatePlayerAroundSelfPlaceholder {
    pub rad_per_sec: f32,
}

pub fn spawn_player(mut cmd: Commands, assets: Res<AssetHandles>) {
    let player_transform = Transform::from_translation(Vec3::ZERO);

    let dimensions = Vec2::new(256.0, 256.0);

    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(dimensions),
            ..Default::default()
        },
        transform: player_transform,
        texture: assets.placeholder.clone(),
        ..Default::default()
    })
    .insert(Player)
    .insert(CleanupOnGameplayEnd)
    .insert(input::Draggable)
    .insert(RigidBody::Sensor)
    .insert(CollisionLayers::new(
        PhysLayer::Draggables,
        PhysLayer::World,
    ))
    // Collider dimensions match texture dimensions (halved)
    .insert(CollisionShape::Cuboid {
        half_extends: (dimensions * 0.5).extend(1.0),
        border_radius: None,
    })
    .insert(RotatePlayerAroundSelfPlaceholder { rad_per_sec: 0.61 });
}

pub fn rotate_player_placeholder(
    mut cmd: Commands,
    mut q: Query<(&mut Transform, &RotatePlayerAroundSelfPlaceholder)>,
    time: Res<Time>,
) {
    let (mut tform, mut rotate) = q.single_mut();
    tform.rotation =
        tform.rotation * Quat::from_axis_angle(Vec3::Z, rotate.rad_per_sec * time.delta_seconds());
}
