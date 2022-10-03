use std::time::Duration;

use crate::game_area::EnemySpawn;
use crate::loading::TextureAssets;
use crate::score::Score;
use crate::weapon::EntityHitEvent;
use crate::{
    character::{Health, Movement},
    player::Player,
};
use crate::{GameState, WALL_WIDTH};
use bevy::math::Vec3Swizzles;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_rapier2d::prelude::*;
use rand::seq::SliceRandom;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(enemy_spawn_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(hit_player)
                    .with_system(spawn_enemies)
                    .with_system(follow_player)
                    .with_system(take_damage),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing)
                    .with_system(enemy_spawn_cleanup)
                    .with_system(drop_enemies),
            );
    }
}

#[derive(Component)]
struct Enemy;

#[derive(Component, Deref, DerefMut)]
struct AttackTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct HitTimer(Timer);

#[derive(Deref, DerefMut)]
struct EnemySpawnTimer(Timer);

pub const ENEMY_SPAWN_TIME_DEFAULT: f32 = 7.;
pub const ENEMY_SPAWN_TIME_INCREASE_RATE: f32 = 0.95;
pub const ENEMY_SPAWN_TIME_MINIMUM: f32 = 0.5;

#[derive(Bundle)]
pub struct EnemyBundle {
    health: Health,
    movement: Movement,
    enemy: Enemy,
    attack_timer: AttackTimer,
    hit_timer: HitTimer,
    collider: Collider,
    name: Name,
    #[bundle]
    sprite: SpriteBundle,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            health: Health::new(100.),
            movement: Movement { speed: 50. },
            enemy: Enemy,
            collider: Collider::cuboid(64., 64.),
            attack_timer: AttackTimer(Timer::from_seconds(2., false)),
            hit_timer: HitTimer(Timer::from_seconds(0.5, false)),
            name: Name::new("Enemy"),
            sprite: SpriteBundle::default(),
        }
    }
}

fn follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies_query: Query<(&mut Transform, &Movement, &HitTimer), With<Enemy>>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let player_translation = player_transform.translation.xy();

    for (mut enemy_transform, movement, hit_timer) in enemies_query.iter_mut() {
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
            enemy_transform.translation += forward
                * time.delta_seconds()
                * movement.speed
                // Move slower when getting shot
                * if !hit_timer.finished() { 0.5 } else { 1. };
        }
    }
}

fn enemy_spawn_setup(mut commands: Commands) {
    commands.insert_resource(EnemySpawnTimer(Timer::from_seconds(
        ENEMY_SPAWN_TIME_DEFAULT,
        false,
    )));
}

fn spawn_enemies(
    mut commands: Commands,
    enemy_spawn_points: Query<&Transform, With<EnemySpawn>>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    textures: Res<TextureAssets>,
) {
    if !enemy_spawn_timer.tick(time.delta()).just_finished() {
        return;
    }

    // Make the next enemy spawn faster
    let old_duration = enemy_spawn_timer.duration().as_secs_f32();

    let new_duration = (old_duration - ENEMY_SPAWN_TIME_MINIMUM) * ENEMY_SPAWN_TIME_INCREASE_RATE
        + ENEMY_SPAWN_TIME_MINIMUM;

    enemy_spawn_timer.set_duration(Duration::from_secs_f32(new_duration));
    enemy_spawn_timer.reset();

    // Choose a random spawn point
    let spawn_points = enemy_spawn_points.iter().collect::<Vec<&Transform>>();

    let spawn_point = spawn_points
        .choose(&mut rand::thread_rng())
        .expect("There are no enemy spawn points on the map");

    let position = spawn_point.translation.truncate() + spawn_point.up().truncate() * WALL_WIDTH;

    commands.spawn_bundle(EnemyBundle {
        sprite: SpriteBundle {
            texture: textures.enemy_texture.clone(),
            transform: Transform {
                translation: position.extend(2.),
                scale: Vec3::new(0.25, 0.25, 1.),
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

fn enemy_spawn_cleanup(mut commands: Commands) {
    commands.remove_resource::<EnemySpawnTimer>();
}

fn take_damage(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut HitTimer, &mut Health), With<Enemy>>,
    mut entity_hit_event_reader: EventReader<EntityHitEvent>,
    time: Res<Time>,
    mut score: ResMut<Score>,
) {
    let entity_hits = entity_hit_event_reader.iter().collect::<Vec<_>>();

    for (enemy_entity, mut hit_timer, mut health) in enemies.iter_mut() {
        hit_timer.tick(time.delta());

        for hit in &entity_hits {
            if enemy_entity.id() == hit.0.id() {
                if hit_timer.finished() {
                    // TODO: Maybe change it from a hard-coded value to a component
                    if health.take_damage(50.) {
                        commands.entity(enemy_entity).despawn_recursive();
                        score.0 += 100;
                    }

                    info!("remaining_health={:?}", health.get_health());

                    hit_timer.reset();
                }
            }
        }
    }
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
            // TODO: Make AttackDamage component?
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
