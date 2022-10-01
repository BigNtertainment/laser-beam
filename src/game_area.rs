use bevy::prelude::*;

use crate::{
    loading::TextureAssets, GameState, GAME_AREA_HEIGHT, GAME_AREA_WIDTH, WALL_HEIGHT, WALL_WIDTH,
};

pub struct GameAreaPlugin;

impl Plugin for GameAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(wall_setup)).add_system_set(SystemSet::on_exit(GameState::Playing).with_system(drop_game_area));
    }
}

#[derive(Component)]
pub struct GameArea;

fn wall_setup(mut commands: Commands, textures: Res<TextureAssets>) {
    let wall_texture = &textures.wall_texture;

    let mut walls = Vec::new();

    // Spawn the walls at the top and bottom
    for i in
        -(GAME_AREA_WIDTH / WALL_WIDTH) as i32 / 2..(GAME_AREA_WIDTH / WALL_WIDTH) as i32 / 2
    {
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
