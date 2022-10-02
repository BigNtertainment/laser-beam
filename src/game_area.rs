use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use pathfinding::prelude::Grid;

use crate::{
    loading::TextureAssets, GameState, GAME_AREA_HEIGHT, GAME_AREA_WIDTH, WALL_HEIGHT, WALL_WIDTH,
};

pub struct GameAreaPlugin;

impl Plugin for GameAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(wall_setup))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(drop_game_area));
    }
}

#[derive(Component)]
pub struct GameArea;

fn wall_setup(mut commands: Commands, textures: Res<TextureAssets>, mut lines: ResMut<DebugLines>) {
    let wall_texture = &textures.wall_texture;

    let mut walls = Vec::new();

    // Spawn the walls at the top and bottom
    for i in -(GAME_AREA_WIDTH / WALL_WIDTH) as i32 / 2..(GAME_AREA_WIDTH / WALL_WIDTH) as i32 / 2 {
        // Top wall
        walls.push(
            commands
                .spawn_bundle(SpriteBundle {
                    texture: wall_texture.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            (i as f32 + 0.5) * WALL_WIDTH,
                            (GAME_AREA_HEIGHT - WALL_HEIGHT) / 2.,
                            0.,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Name::new("Wall"))
                .id(),
        );

        // Bottom wall
        walls.push(
            commands
                .spawn_bundle(SpriteBundle {
                    texture: wall_texture.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            (i as f32 + 0.5) * WALL_WIDTH,
                            -(GAME_AREA_HEIGHT - WALL_HEIGHT) / 2.,
                            0.,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Name::new("Wall"))
                .id(),
        );
    }

    // Spawn the left and right walls
    for i in -(GAME_AREA_HEIGHT / WALL_HEIGHT - 1.) as i32 / 2
        ..(GAME_AREA_HEIGHT / WALL_HEIGHT - 1.) as i32 / 2
    {
        walls.push(
            commands
                .spawn_bundle(SpriteBundle {
                    texture: wall_texture.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            -(GAME_AREA_WIDTH - WALL_WIDTH) / 2.,
                            (i as f32 + 0.5) * WALL_HEIGHT,
                            0.,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Name::new("Wall"))
                .id(),
        );

        walls.push(
            commands
                .spawn_bundle(SpriteBundle {
                    texture: wall_texture.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            (GAME_AREA_WIDTH - WALL_WIDTH) / 2.,
                            (i as f32 + 0.5) * WALL_HEIGHT,
                            0.,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Name::new("Wall"))
                .id(),
        );
    }

    let grid_size = (
        (GAME_AREA_WIDTH / (WALL_WIDTH / 2.)) as usize,
        (GAME_AREA_HEIGHT / (WALL_HEIGHT / 2.)) as usize,
    );

    let mut grid = Grid::new(grid_size.0, grid_size.1);

    grid.enable_diagonal_mode();
    grid.fill();

    let inner_walls = vec![(15, 5), (16, 5), (17, 5), (15, 6), (16, 6), (17, 6)];

    for wall in inner_walls {
        grid.remove_vertex((wall.0 * 2, wall.1 * 2));
        grid.remove_vertex((wall.0 * 2, wall.1 * 2 - 1));
        grid.remove_vertex((wall.0 * 2, wall.1 * 2 + 1));
        grid.remove_vertex((wall.0 * 2 - 1, wall.1 * 2));
        grid.remove_vertex((wall.0 * 2 + 1, wall.1 * 2));
        grid.remove_vertex((wall.0 * 2 + 1, wall.1 * 2 + 1));
        grid.remove_vertex((wall.0 * 2 - 1, wall.1 * 2 - 1));
        grid.remove_vertex((wall.0 * 2 + 1, wall.1 * 2 - 1));
        grid.remove_vertex((wall.0 * 2 - 1, wall.1 * 2 + 1));

        walls.push(
            commands
                .spawn_bundle(SpriteBundle {
                    texture: wall_texture.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            (wall.0) as f32 * WALL_WIDTH - GAME_AREA_WIDTH / 2.,
                            (wall.1) as f32 * -WALL_HEIGHT + GAME_AREA_HEIGHT / 2.,
                            0.,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Name::new("Wall"))
                .id(),
        );
    }

    println!("{:?}", grid);

    for i in 0..grid.width {
        for j in 0..grid.height {
            if !grid.has_vertex((i, j)) {
                continue;
            }

            let world_coords = grid_to_world_coords((i, j));

            lines.line(
                (world_coords + Vec2::new(0.25, 0.25) * WALL_WIDTH).extend(0.),
                (world_coords + Vec2::new(-0.25, 0.25) * WALL_WIDTH).extend(0.),
                30.0,
            );
            lines.line(
                (world_coords + Vec2::new(-0.25, 0.25) * WALL_WIDTH).extend(0.),
                (world_coords + Vec2::new(-0.25, -0.25) * WALL_WIDTH).extend(0.),
                30.0,
            );
            lines.line(
                (world_coords + Vec2::new(-0.25, -0.25) * WALL_WIDTH).extend(0.),
                (world_coords + Vec2::new(0.25, -0.25) * WALL_WIDTH).extend(0.),
                30.0,
            );
            lines.line(
                (world_coords + Vec2::new(0.25, -0.25) * WALL_WIDTH).extend(0.),
                (world_coords + Vec2::new(0.25, 0.25) * WALL_WIDTH).extend(0.),
                30.0,
            );
        }
    }

    commands.insert_resource(grid);

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

pub fn world_to_grid_coords(world: Vec2) -> (usize, usize) {
    (
        ((GAME_AREA_WIDTH / 2. + world.x) / (WALL_WIDTH / 2.)) as usize,
        ((GAME_AREA_HEIGHT / 2. - world.y) / (WALL_WIDTH / 2.)) as usize,
    )
}

pub fn grid_to_world_coords(grid: (usize, usize)) -> Vec2 {
    Vec2::new(
        grid.0 as f32 * (WALL_WIDTH / 2.) - GAME_AREA_WIDTH / 2.,
        -(grid.1 as f32 * (WALL_HEIGHT / 2.) - GAME_AREA_HEIGHT / 2.),
    )
}

#[cfg(test)]
mod tests {
    use crate::game_area::{grid_to_world_coords, world_to_grid_coords};

    #[test]
    fn coords_conversion() {
        let coords = (3, 6);

        assert_eq!(world_to_grid_coords(grid_to_world_coords(coords)), coords);
    }
}
