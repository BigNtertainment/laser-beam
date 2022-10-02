use crate::game_area::{grid_to_world_coords, world_to_grid_coords};
use crate::loading::TextureAssets;
use crate::GameState;
use crate::{
    character::{Health, Movement},
    player::Player,
};
use bevy::math::Vec3Swizzles;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use pathfinding::prelude::{astar, Grid};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_enemies))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(hit_player)
                    .with_system(follow_player.label("enemy_ai"))
                    .with_system(update_enemy_position.after("enemy_ai")),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(drop_enemies));
    }
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct AttackTimer(Timer);

#[derive(Component)]
struct PathCache {
    player_square: (usize, usize),
    path: Option<Vec<Vec2>>,
    index: usize,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    health: Health,
    movement: Movement,
    enemy: Enemy,
    attack_timer: AttackTimer,
    path_cache: PathCache,
    #[bundle]
    sprite: SpriteBundle,
}

fn follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies_query: Query<(&mut Transform, &mut PathCache, &Movement), With<Enemy>>,
    time: Res<Time>,
    grid: Res<Grid>,
) {
    let player_transform = player_query.single();
    let player_translation = player_transform.translation.xy();

    let player_square = world_to_grid_coords(player_translation);

    for (mut enemy_transform, mut path_cache, movement) in enemies_query.iter_mut() {
        let enemy_square = world_to_grid_coords(enemy_transform.translation.truncate());

        if enemy_square == player_square {
            // If the enemy is close to the player, approach them in a straight line
            if collide(
                enemy_transform.translation,
                Vec2::splat(16.),
                player_transform.translation,
                Vec2::splat(32.),
            )
            .is_none()
            {
                let direction =
                    (enemy_transform.translation.truncate() - player_translation).normalize();

                enemy_transform.translation +=
                    (direction * movement.speed * time.delta_seconds()).extend(0.0);
            }
        } else {
            // If the enemy is far away, use A* to traverse the map
            if path_cache.player_square == player_square && path_cache.path.is_some() {
                continue;
            }

            // Calculate the path and save it in cache
            let path = astar(
                &enemy_square,
                |p| grid.neighbours(*p).into_iter().map(|p| (p, 1)),
                |p| grid.distance(*p, player_square),
                |p| *p == player_square,
            );

            if let Some(path) = path {
                let path = path.0;

                // Insert the path transformed from grid coords into world coords
                path_cache.path = Some(
                    path.into_iter()
                        .map(|point| grid_to_world_coords(point))
                        .collect(),
                );

                path_cache.index = 1;
                path_cache.player_square = player_square;
            }
        }
    }
}

fn update_enemy_position(
    mut enemies_query: Query<(&mut Transform, &mut PathCache, &Movement), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut enemy_transform, mut path_cache, movement) in enemies_query.iter_mut() {
        if let Some(path) = path_cache.path.as_ref() {
            let direction = path[path_cache.index] - enemy_transform.translation.truncate();

            let direction_normalized = direction.normalize_or_zero();

            let movement_vector = direction_normalized * movement.speed * time.delta_seconds();

            // If the next goal has been reached
            if direction.length() <= movement_vector.length() {
                enemy_transform.translation.x = path[path_cache.index].x;
                enemy_transform.translation.y = path[path_cache.index].y;

                path_cache.index += 1;

                // The enemy finished the path
                if path_cache.index >= path_cache.path.as_ref().unwrap().len() {
                    path_cache.index = 0;
                    path_cache.path = None;
                    path_cache.player_square = (0, 0);
                }

                continue;
            }

            enemy_transform.translation += movement_vector.extend(0.0);
        }
    }
}

fn spawn_enemies(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn_bundle(EnemyBundle {
        health: Health::new(100.),
        movement: Movement { speed: 50.0 },
        enemy: Enemy,
        attack_timer: AttackTimer(Timer::from_seconds(2., true)),
        path_cache: PathCache {
            player_square: (12, 12),
            path: None,
            index: 0,
        },
        sprite: SpriteBundle {
            texture: textures.enemy_texture.clone(),
            transform: Transform::from_translation(Vec3::new(-250., 200., 1.)),
            ..default()
        },
    }).insert(Name::new("Enemy"));
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

fn drop_enemies(mut commands: Commands, enemies: Query<Entity, With<Enemy>>) {
    for enemy in enemies.iter() {
        commands.entity(enemy).despawn_recursive();
    }
}
