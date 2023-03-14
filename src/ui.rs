use bevy::{app::AppExit, prelude::*};

use crate::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(game_ui).add_systems((
            button_new_game_system,
            button_exit_system,
            on_game_state_change,
        ));
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_GOOD_BUTTON: Color =
    Color::rgb(0.35, 0.75, 0.35);
const PRESSED_DANGER_BUTTON: Color =
    Color::rgb(0.75, 0.35, 0.35);

#[derive(Debug, Component)]
struct ButtonNewGame;

#[derive(Debug, Component)]
struct ButtonExit;

#[derive(Debug, Component)]
struct Menu;

fn game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(
                    Val::Percent(100.0),
                    Val::Percent(100.0),
                ),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(10.0)),
                gap: Size::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(
                            Val::Px(250.0),
                            Val::Px(250.0),
                        ),
                        justify_content:
                            JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load(
                                "fonts/AlfaSlabOne-Regular.ttf",
                            ),
                            font_size: 40.0,
                            color: Color::rgb(
                                0.9, 0.9, 0.9,
                            ),
                        },
                    ));
                }).insert(ButtonNewGame);

                parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(
                            Val::Px(250.0),
                            Val::Px(250.0),
                        ),
                        justify_content:
                            JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load(
                                "fonts/AlfaSlabOne-Regular.ttf",
                            ),
                            font_size: 40.0,
                            color: Color::rgb(
                                0.9, 0.9, 0.9,
                            ),
                        },
                    ));
                }).insert(ButtonExit);
        }).insert(Menu);
}

fn button_new_game_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
        ),
        (
            Changed<Interaction>,
            With<ButtonNewGame>,
        ),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, children) in
        &mut interaction_query
    {
        let mut text =
            text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value =
                    "Starting".to_string();
                *color = PRESSED_GOOD_BUTTON.into();

                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                text.sections[0].value =
                    "Start".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value =
                    "New Game".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
fn button_exit_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
        ),
        (Changed<Interaction>, With<ButtonExit>),
    >,
    mut exit: EventWriter<AppExit>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in
        &mut interaction_query
    {
        let mut text =
            text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value =
                    "Exiting".to_string();
                *color = PRESSED_DANGER_BUTTON.into();
                exit.send(AppExit)
            }
            Interaction::Hovered => {
                text.sections[0].value = "Exit".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Exit".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn on_game_state_change(
    game_state: Res<State<GameState>>,
    mut game_menu: Query<&mut Visibility, With<Menu>>,
) {
    if game_state.is_changed() {
        for mut game_menu in game_menu.iter_mut() {
            match game_state.0 {
                GameState::Menu => {
                    *game_menu = Visibility::Visible
                }
                GameState::Playing => {
                    *game_menu = Visibility::Hidden
                }
                GameState::Paused => {
                    *game_menu = Visibility::Visible
                }
            }
        }
    }
}
