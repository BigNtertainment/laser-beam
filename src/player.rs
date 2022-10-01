use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

use crate::GAME_AREA_WIDTH;
use crate::GAME_AREA_HEIGHT;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_player.label("player_movement"))
                    .with_system(camera_follow.after("player_movement")),
            );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(Name::new("Player"));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}

fn camera_follow(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player = player.single();
    let mut camera = camera.iter_mut().next().unwrap();

    camera.translation.x = player.translation.x.clamp(-GAME_AREA_WIDTH / 2.0, GAME_AREA_WIDTH / 2.0);
    camera.translation.y = player.translation.y.clamp(-GAME_AREA_HEIGHT / 2.0, GAME_AREA_HEIGHT / 2.0);
}
