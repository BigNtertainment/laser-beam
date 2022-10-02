use std::time::Duration;

use crate::game_area::EnemySpawn;
use crate::loading::TextureAssets;
use crate::{
    character::{Health, Movement},
    player::Player,
};
use crate::{GameState, WALL_WIDTH};
use bevy::math::Vec3Swizzles;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::seq::SliceRandom;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(7., false)))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(hit_player)
                    .with_system(spawn_enemies)
                    .with_system(follow_player),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(drop_enemies));
    }
}

#[derive(Component)]
struct Enemy;

#[derive(Component, Deref, DerefMut)]
struct AttackTimer(Timer);

#[derive(Deref, DerefMut)]
struct EnemySpawnTimer(Timer);

pub const ENEMY_SPAWN_TIME_INCREASE_RATE: f32 = 0.95;
pub const ENEMY_SPAWN_TIME_MINIMUM: f32 = 0.2;

#[derive(Bundle)]
pub struct EnemyBundle {
    health: Health,
    movement: Movement,
    enemy: Enemy,
    attack_timer: AttackTimer,
    #[bundle]
    sprite: SpriteBundle,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            health: Health::new(100.),
            movement: Movement { speed: 50. },
            enemy: Enemy,
            attack_timer: AttackTimer(Timer::from_seconds(2., false)),
            sprite: SpriteBundle::default(),
        }
    }
}

fn follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies_query: Query<(&mut Transform, &Movement), With<Enemy>>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let player_translation = player_transform.translation.xy();

    for (mut enemy_transform, movement) in enemies_query.iter_mut() {
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
        enemy_transform.rotation = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));

        if collide(
            enemy_transform.translation,
            Vec2::splat(16.),
            player_transform.translation,
            Vec2::splat(32.),
        )
        .is_none()
        {
            let forward = enemy_transform.up();
            enemy_transform.translation += forward * time.delta_seconds() * movement.speed;
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    enemy_spawn_points: Query<&Transform, With<EnemySpawn>>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    textures: Res<TextureAssets>,
) {
    println!("{}", enemy_spawn_timer.elapsed_secs());

    if !enemy_spawn_timer.tick(time.delta()).just_finished() {
        return;
    }

    // Make the next enemy spawn faster
    let old_duration = enemy_spawn_timer.duration().as_secs_f32();

    let new_duration = (old_duration - ENEMY_SPAWN_TIME_MINIMUM) * ENEMY_SPAWN_TIME_INCREASE_RATE + ENEMY_SPAWN_TIME_MINIMUM;

    enemy_spawn_timer.set_duration(Duration::from_secs_f32(new_duration));
    enemy_spawn_timer.reset();

    // Choose a random spawn point
    let spawn_points = enemy_spawn_points.iter().collect::<Vec<&Transform>>();

    let spawn_point = spawn_points
        .choose(&mut rand::thread_rng())
        .expect("There are no enemy spawn points on the map");

    let position = spawn_point.translation.truncate() + spawn_point.up().truncate() * WALL_WIDTH;

    commands.spawn_bundle(EnemyBundle {
        health: Health::new(100.),
        movement: Movement { speed: 50. },
        enemy: Enemy,
        attack_timer: AttackTimer(Timer::from_seconds(2., false)),
        sprite: SpriteBundle {
            texture: textures.enemy_texture.clone(),
            transform: Transform {
                translation: position.extend(0.),
                ..default()
            },
            ..default()
        },
    });
}

fn hit_player(
    mut enemy_query: Query<(&Transform, &mut AttackTimer), With<Enemy>>,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    time: Res<Time>,
) {
    let (player_transform, mut player_health) = player_query.single_mut();

    for (enemy_transform, mut attack_timer) in enemy_query.iter_mut() {
        attack_timer.tick(time.delta());

        if collide(
            player_transform.translation,
            Vec2::splat(32.),
            enemy_transform.translation,
            Vec2::splat(32.),
        )
        .is_some()
            && attack_timer.finished()
        {
            player_health.take_damage(10.);
            attack_timer.reset();
        }
    }
}

fn drop_enemies(mut commands: Commands, enemies: Query<Entity, With<Enemy>>) {
    for enemy in enemies.iter() {
        commands.entity(enemy).despawn_recursive();
    }
}
