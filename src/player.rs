use crate::actions::Actions;
use crate::camera::MainCamera;
use crate::character::Health;
use crate::loading::TextureAssets;
use crate::GameState;
use crate::WALL_SIZE;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::GAME_AREA_HEIGHT;
use crate::GAME_AREA_WIDTH;
use bevy::math::Vec3Swizzles;
use std::f32::consts::PI;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    move_speed: f32,
    rotation_speed: f32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    sprite_budle: SpriteBundle,
    name: Name,
    player: Player,
    health: Health,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_player.label("player_movement"))
                    .with_system(aim_player.after("player_movement"))
                    .with_system(camera_follow.after("player_movement"))
                    .with_system(check_if_dead),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing)
                .with_system(drop_player),
            );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn_bundle(PlayerBundle {
        sprite_budle: SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        },
        name: Name::new("Player"),
        player: Player {
            move_speed: 150.,
            rotation_speed: 1.,
        },
        health: Health::new(100.0),
    });
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &Handle<Image>, &Player)>,
    images: Res<Assets<Image>>,
    mut lines: ResMut<DebugLines>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    for (mut player_transform, texture, player) in &mut player_query {
        let movement = Vec3::new(
            actions.player_movement.unwrap().x * player.move_speed * time.delta_seconds(),
            actions.player_movement.unwrap().y * player.move_speed * time.delta_seconds(),
            0.,
        );

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

fn aim_player(
    time: Res<Time>,
    windows: Res<Windows>,
    mut player_query: Query<(&mut Transform, &Player)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = windows.primary();
    let (camera, camera_transform) = camera_query.single();

    let (mut player_transform, player) = player_query.single_mut();
    let player_translation = player_transform.translation.xy();

    if let Some(cursor_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // get world cursor position
        let ndc = (cursor_position / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let cursor_world_position = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();

        // rotate player to cursor
        let to_cursor = (cursor_world_position - player_translation).normalize();
        let target_rotation = Quat::from_rotation_arc(Vec3::Y, to_cursor.extend(0.));
        let target_rotation_z =
            (Quat::from_rotation_arc(Vec3::Y, to_cursor.extend(0.)).z + 1.) / 2.;

        // TODO: All of the code below can be optimized and simplified

        let player_rotation = (player_transform.rotation.z + 1.) / 2.;
        let diff = player_rotation - target_rotation_z;

        if diff.abs() < 0.001 {
            player_transform.rotation = target_rotation;
            return;
        }

        let rotation_amount = time.delta_seconds() * player.rotation_speed;

        if diff > 0. {
            if diff < 0.5 {
                player_transform.rotate_z(-rotation_amount);
            } else {
                let old_rotation = player_transform.rotation;
                player_transform.rotate_z(rotation_amount);

                if player_transform.rotation.z < old_rotation.z {
                    player_transform.rotation = Quat::from_rotation_z(-PI);
                }
            }
        } else {
            if 1. + diff < 0.5 {
                let old_rotation = player_transform.rotation;
                player_transform.rotate_z(-rotation_amount);

                if player_transform.rotation.z > old_rotation.z {
                    player_transform.rotation = Quat::from_rotation_z(PI);
                }
            } else {
                player_transform.rotate_z(rotation_amount);
            }
        }
    }
}

fn check_if_dead(player: Query<&Health, With<Player>>, mut state: ResMut<State<GameState>>) {
    let player = player.single();

    if player.get_health() <= 0.0 {
        state.set(GameState::GameOver).unwrap();
    }
}

fn drop_player(mut commands: Commands, player: Query<Entity, With<Player>>) {
    commands.entity(player.single()).despawn_recursive();
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
