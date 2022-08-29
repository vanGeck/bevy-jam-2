use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioPlugin};
use iyes_loopless::condition::ConditionSet;

use crate::audio::play_audio::{change_audio_settings, play_sfx, skip_to_next_song};
use crate::audio::record_player::check_record_player_input;
use crate::audio::sound_event::{AudioResource, MusicChannel, SfxChannel, SoundEvent};
use crate::AppState;

pub struct MyAudioPlugin;

impl Plugin for MyAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_event::<SoundEvent>()
            .add_audio_channel::<MusicChannel>()
            .add_audio_channel::<SfxChannel>()
            .init_resource::<AudioResource>()
            .add_system_set(
                ConditionSet::new()
                    .run_not_in_state(AppState::Loading)
                    .with_system(play_sfx)
                    .with_system(change_audio_settings)
                    .with_system(check_record_player_input)
                    .with_system(skip_to_next_song)
                    .into(),
            );
    }
}
