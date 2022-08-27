use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;

use crate::game::{AlbumId, SoundId};

/// Elsewhere in the application, you can broadcast `SoundEvents`. The `PlaySfxSystem` below listens
/// for such events and actually plays the sound effect that was requested.
#[allow(dead_code)]
#[derive(Debug)]
pub enum SoundEvent {
    /// SoundType and whether the music should be interrupted during play.
    Sfx(SoundId),
    /// Whatever song happens to be playing, stop it. Play songs from the given album.
    /// If the album was already playing, play the next song.
    NextTrack(AlbumId),
    /// Start playing songs from the given album. If the album is already playing, do nothing.
    PlayAlbum(AlbumId),
    KillAllSoundEffects,
    KillAllMusic,
}

#[derive(Component, Default, Clone)]
pub struct MusicChannel;

#[derive(Component, Default, Clone)]
pub struct SfxChannel;

#[derive(Default)]
pub struct AudioResource {
    /// If the music player is currently playing a song, its details are stored here.
    /// Otherwise this is None.
    pub current: Option<CurrentTrack>,
    /// This is set to true when the system is in the process of skipping to the next song.
    /// Basically, this is to prevent skipping more than 1 song at a time.
    pub skipping: bool,
}

pub struct CurrentTrack {
    /// An asset id to the vec of music tracks (colloquially referred to as 'album') that is
    /// currently being played.
    pub album: AlbumId,
    /// The track number. This is the index of the vec of audio handles in the AssetStorage.
    pub track: usize,
    /// A kira thing. This is how we track the status of the current playback.
    pub instance: Handle<AudioInstance>,
}

#[derive(Component)]
pub struct AudioTextDisplay;
