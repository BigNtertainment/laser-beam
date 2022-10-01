use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use crate::WALL_SIZE;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::GAME_AREA_HEIGHT;
use crate::GAME_AREA_WIDTH;

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
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..Default::default()
        })
        .insert(Player)
        .insert(Name::new("Player"));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &Handle<Image>), With<Player>>,
    images: Res<Assets<Image>>,
    mut lines: ResMut<DebugLines>,
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
    for (mut player_transform, texture) in &mut player_query {
        player_transform.translation += movement;

        // Keep the player in the game area
        let texture_size = images.get(texture).unwrap().texture_descriptor.size;

        let player_size = Vec2::new(
            texture_size.width as f32 * player_transform.scale.x,
            texture_size.height as f32 * player_transform.scale.y,
        );

        let bounding_box = Vec2::new(
            GAME_AREA_WIDTH - WALL_SIZE * 2.0 - player_size.x,
            GAME_AREA_HEIGHT - WALL_SIZE * 2.0 - player_size.y,
        );

        lines.line(
            (Vec2::new(1., 1.) * bounding_box / 2.).extend(0.0),
            (Vec2::new(1., -1.) * bounding_box / 2.).extend(0.0),
            0.,
        );
        lines.line(
            (Vec2::new(1., -1.) * bounding_box / 2.).extend(0.0),
            (Vec2::new(-1., -1.) * bounding_box / 2.).extend(0.0),
            0.,
        );
        lines.line(
            (Vec2::new(-1., -1.) * bounding_box / 2.).extend(0.0),
            (Vec2::new(-1., 1.) * bounding_box / 2.).extend(0.0),
            0.,
        );
        lines.line(
            (Vec2::new(-1., 1.) * bounding_box / 2.).extend(0.0),
            (Vec2::new(1., 1.) * bounding_box / 2.).extend(0.0),
            0.,
        );

        player_transform.translation.x = player_transform
            .translation
            .x
            .clamp(-bounding_box.x / 2.0, bounding_box.x / 2.0);
        player_transform.translation.y = player_transform
            .translation
            .y
            .clamp(-bounding_box.y / 2.0, bounding_box.y / 2.0);
    }
}

fn camera_follow(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    windows: Res<Windows>,
) {
    let player = player.single();
    let mut camera = camera.iter_mut().next().unwrap();
    let window = windows.primary();

    let bounding_box = Vec2::new(
        GAME_AREA_WIDTH - window.width(),
        GAME_AREA_HEIGHT - window.height(),
    );

    camera.translation.x = player
        .translation
        .x
        .clamp(-bounding_box.x / 2.0, bounding_box.x / 2.0);
    camera.translation.y = player
        .translation
        .y
        .clamp(-bounding_box.y / 2.0, bounding_box.y / 2.0);
}
