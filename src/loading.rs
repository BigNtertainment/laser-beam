use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .continue_to_state(GameState::Menu),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
    #[asset(
        paths(
            "audio/footsteps/footstep_1.wav",
            "audio/footsteps/footstep_2.wav",
            "audio/footsteps/footstep_3.wav",
            "audio/footsteps/footstep_4.wav"
        ),
        collection(typed)
    )]
    pub footsteps: Vec<Handle<AudioSource>>,
    #[asset(
        paths(
            "audio/growls/growl_1.wav",
            "audio/growls/growl_2.wav",
            "audio/growls/growl_3.wav",
            "audio/growls/growl_4.wav",
            "audio/growls/growl_5.wav"
        ),
        collection(typed)
    )]
    pub growls: Vec<Handle<AudioSource>>,
    #[asset(
        paths("audio/attacks/attack_1.wav", "audio/attacks/attack_2.wav"),
        collection(typed)
    )]
    pub attacks: Vec<Handle<AudioSource>>,
    #[asset(path = "audio/laser/laser.wav")]
    pub laser: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/player.png")]
    pub player_texture: Handle<Image>,
    #[asset(path = "textures/wall.png")]
    pub wall_texture: Handle<Image>,
    #[asset(path = "textures/enemy.png")]
    pub enemy_texture: Handle<Image>,
    #[asset(path = "textures/window.png")]
    pub enemy_spawn_texture: Handle<Image>,
    #[asset(path = "textures/laser.png")]
    pub laser_texture: Handle<Image>,
    #[asset(path = "textures/laser-end.png")]
    pub laser_end_texture: Handle<Image>,
    #[asset(path = "textures/floor.png")]
    pub floor_texture: Handle<Image>,
}
