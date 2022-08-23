use crate::game::{MusicType, SoundType};
use bevy::prelude::*;

/// Elsewhere in the application, you can broadcast `SoundEvents`. The `PlaySfxSystem` below listens
/// for such events and actually plays the sound effect that was requested.
#[allow(dead_code)]
#[derive(Debug)]
pub enum SoundEvent {
    /// SoundType and whether the music should be interrupted during play.
    Sfx(SoundType),
    /// MusicType. If present, play this song. Otherwise, stop all music.
    Music(Option<(MusicType, Looped)>),
    KillAllSoundEffects,
    KillAllMusic,
}

pub type Looped = bool;

#[derive(Component, Default, Clone)]
pub struct MusicChannel;

#[derive(Component, Default, Clone)]
pub struct SfxChannel;

#[derive(Default)]
pub struct AudioResource {}
