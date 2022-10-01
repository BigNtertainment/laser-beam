use crate::health_bar::HealthBar;
use crate::loading::TextureAssets;
use crate::GameState;
use crate::{
    character::{Health, Movement},
    player::Player,
};
use bevy::math::Vec3Swizzles;
use bevy::{prelude::*, sprite::collide_aabb::collide};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_enemies))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(hit_player)
                    .with_system(follow_player),
            );
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
    for _ in 0..2 {
        commands.spawn_bundle(EnemyBundle {
            health: Health::new(100.),
            movement: Movement { speed: 50. },
            enemy: Enemy,
            attack_timer: AttackTimer(Timer::from_seconds(2., true)),
            sprite: SpriteBundle {
                texture: textures.enemy_texture.clone(),
                transform: Transform::from_translation(Vec3::new(-64., 0., 1.)),
                ..default()
            },
        });
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
            player_health.take_damage(10.);
        }
    }
}
