use bevy::prelude::*;

use crate::{character::Health, player::Player, GameState};

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(overlay))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(health_bar_update))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(clean_health_bar));
    }
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthBarContainer;

fn overlay(mut commands: Commands) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.), Val::Px(25.)),
                        ..default()
                    },
                    color: Color::RED.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                ..default()
                            },
                            color: Color::GREEN.into(),
                            ..default()
                        })
                        .insert(HealthBar);
                });
        })
        .insert(HealthBarContainer);
}

fn health_bar_update(
    player_query: Query<&Health, With<Player>>,
    mut health_bar_query: Query<&mut Style, With<HealthBar>>,
) {
    let player_health = player_query.single();
    let mut health_bar_style = health_bar_query.single_mut();

    health_bar_style.size.width =
        Val::Percent((player_health.get_health() / player_health.get_max_health()) * 100.);
}

fn clean_health_bar(
    mut commands: Commands,
    health_bar_query: Query<Entity, With<HealthBarContainer>>,
) {
    commands
        .entity(health_bar_query.single())
        .despawn_recursive();
}
