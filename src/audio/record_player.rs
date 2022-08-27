use bevy::prelude::*;

use crate::animation::AnimationTimer;
use crate::audio::sound_event::SoundEvent;
use crate::game::AlbumId;
use crate::mouse::MouseInteractive;

// TODO: Move to different package.
pub fn animate(time: Res<Time>, mut query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer)>) {
    for (mut sprite, mut anim) in query.iter_mut() {
        sprite.index = anim.tick(time.delta());
    }
}

#[derive(Component)]
pub struct RecordPlayer;

pub fn check_record_player_input(
    mut audio: EventWriter<SoundEvent>,
    query: Query<&MouseInteractive, With<RecordPlayer>>,
) {
    if let Ok(interactive) = query.get_single() {
        if interactive.clicked {
            audio.send(SoundEvent::NextTrack(AlbumId::Jazz));
        }
    }
}