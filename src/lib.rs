mod actions;
mod audio;
mod camera;
mod character;
mod debug;
mod enemy;
mod game_area;
mod game_over;
mod loading;
mod menu;
mod player;
mod score;
mod shaders;
mod ui;
mod weapon;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::enemy::EnemyPlugin;
use crate::game_area::GameAreaPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::ui::HealthBarPlugin;
use bevy::app::App;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::CameraPlugin;
use character::HealthPlugin;
use debug::DebugPlugin;
use game_over::GameOver;
use score::ScorePlugin;
use shaders::ShaderPlugin;
use weapon::WeaponPlugin;

pub const GAME_AREA_WIDTH: f32 = 1000.0;
pub const GAME_AREA_HEIGHT: f32 = 800.0;

pub const WALL_WIDTH: f32 = 25.0;
pub const WALL_HEIGHT: f32 = 25.0;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // Game over screen ¯\_(ツ)_/¯
    GameOver,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(DebugPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(GameOver)
            .add_plugin(ShaderPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(GameAreaPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(HealthPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(WeaponPlugin)
            .add_plugin(HealthBarPlugin)
            .add_plugin(EnemyPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
