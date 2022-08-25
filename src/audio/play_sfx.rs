use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl, PlaybackState};

use crate::audio::sound_event::{
    AudioResource, CurrentTrack, MusicChannel, SfxChannel, SoundEvent,
};
use crate::config::config_audio::AudioConfig;
use crate::game::AssetStorage;

/// This system is responsible for playing sound effects.
/// To play any sound effect, just broadcast a `SoundEvent` in the corresponding event channel.
/// This system will take care of the rest.
pub fn play_sfx(
    mut resource: ResMut<AudioResource>,
    mut events: EventReader<SoundEvent>,
    assets: Res<AssetStorage>,
    channel_music: Res<AudioChannel<MusicChannel>>,
    channel_sfx: Res<AudioChannel<SfxChannel>>,
) {
    for event in events.iter() {
        debug!("Received sound event: {:?}", event);
        match event {
            SoundEvent::KillAllSoundEffects => {
                channel_sfx.stop();
            }
            SoundEvent::KillAllMusic => {
                channel_music.stop();
            }
            SoundEvent::Sfx(sound_type) => {
                if let Some(handle) = assets.sound(sound_type) {
                    channel_sfx.play(handle);
                } else {
                    info!(
                        "Tried to play SoundType::{:?} but couldn't find that asset.",
                        sound_type
                    );
                }
            }
            SoundEvent::Music(album) => {
                let next_track = if let Some(current) = &resource.current {
                    if current.album == *album {
                        (current.track + 1).rem_euclid(assets.album_len(album))
                    } else {
                        0
                    }
                } else {
                    0
                };
                if let Some(handle) = assets.album_track(album, next_track) {
                    channel_music.stop();
                    let instance = channel_music.play(handle).handle();
                    resource.current = Some(CurrentTrack {
                        album: *album,
                        track: next_track,
                        instance: instance,
                    });
                    resource.skipping = false;
                } else {
                    info!(
                        "Tried to play MusicType::{:?} but couldn't find that asset.",
                        album
                    );
                }
            }
        };
    }
}

pub fn change_audio_settings(
    config: Res<AudioConfig>,
    channel_music: Res<AudioChannel<MusicChannel>>,
    channel_sfx: Res<AudioChannel<SfxChannel>>,
) {
    if config.is_changed() {
        channel_music.set_volume(config.music_volume.unwrap_or(0.));
        channel_sfx.set_volume(config.sfx_volume.unwrap_or(0.));
    }
}

pub fn skip_to_next_song(
    mut audio: EventWriter<SoundEvent>,
    channel: Res<AudioChannel<MusicChannel>>,
    mut resource: ResMut<AudioResource>,
) {
    if resource.skipping {
        return;
    }
    if let Some(current) = &resource.current {
        let state = channel.state(&current.instance);
        if state == PlaybackState::Stopped {
            audio.send(SoundEvent::Music(current.album));
            resource.skipping = true;
        }
    }
}
