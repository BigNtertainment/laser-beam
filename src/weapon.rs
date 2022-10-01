use crate::GameState;
use bevy::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(shoot));
    }
}

#[derive(Bundle)]
pub struct WeaponBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
}

#[derive(Component)]
pub struct Weapon {
    pub cooldown: Timer,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            cooldown: Timer::from_seconds(10., true),
        }
    }
}

fn shoot(mut weapon: Query<&mut Weapon>, time: Res<Time>) {
    for mut weapon in weapon.iter_mut() {
        if weapon.cooldown.tick(time.delta()).just_finished() {
            info!("Shoot");
        }
    }
}
