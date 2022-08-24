use crate::audio::sound_event::SoundEvent;
use crate::game::MusicId;
use bevy::prelude::*;

/// === Systems ===
pub fn setup_audio(mut audio: EventWriter<SoundEvent>) {
    audio.send(SoundEvent::Music(Some((MusicId::Placeholder, false))));
}
