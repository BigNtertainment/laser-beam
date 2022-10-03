use crate::actions::Actions;
use crate::camera::MainCamera;
use crate::character::{Health, Movement, Rotation};
use crate::loading::TextureAssets;
use crate::weapon::{Weapon, WeaponBundle};
use crate::GameState;
use crate::{WALL_HEIGHT, WALL_WIDTH};
use bevy::prelude::*;

use crate::GAME_AREA_HEIGHT;
use crate::GAME_AREA_WIDTH;
use bevy::math::Vec3Swizzles;
use std::f32::consts::PI;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    name: Name,
    player: Player,
    health: Health,
    movement: Movement,
    rotation: Rotation,
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
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(drop_player));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(PlayerBundle {
            sprite_bundle: SpriteBundle {
                texture: textures.player_texture.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 5.))
                    .with_scale(Vec3::new(0.25, -0.25, 1.)),
                ..default()
            },
            name: Name::new("Player"),
            player: Player,
            health: Health::new(100.0),
            movement: Movement { speed: 100. },
            rotation: Rotation {
                rotation_speed: 1.5,
            },
        })
        .with_children(|parent| {
            parent.spawn_bundle(WeaponBundle {
                weapon: Weapon::default(),
            });
        });
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &Handle<Image>, &Movement), With<Player>>,
    images: Res<Assets<Image>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    for (mut player_transform, texture, player_movement) in &mut player_query {
        let movement = Vec3::new(
            actions.player_movement.unwrap().x * player_movement.speed * time.delta_seconds(),
            actions.player_movement.unwrap().y * player_movement.speed * time.delta_seconds(),
            0.,
        );

        player_transform.translation += movement;

        // Keep the player in the game area
        let texture_size = images.get(texture).unwrap().texture_descriptor.size;

        let player_size = Vec2::new(
            texture_size.width as f32 * player_transform.scale.x.abs(),
            texture_size.height as f32 as f32 * player_transform.scale.y.abs(),
        );

        let game_area = Vec2::new(
            GAME_AREA_WIDTH - WALL_WIDTH * 2.0,
            GAME_AREA_HEIGHT - WALL_HEIGHT * 2.0,
        );

        let bounding_box = Vec2::new(game_area.x - player_size.x, game_area.y - player_size.y);

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
    mut player_query: Query<(&mut Transform, &Rotation), With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = windows.primary();
    let (camera, camera_transform) = camera_query.single();

    let (mut player_transform, rotation) = player_query.single_mut();
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

        let rotation_amount = time.delta_seconds() * rotation.rotation_speed;

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
    let mut camera_transform = camera.iter_mut().next().unwrap();
    let window = windows.primary();

    let bounding_box = Vec2::new(
        GAME_AREA_WIDTH - window.width(),
        GAME_AREA_HEIGHT - window.height(),
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
