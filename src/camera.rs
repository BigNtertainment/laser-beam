use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

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
            ..Default::default()
        })
        .insert(Name::new("Camera"))
        .insert(MainCamera);
}
