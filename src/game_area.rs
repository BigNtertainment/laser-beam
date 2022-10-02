use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution};

use crate::{
    loading::TextureAssets, GameState, GAME_AREA_HEIGHT, GAME_AREA_WIDTH, WALL_HEIGHT, WALL_WIDTH,
};

pub const ENEMY_SPAWN_NUMBER: u32 = 6;

pub struct GameAreaPlugin;

impl Plugin for GameAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(world_setup))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(drop_game_area));
    }
}

#[derive(Component)]
pub struct GameArea;

#[derive(PartialEq, Clone, Copy)]
pub enum Wall {
    Top,
    Right,
    Bottom,
    Left,
}

impl Distribution<Wall> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Wall {
        match rng.gen_range(0..=3) {
            0 => Wall::Top,
            1 => Wall::Right,
            2 => Wall::Bottom,
            3 => Wall::Left,
            _ => panic!("How did we get here?"),
        }
    }
}

#[derive(PartialEq)]
pub struct EnemySpawnPoint {
    pub wall: Wall,
    pub position: i32,
}

impl Distribution<EnemySpawnPoint> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> EnemySpawnPoint {
        let wall = rng.gen();

        let position = match wall {
            Wall::Top | Wall::Bottom => rng.gen_range(
                (-GAME_AREA_WIDTH / WALL_WIDTH / 2.) as i32 + 2
                    ..(GAME_AREA_WIDTH / WALL_WIDTH / 2.) as i32 - 2,
            ),
            Wall::Left | Wall::Right => rng.gen_range(
                (-GAME_AREA_HEIGHT / WALL_HEIGHT / 2.) as i32 + 2
                    ..(GAME_AREA_HEIGHT / WALL_HEIGHT / 2.) as i32 - 2,
            ),
        };

        EnemySpawnPoint { wall, position }
    }
}

#[derive(Component)]
pub struct EnemySpawn;

#[derive(PartialEq)]
enum WallTile {
    Empty,
    Wall,
    EnemySpawn,
}

fn get_wall_tile(position: i32, face: &Wall, enemy_spawns: &Vec<EnemySpawnPoint>) -> WallTile {
    let mut enemy_spawn_point = EnemySpawnPoint {
        position,
        wall: *face,
    };

    if enemy_spawns.contains(&enemy_spawn_point) {
        return WallTile::EnemySpawn;
    }

    enemy_spawn_point.position += 1;

    if enemy_spawns.contains(&enemy_spawn_point) {
        return WallTile::Empty;
    }

    enemy_spawn_point.position -= 2;

    if enemy_spawns.contains(&enemy_spawn_point) {
        return WallTile::Empty;
    }

    WallTile::Wall
}

fn world_setup(mut commands: Commands, textures: Res<TextureAssets>) {
    // Generate enemy spawns
    let mut enemy_spawns = Vec::new();

    for _ in 0..ENEMY_SPAWN_NUMBER {
        let enemy_spawn = loop {
            let result: EnemySpawnPoint = rand::random();

            if get_wall_tile(result.position, &result.wall, &enemy_spawns) == WallTile::Wall
                && get_wall_tile(result.position - 1, &result.wall, &enemy_spawns) == WallTile::Wall
                && get_wall_tile(result.position + 1, &result.wall, &enemy_spawns) == WallTile::Wall
            {
                break result;
            }
        };

        enemy_spawns.push(enemy_spawn);
    }

    let wall_texture = &textures.wall_texture;
    let enemy_spawn_texture = &textures.enemy_spawn_texture;

    let mut walls = Vec::new();

    let mut spawn_wall = |position: i32, face: Wall| {

        let translation = match face {
            Wall::Top => Vec2::new(
                (position as f32 + 0.5) * WALL_WIDTH,
                (GAME_AREA_HEIGHT - WALL_HEIGHT) / 2.,
            ),
            Wall::Bottom => Vec2::new(
                (position as f32 + 0.5) * WALL_WIDTH,
                -(GAME_AREA_HEIGHT - WALL_HEIGHT) / 2.,
            ),
            Wall::Left => Vec2::new(
                -(GAME_AREA_WIDTH - WALL_WIDTH) / 2.,
                (position as f32 + 0.5) * WALL_HEIGHT,
            ),
            Wall::Right => Vec2::new(
                (GAME_AREA_WIDTH - WALL_WIDTH) / 2.,
                (position as f32 + 0.5) * WALL_HEIGHT,
            ),
        };

        let rotation = Vec2::Y.angle_between(match face {
            Wall::Top => Vec2::NEG_Y,
            Wall::Bottom => Vec2::Y,
            Wall::Left => Vec2::X,
            Wall::Right => Vec2::NEG_X,
        });

        let is_enemy_spawn = match get_wall_tile(position, &face, &enemy_spawns) {
            WallTile::Wall => false,
            WallTile::EnemySpawn => true,
            WallTile::Empty => return commands.spawn().insert(Name::new("Nothing :(")).id(),
        };

        let mut wall = commands.spawn_bundle(SpriteBundle {
            texture: if is_enemy_spawn {
                enemy_spawn_texture.clone()
            } else {
                wall_texture.clone()
            },
            transform: Transform {
                translation: translation.extend(0.),
                rotation: Quat::from_rotation_z(rotation),
                ..Default::default()
            },
            ..Default::default()
        });

        wall.insert(Name::new(if is_enemy_spawn {
            "EnemySpawn"
        } else {
            "Wall"
        }));

        if is_enemy_spawn {
            wall.insert(EnemySpawn);
        }

        wall.id()
    };

    // Spawn the walls at the top and bottom
    for i in -(GAME_AREA_WIDTH / WALL_WIDTH) as i32 / 2..(GAME_AREA_WIDTH / WALL_WIDTH) as i32 / 2 {
        // Top wall
        walls.push(spawn_wall(i, Wall::Top));

        // Bottom wall
        walls.push(spawn_wall(i, Wall::Bottom));
    }

    // Spawn the left and right walls
    for i in -(GAME_AREA_HEIGHT / WALL_HEIGHT - 1.) as i32 / 2
        ..(GAME_AREA_HEIGHT / WALL_HEIGHT - 1.) as i32 / 2
    {
        // Left wall
        walls.push(spawn_wall(i, Wall::Left));

        // Right wall
        walls.push(spawn_wall(i, Wall::Right));
    }

    commands
        .spawn()
        // Later move it to an entity containing the entire game area (including floors and windows)
        .insert(GameArea)
        .insert(Name::new("Walls"))
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&*walls);
}

fn drop_game_area(mut commands: Commands, game_area: Query<Entity, With<GameArea>>) {
    for game_area in game_area.iter() {
        commands.entity(game_area).despawn_recursive();
    }
}
