use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{GAME_AREA_WIDTH, WALL_WIDTH, WALL_HEIGHT, GAME_AREA_HEIGHT};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_setup);
    }
}

fn camera_setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            camera: Camera {
                priority: 1,
                ..Default::default()
            },
			projection: OrthographicProjection {
				scaling_mode: ScalingMode::Auto { min_width: (GAME_AREA_WIDTH + 2. * WALL_WIDTH) / 2., min_height: (GAME_AREA_HEIGHT + 2. * WALL_HEIGHT) / 2. },
				..Default::default()
			},
			..Default::default()
		})
        .insert(Name::new("Camera"));
}
