use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<Movement>()
            .register_type::<Rotation>();
    }
}

#[derive(Default, Reflect, Inspectable, Component)]
#[reflect(Component)]
pub struct Health {
    health: f32,
    max_health: f32,
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Self {
            health: max_health,
            max_health,
        }
    }

    /// # Returns
    /// True if the health reached zero.
    #[allow(unused)]
    #[deny(unused_results)]
    pub fn take_damage(&mut self, amount: f32) -> bool {
        self.health -= amount;

        self.health <= 0.0
    }

    #[allow(unused)]
    pub fn heal(&mut self, amount: f32) {
        self.health += amount;

        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    #[allow(unused)]
    pub fn get_health(&self) -> f32 {
        self.health
    }

    #[allow(unused)]
    pub fn get_max_health(&self) -> f32 {
        self.max_health
    }
}

#[derive(Default, Reflect, Inspectable, Component)]
#[reflect(Component)]
pub struct Movement {
    pub speed: f32,
}

#[derive(Default, Reflect, Inspectable, Component)]
#[reflect(Component)]
pub struct Rotation {
    pub rotation_speed: f32,
}
