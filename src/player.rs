use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use crate::{WALL_WIDTH, WALL_HEIGHT};
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
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)).with_scale(Vec3::new(0.1, 0.1, 1.)),
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

        let game_area = Vec2::new(
            GAME_AREA_WIDTH - WALL_WIDTH * 2.0,
            GAME_AREA_HEIGHT - WALL_HEIGHT * 2.0,
        );

        let bounding_box = Vec2::new(
            game_area.x - player_size.x,
            game_area.y - player_size.y,
        );

        lines.line(
            (Vec2::new(1., 1.) * game_area / 2.).extend(0.0),
            (Vec2::new(1., -1.) * game_area / 2.).extend(0.0),
            0.,
        );
        lines.line(
            (Vec2::new(1., -1.) * game_area / 2.).extend(0.0),
            (Vec2::new(-1., -1.) * game_area / 2.).extend(0.0),
            0.,
        );
        lines.line(
            (Vec2::new(-1., -1.) * game_area / 2.).extend(0.0),
            (Vec2::new(-1., 1.) * game_area / 2.).extend(0.0),
            0.,
        );
        lines.line(
            (Vec2::new(-1., 1.) * game_area / 2.).extend(0.0),
            (Vec2::new(1., 1.) * game_area / 2.).extend(0.0),
            0.,
        );

        lines.line(
            (Vec2::new(0., 1.) * game_area / 2.).extend(0.0),
            (Vec2::new(0., 1.) * game_area / 2.).extend(0.0) + Vec3::new(0., WALL_HEIGHT, 0.),
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
    mut camera: Query<(&mut Transform, &Camera), Without<Player>>,
    mut lines: ResMut<DebugLines>,
) {
    let player = player.single();
    let (mut camera_transform, camera) = camera.iter_mut().next().unwrap();

    let viewport = camera.logical_viewport_size().unwrap();

    let bounding_box = Vec2::new(
        GAME_AREA_WIDTH - viewport.x as f32 / 2.,
        GAME_AREA_HEIGHT - viewport.y as f32 / 2.,
    );

    println!("{:?}", viewport);

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

    camera_transform.translation.x = if bounding_box.x >= 0. {
        player
            .translation
            .x
            .clamp(-bounding_box.x / 2.0, bounding_box.x / 2.0)
    } else {
        // If the screen is wider than the play area, keep the camera centered
        0.
    };

    camera_transform.translation.y = if bounding_box.y >= 0. {
        player
            .translation
            .y
            .clamp(-bounding_box.y / 2.0, bounding_box.y / 2.0)
    } else {
        // If the screen is higher than the play area, keep the camera centered
        0.
    };
}
