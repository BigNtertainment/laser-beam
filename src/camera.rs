use bevy::{prelude::*, render::camera::ScalingMode};

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
                scale: 1.0,
                scaling_mode: ScalingMode::Auto {
                    min_width: 960.,
                    min_height: 480.,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Camera"));
}
