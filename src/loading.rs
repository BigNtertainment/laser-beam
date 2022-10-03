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
    #[asset(path = "audio/footsteps", collection(typed))]
    pub footsteps: Vec<Handle<AudioSource>>,
    #[asset(path = "audio/growls", collection(typed))]
    pub growls: Vec<Handle<AudioSource>>,
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
