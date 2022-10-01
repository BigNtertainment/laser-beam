use bevy::prelude::*;

pub struct LaserTimer(Timer);

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LaserTimer(Timer::from_seconds(10., true)))
            .add_system(laser);
    }
}

fn laser(time: Res<Time>, mut laser_timer: ResMut<LaserTimer>) {
    if laser_timer.0.tick(time.delta()).just_finished() {
        info!("Shoot");
    }
}
