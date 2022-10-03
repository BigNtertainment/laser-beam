use std::f32::consts::PI;

use crate::{loading::TextureAssets, player::Player, GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityHitEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_laser))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(drop_laser))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(shoot));
    }
}

pub struct EntityHitEvent(pub Entity);

#[derive(Bundle)]
pub struct WeaponBundle {
    pub weapon: Weapon,
}

#[derive(PartialEq)]
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
            cooldown: Timer::from_seconds(10., false),
            beaming_time: Timer::from_seconds(2., false),
            status: WeaponStatus::Idle,
        }
    }
}

#[derive(Component)]
struct Laser;

#[derive(Bundle)]
struct LaserBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    laser: Laser,
}

fn setup_laser(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn_bundle(LaserBundle {
        sprite_bundle: SpriteBundle {
            texture: textures.laser_texture.clone(),
            visibility: Visibility { is_visible: false },
            ..default()
        },
        laser: Laser,
    });
}

fn drop_laser(mut commands: Commands, laser: Query<Entity, With<Laser>>) {
    commands.entity(laser.single()).despawn_recursive();
}

fn shoot(
    mut weapon: Query<&mut Weapon>,
    player_query: Query<&Transform, With<Player>>,
    mut laser_query: Query<
        (&mut Transform, &mut Visibility, &Handle<Image>),
        (With<Laser>, Without<Player>),
    >,
    mut entity_hit_event_w: EventWriter<EntityHitEvent>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    images: Res<Assets<Image>>,
) {
    let ray_cast_filter = QueryFilter::default();
    let player_transform = player_query.single();
    let (mut laser_transform, mut laser_visibility, laser_texture) = laser_query.single_mut();
    let shoot_direction = player_transform.up();

    for mut weapon in weapon.iter_mut() {
        laser_visibility.is_visible = weapon.status == WeaponStatus::Beaming;

        let laser_texture_height = images
            .get(laser_texture)
            .unwrap()
            .texture_descriptor
            .size
            .height as f32;

        let laser_start = player_transform.translation + shoot_direction * 20.;

        match weapon.status {
            WeaponStatus::Beaming => {
                if let Some((hit_entity, toi)) = rapier_context.cast_ray(
                    player_transform.translation.truncate(),
                    shoot_direction.truncate(),
                    100000.,
                    true,
                    ray_cast_filter,
                ) {
                    entity_hit_event_w.send(EntityHitEvent(hit_entity));
                    info!("entity hit event sent {:?}", hit_entity);

                    let laser_end = player_transform.translation + shoot_direction * toi;

                    let laser_position = (laser_start + laser_end) / 2.;

                    laser_transform.translation = laser_position;
                    laser_transform.translation.z = 3.;

                    laser_transform.rotation = Quat::from_rotation_z(
                        Vec2::Y.angle_between(shoot_direction.truncate()) - PI / 2.,
                    );
    
                    laser_transform.scale.x = (laser_end - laser_start).length() / laser_texture_height;
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
