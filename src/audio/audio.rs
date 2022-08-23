use bevy::prelude::*;
use crate::audio::sound_event::SoundEvent;
use crate::game::MusicId;

/// === Systems ===
pub fn setup_audio(mut audio: EventWriter<SoundEvent>) {
    audio.send(SoundEvent::Music(Some((MusicId::Placeholder, false))));
}