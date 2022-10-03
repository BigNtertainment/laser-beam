use crate::loading::FontAssets;
use crate::score::Score;
use crate::GameState;
use bevy::prelude::*;

pub struct GameOver;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for GameOver {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(GameState::GameOver).with_system(click_play_button),
            )
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(cleanup_menu));
    }
}

#[derive(Component)]
struct GameOverMenu;

struct ButtonColors {
    normal: UiColor,
    hovered: UiColor,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    score: Res<Score>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // Game Over
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Game Over".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 72.0,
                            color: Color::WHITE,
                        },
                    }],
                    alignment: Default::default(),
                },
                style: Style {
                    margin: UiRect {
                        bottom: Val::Percent(3.5),
                        ..default()
                    },
                    ..default()
                },
                ..Default::default()
            });

            // Your Score
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: format!("Your score: {}", score.0),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 36.0,
                            color: Color::WHITE,
                        },
                    }],
                    alignment: Default::default(),
                },
                style: Style {
                    margin: UiRect {
                        bottom: Val::Percent(5.),
                        ..default()
                    },
                    ..default()
                },
                ..Default::default()
            });

            // Menu button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: button_colors.normal,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Menu".to_string(),
                                style: TextStyle {
                                    font: font_assets.fira_sans.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            }],
                            alignment: Default::default(),
                        },
                        ..Default::default()
                    });
                });

            // Menu button
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Menu".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        })
        .insert(GameOverMenu);
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Menu).unwrap();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<GameOverMenu>>) {
    commands.entity(button.single()).despawn_recursive();
}
