use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::GameState;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin).add_system_set(SystemSet::on_exit(GameState::Playing).with_system(stop_sound));
    }
}

fn stop_sound(audio: Res<Audio>) {
    audio.stop();
}
