use crate::{player::Player, GameState};
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityHitEvent>()
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(shoot));
    }
}

pub struct EntityHitEvent(pub Entity);

#[derive(Bundle)]
pub struct WeaponBundle {
    pub weapon: Weapon,
}

pub enum WeaponStatus {
    Idle,
    Beaming,
}

#[derive(Component)]
pub struct Weapon {
    pub cooldown: Timer,
    pub beaming_time: Timer,
    pub status: WeaponStatus,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            cooldown: Timer::from_seconds(5., false),
            beaming_time: Timer::from_seconds(2., false),
            status: WeaponStatus::Idle,
        }
    }
}

fn shoot(
    mut weapon: Query<&mut Weapon>,
    player_query: Query<&Transform, With<Player>>,
    mut entity_hit_event_w: EventWriter<EntityHitEvent>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut lines: ResMut<DebugLines>,
) {
    let ray_cast_filter = QueryFilter::default();
    let player_transform = player_query.single();
    let shoot_direction = player_transform.up() * 100000.;

    for mut weapon in weapon.iter_mut() {
        match weapon.status {
            WeaponStatus::Beaming => {
                lines.line(
                    player_transform.translation,
                    player_transform.translation + shoot_direction,
                    0.,
                );

                if let Some((hit_entity, _)) = rapier_context.cast_ray(
                    player_transform.translation.truncate(),
                    shoot_direction.truncate(),
                    1.0,
                    true,
                    ray_cast_filter,
                ) {
                    entity_hit_event_w.send(EntityHitEvent(hit_entity));
                    info!("entity hit event sent {:?}", hit_entity);
                }

                if weapon.beaming_time.tick(time.delta()).just_finished() {
                    weapon.status = WeaponStatus::Idle;
                    weapon.cooldown.reset();
                }
            }
            WeaponStatus::Idle => {
                if weapon.cooldown.tick(time.delta()).just_finished() {
                    weapon.beaming_time.reset();
                    weapon.status = WeaponStatus::Beaming;
                }
            }
        }
    }
}
