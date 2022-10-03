use std::f32::consts::PI;

use crate::{
    loading::{AudioAssets, TextureAssets},
    player::Player,
    GameState,
};
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioTween};
use bevy_rapier2d::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityHitEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_laser))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(drop_laser))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(shoot).with_system(play_laser_sound));
    }
}

pub const LASER_END_WIDTH: f32 = 50.;
pub const LASER_END_HEIGHT: f32 = 50.;

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

#[derive(Component)]
struct LaserEnd;

#[derive(Bundle)]
struct LaserEndBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    laser: Laser,
    laser_end: LaserEnd,
}

#[derive(Deref, DerefMut)]
struct LaserSound(Handle<AudioInstance>);

fn setup_laser(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    audio: Res<Audio>,
    sounds: Res<AudioAssets>,
) {
    commands
        .spawn_bundle(LaserBundle {
            sprite_bundle: SpriteBundle {
                texture: textures.laser_texture.clone(),
                visibility: Visibility { is_visible: false },
                ..default()
            },
            laser: Laser,
        })
        .insert(Name::new("Laser"));

    commands
        .spawn_bundle(LaserEndBundle {
            sprite_bundle: SpriteBundle {
                texture: textures.laser_end_texture.clone(),
                visibility: Visibility { is_visible: false },
                ..default()
            },
            laser: Laser,
            laser_end: LaserEnd,
        })
        .insert(Name::new("LaserEnd"));

    commands.insert_resource(LaserSound(audio.play(sounds.laser.clone()).with_volume(0.).with_playback_rate(0.).looped().handle()));
}

fn drop_laser(mut commands: Commands, laser: Query<Entity, With<Laser>>) {
    for laser in laser.iter() {
        commands.entity(laser).despawn_recursive();
    }
}

fn shoot<'a>(
    mut weapon: Query<&mut Weapon>,
    player_query: Query<&Transform, With<Player>>,
    mut laser_query: Query<
        (&mut Transform, &mut Visibility, &Handle<Image>),
        (With<Laser>, Without<LaserEnd>, Without<Player>),
    >,
    mut laser_end_query: Query<
        (&mut Transform, &mut Visibility, &Handle<Image>),
        (With<Laser>, With<LaserEnd>, Without<Player>),
    >,
    transforms: Query<(Entity, &Transform), (Without<Weapon>, Without<Player>, Without<Laser>)>,
    mut entity_hit_event_w: EventWriter<EntityHitEvent>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    images: Res<Assets<Image>>,
) {
    let ray_cast_filter = QueryFilter::default();
    let player_transform = player_query.single();
    let (mut laser_transform, mut laser_visibility, laser_texture) = laser_query.single_mut();
    let (mut laser_end_transform, mut laser_end_visibility, laser_end_texture) =
        laser_end_query.single_mut();
    let shoot_direction = player_transform.up();

    for mut weapon in weapon.iter_mut() {
        laser_visibility.is_visible = weapon.status == WeaponStatus::Beaming;
        laser_end_visibility.is_visible = weapon.status == WeaponStatus::Beaming;

        let laser_texture_height = images
            .get(laser_texture)
            .unwrap()
            .texture_descriptor
            .size
            .height as f32;

        let laser_start = player_transform.translation + shoot_direction * 20.;

        match weapon.status {
            WeaponStatus::Beaming => {
                if let Some((hit, toi)) = rapier_context.cast_ray(
                    player_transform.translation.truncate(),
                    shoot_direction.truncate(),
                    100000.,
                    true,
                    ray_cast_filter,
                ) {
                    entity_hit_event_w.send(EntityHitEvent(hit));
                    info!("entity hit event sent {:?}", hit);

                    let laser_end = player_transform.translation + shoot_direction * (toi + 2.);

                    let laser_position = (laser_start + laser_end) / 2.;

                    laser_transform.translation = laser_position;
                    laser_transform.translation.z = 3.;

                    laser_transform.rotation = Quat::from_rotation_z(
                        Vec2::Y.angle_between(shoot_direction.truncate()) - PI / 2.,
                    );

                    laser_transform.scale.x =
                        (laser_end - laser_start).length() / laser_texture_height;

                    let mut hit_transform = &Transform::default();

                    for (entity, transform) in transforms.iter() {
                        if hit == entity {
                            hit_transform = transform;
                        }
                    }

                    laser_end_transform.rotation = Quat::from_rotation_z(
                        hit_transform.rotation.to_euler(EulerRot::XYZ).2 + PI / 2.,
                    );

                    let laser_end_texture_size = images
                        .get(laser_end_texture)
                        .unwrap()
                        .texture_descriptor
                        .size;

                    laser_end_transform.scale = Vec3::new(
                        LASER_END_WIDTH / laser_end_texture_size.width as f32,
                        LASER_END_HEIGHT / laser_end_texture_size.height as f32,
                        1.,
                    );

                    laser_end_transform.translation = laser_end
                        + laser_end_transform.right()
                            * laser_end_transform.scale.y
                            * laser_end_texture_size.height as f32
                            / 4.;
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

fn play_laser_sound(weapon: Query<&Weapon>, laser_sound: Res<LaserSound>, mut audio_instances: ResMut<Assets<AudioInstance>>, time: Res<Time>) {
    let weapon = weapon.single();

    if let Some(laser_audio) = audio_instances.get_mut(&laser_sound.0) {
        let audio_tween = AudioTween::linear(time.delta());
    
        match weapon.status {
            WeaponStatus::Idle => {
                let progress = 1. - weapon.cooldown.percent_left();
    
                laser_audio.set_volume(0.05 + 0.1 * progress as f64, audio_tween.clone());
                laser_audio.set_playback_rate(0.8 * progress as f64, audio_tween);
            },
            WeaponStatus::Beaming => {
                // let progress = weapon.cooldown.percent_left() * 0.25;

                laser_audio.set_volume(0.15, audio_tween.clone());
                laser_audio.set_playback_rate(0.8, audio_tween);
            }
        }
    }
}