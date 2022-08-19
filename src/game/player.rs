use bevy::math::Vec3;
use bevy::prelude::{Commands, ImageBundle, Query, Res, Sprite, Time, Vec2, With, Component, Color};
use crate::{Quat, SpriteBundle, Transform};
use crate::game::assets::AssetHandles;
use crate::game::CleanupOnGameplayEnd;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct RotatePlayerAroundSelfPlaceholder {
    pub rad_per_sec: f32
}

pub fn spawn_player(mut cmd: Commands, assets: Res<AssetHandles>){
    let player_tform = Transform::from_translation(Vec3::ZERO);
    
    cmd.spawn_bundle(SpriteBundle{
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(256.0, 256.0)),
            ..Default::default()
        },
        transform: player_tform,
        texture: assets.placeholder.clone(),
        ..Default::default()
    })
        .insert(Player)
        .insert(CleanupOnGameplayEnd)
        .insert(RotatePlayerAroundSelfPlaceholder{
            rad_per_sec: 0.61
        });
}

pub fn rotate_player_placeholder(mut cmd: Commands, mut q: Query<(&mut Transform, &RotatePlayerAroundSelfPlaceholder)>, time: Res<Time>){
    let (mut tform, mut rotate) = q.single_mut();
    tform.rotation = tform.rotation * Quat::from_axis_angle(Vec3::Z, rotate.rad_per_sec * time.delta_seconds());
}