use bevy::prelude::*;

use crate::{character::Health, loading::FontAssets, player::Player, score::Score, GameState};

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(overlay))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(health_bar_update)
                    .with_system(score_update),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(clean_ui));
    }
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct ScoreUi;

#[derive(Component)]
pub struct Ui;

fn overlay(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // Health bar
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(35.), Val::Px(25.)),
                        padding: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    color: Color::BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                ..default()
                            },
                            color: Color::RED.into(),
                            ..default()
                        })
                        .insert(HealthBar);
                });

            parent
                .spawn_bundle(TextBundle::from_section(
                    "0",
                    TextStyle {
                        font: fonts.fira_sans.clone(),
                        font_size: 27.,
                        color: Color::WHITE,
                    },
                ))
                .insert(ScoreUi);
        })
        .insert(Ui);
}

fn health_bar_update(
    player_query: Query<&Health, With<Player>>,
    mut health_bar_query: Query<&mut Style, With<HealthBar>>,
    time: Res<Time>,
) {
    let player_health = player_query.single();
    let mut health_bar_style = health_bar_query.single_mut();

    let rate = 6. * time.delta_seconds();

    let target = (player_health.get_health() / player_health.get_max_health()) * 100.;
    let mut current = match health_bar_style.size.width {
        Val::Percent(val) => val,
        _ => panic!("health bar width not in percent"),
    };
    current += (target - current) * rate;
    health_bar_style.size.width = Val::Percent(current);
}

fn score_update(mut score_ui: Query<&mut Text, With<ScoreUi>>, score: Res<Score>) {
    score_ui.single_mut().sections[0].value = format!("{}", score.0);
}

fn clean_ui(mut commands: Commands, ui_query: Query<Entity, With<Ui>>) {
    commands.entity(ui_query.single()).despawn_recursive();
}
