use bevy::prelude::*;

use crate::animation::AnimationTimer;

pub fn animate(time: Res<Time>, mut query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer)>) {
    for (mut sprite, mut anim) in query.iter_mut() {
        sprite.index = anim.tick(time.delta());
    }
}
