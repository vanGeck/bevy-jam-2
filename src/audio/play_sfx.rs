use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::audio::sound_event::{MusicChannel, SfxChannel, SoundEvent};
use crate::config::config_audio::AudioConfig;
use crate::game::AssetStorage;

/// This system is responsible for playing sound effects.
/// To play any sound effect, just broadcast a `SoundEvent` in the corresponding event channel.
/// This system will take care of the rest.
pub fn play_sfx(
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
            SoundEvent::Music(Some((music_type, looped))) => {
                if let Some(handle) = assets.music(music_type) {
                    channel_music.stop();
                    if *looped {
                        // TODO: looping.
                        channel_music.play(handle);
                    } else {
                        channel_music.play(handle);
                    }
                } else {
                    info!(
                        "Tried to play MusicType::{:?} but couldn't find that asset.",
                        music_type
                    );
                }
            }
            SoundEvent::Music(None) => {
                channel_music.stop();
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
