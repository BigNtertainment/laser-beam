use crate::loading::TextureAssets;
use crate::weapon::EntityHitEvent;
use crate::{
    character::{Health, Movement},
    player::Player,
};
use crate::{GameState, GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use bevy::math::Vec3Swizzles;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_enemies))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(hit_player)
                    .with_system(follow_player)
                    .with_system(take_damage),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(drop_enemies));
    }
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct AttackTimer(Timer);

#[derive(Bundle)]
pub struct EnemyBundle {
    health: Health,
    movement: Movement,
    enemy: Enemy,
    attack_timer: AttackTimer,
    collider: Collider,
    #[bundle]
    sprite: SpriteBundle,
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

fn spawn_enemies(mut commands: Commands, textures: Res<TextureAssets>) {
    let mut rng = rand::thread_rng();

    for _ in 0..200 {
        let position = Vec2::new(
            rng.gen_range(0.0..(GAME_AREA_WIDTH / 2.)),
            rng.gen_range(0.0..(GAME_AREA_HEIGHT / 2.)),
        );

        commands.spawn_bundle(EnemyBundle {
            health: Health::new(100.),
            movement: Movement { speed: 50. },
            enemy: Enemy,
            attack_timer: AttackTimer(Timer::from_seconds(2., true)),
            collider: Collider::cuboid(32., 32.),
            sprite: SpriteBundle {
                texture: textures.enemy_texture.clone(),
                transform: Transform::from_translation(position.extend(0.)),
                ..default()
            },
        });
    }
}

fn take_damage(
    mut enemies: Query<(Entity, &mut Health), With<Enemy>>,
    mut entity_hit_event_r: EventReader<EntityHitEvent>,
    mut commands: Commands,
) {
    for hit in entity_hit_event_r.iter() {
        for (enemy_entity, mut health) in enemies.iter_mut() {
            if enemy_entity.id() == hit.0.id() {
                if health.take_damage(50.) {
                    commands.entity(enemy_entity).despawn_recursive();
                }

                info!("remaining_health={:?}", health.get_health());
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
        if collide(
            player_transform.translation,
            Vec2::splat(32.),
            enemy_transform.translation,
            Vec2::splat(32.),
        )
        .is_some()
            && attack_timer.0.tick(time.delta()).just_finished()
        {
            // TODO: Make AttackDamage component?
            player_health.take_damage(10.);
        }
    }
}

fn drop_enemies(mut commands: Commands, enemies: Query<Entity, With<Enemy>>) {
    for enemy in enemies.iter() {
        commands.entity(enemy).despawn_recursive();
    }
}
