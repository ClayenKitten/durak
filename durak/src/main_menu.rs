use bevy::prelude::*;

use self::main::MenuButtonAction;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_systems(Update, button_system)
            .add_systems(
                Update,
                main::menu_action.run_if(not(in_state(MainMenuState::None))),
            )
            // Setup
            .add_systems(OnEnter(MainMenuState::Main), main::setup)
            .add_systems(OnEnter(MainMenuState::CreateGame), create_game::setup)
            .add_systems(OnEnter(MainMenuState::JoinGame), join_game::setup)
            .add_systems(OnEnter(MainMenuState::Lobby), lobby::setup)
            // Update
            .add_systems(
                Update,
                (
                    create_game::on_creation.run_if(in_state(MainMenuState::CreateGame)),
                    join_game::on_join.run_if(in_state(MainMenuState::JoinGame)),
                ),
            )
            // Cleanup
            .add_systems(OnExit(MainMenuState::Main), cleanup::<main::OnMainMenu>)
            .add_systems(
                OnExit(MainMenuState::CreateGame),
                cleanup::<create_game::OnCreateGameScreen>,
            )
            .add_systems(
                OnExit(MainMenuState::JoinGame),
                cleanup::<join_game::OnJoinGameScreen>,
            )
            .add_systems(
                OnExit(MainMenuState::Lobby),
                cleanup::<lobby::OnLobbyScreen>,
            );
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MainMenuState {
    #[default]
    Main,
    CreateGame,
    JoinGame,
    Lobby,
    None,
}

mod main {
    use bevy::{app::AppExit, prelude::*};
    use durak_lib::{
        common::GameId,
        network::{CreateGameData, JoinGameData},
    };

    use crate::{
        network::{CreateGameRequest, JoinGameRequest},
        GameScreen,
    };

    use super::{spawn_button, MainMenuState};

    pub fn setup(mut commands: Commands) {
        let mut container = commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.),
                    ..default()
                },
                ..default()
            },
            OnMainMenu,
        ));

        container.with_children(|parent| {
            spawn_button(parent, "Create", MenuButtonAction::GoToCreate);
            spawn_button(parent, "Join", MenuButtonAction::GoToJoin);
            spawn_button(parent, "Quit", MenuButtonAction::Quit);
        });
    }

    pub fn menu_action(
        mut commands: Commands,
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut game_state: ResMut<NextState<GameScreen>>,
        mut menu_state: ResMut<NextState<MainMenuState>>,
        mut exit: EventWriter<AppExit>,
    ) {
        for (interaction, action) in interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                match action {
                    MenuButtonAction::GoToCreate => {
                        menu_state.0 = Some(MainMenuState::CreateGame);
                    }
                    MenuButtonAction::Create { password } => {
                        let query = CreateGameData {
                            password: password.clone(),
                        };
                        commands.spawn(CreateGameRequest(query));
                    }
                    MenuButtonAction::GoToJoin => {
                        menu_state.0 = Some(MainMenuState::JoinGame);
                    }
                    MenuButtonAction::Join { id, password } => {
                        let query = JoinGameData {
                            id: *id,
                            password: password.clone(),
                        };
                        commands.spawn(JoinGameRequest(query));
                    }
                    MenuButtonAction::GoToMainMenu => {
                        menu_state.0 = Some(MainMenuState::Main);
                    }
                    MenuButtonAction::StartTheGame => {
                        menu_state.0 = Some(MainMenuState::None);
                        game_state.0 = Some(GameScreen::RoundSetup);
                    }
                    MenuButtonAction::Quit => exit.send(AppExit),
                }
            }
        }
    }

    #[derive(Debug, Component)]
    pub enum MenuButtonAction {
        GoToCreate,
        Create { password: String },
        GoToJoin,
        Join { id: GameId, password: String },
        GoToMainMenu,
        StartTheGame,
        Quit,
    }

    /// Marker component used for cleanup.
    #[derive(Debug, Clone, Copy, Component)]
    pub struct OnMainMenu;
}

mod create_game {
    use bevy::prelude::*;

    use crate::network::{CreateGameRequest, OnResponce};

    use super::{main::MenuButtonAction, spawn_button, MainMenuState};

    pub fn setup(mut commands: Commands) {
        let mut container = commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.),
                    ..default()
                },
                ..default()
            },
            OnCreateGameScreen,
        ));

        container.with_children(|parent| {
            spawn_button(
                parent,
                "Create",
                MenuButtonAction::Create {
                    password: String::from("password"),
                },
            );
            spawn_button(parent, "Return", MenuButtonAction::GoToMainMenu);
        });
    }

    pub fn on_creation(
        mut state: ResMut<NextState<MainMenuState>>,
        mut event_reader: EventReader<OnResponce<CreateGameRequest>>,
    ) {
        for OnResponce(responce) in event_reader.iter() {
            state.0 = Some(MainMenuState::Lobby);
        }
    }

    /// Marker component used for cleanup.
    #[derive(Debug, Clone, Copy, Component)]
    pub struct OnCreateGameScreen;
}

mod join_game {
    use bevy::prelude::*;
    use durak_lib::common::GameId;

    use crate::network::{JoinGameRequest, OnResponce};

    use super::{main::MenuButtonAction, spawn_button, MainMenuState};

    pub fn setup(mut commands: Commands) {
        let mut container = commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.),
                    ..default()
                },
                ..default()
            },
            OnJoinGameScreen,
        ));

        container.with_children(|parent| {
            spawn_button(
                parent,
                "Join",
                MenuButtonAction::Join {
                    id: GameId::new(0),
                    password: String::from("password"),
                },
            );
            spawn_button(parent, "Return", MenuButtonAction::GoToMainMenu);
        });
    }

    pub fn on_join(
        mut state: ResMut<NextState<MainMenuState>>,
        mut event_reader: EventReader<OnResponce<JoinGameRequest>>,
    ) {
        for OnResponce(responce) in event_reader.iter() {
            state.0 = Some(MainMenuState::Lobby);
        }
    }

    /// Marker component used for cleanup.
    #[derive(Debug, Clone, Copy, Component)]
    pub struct OnJoinGameScreen;
}

mod lobby {
    use bevy::prelude::*;
    use durak_lib::common::PlayerId;

    use crate::main_menu::Colors;

    use super::{main::MenuButtonAction, spawn_button};

    pub fn setup(mut commands: Commands) {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnLobbyScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Px(100.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Colors::PRIMARY.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                "Game #124252115",
                                TextStyle {
                                    font_size: 40.,
                                    color: Colors::TEXT,
                                    ..default()
                                },
                            ),
                            ..default()
                        });
                    });
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_grow: 1.,
                            flex_direction: FlexDirection::Column,
                            width: Val::Percent(100.),
                            height: Val::Auto,
                            border: UiRect::all(Val::Px(10.)),
                            row_gap: Val::Px(10.),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        spawn_player_entry(parent, PlayerId::new(0), true);
                        spawn_player_entry(parent, PlayerId::new(1), false);
                        spawn_player_entry(parent, PlayerId::new(2), false);
                        spawn_player_entry(parent, PlayerId::new(3), false);
                    });
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Auto,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        background_color: Colors::PRIMARY.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        spawn_button(parent, "Leave", MenuButtonAction::GoToMainMenu);
                        spawn_button(parent, "Start", MenuButtonAction::StartTheGame);
                    });
            });

        fn spawn_player_entry(parent: &mut ChildBuilder, player: PlayerId, is_host: bool) {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Px(100.),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Colors::BACKGROUND.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Percent(100.),
                                aspect_ratio: Some(1.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Colors::PRIMARY.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    player.to_string(),
                                    TextStyle {
                                        font_size: 40.,
                                        color: Colors::TEXT,
                                        ..default()
                                    },
                                ),
                                ..default()
                            });
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                align_items: AlignItems::Center,
                                padding: UiRect::left(Val::Px(20.)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    format!("Player #{}", player),
                                    TextStyle {
                                        font_size: 40.,
                                        color: Colors::TEXT,
                                        ..default()
                                    },
                                ),
                                ..default()
                            });
                        });

                    if is_host {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    height: Val::Percent(100.),
                                    aspect_ratio: Some(1.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Colors::PRIMARY.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text::from_section(
                                        "Host",
                                        TextStyle {
                                            font_size: 40.,
                                            color: Colors::TEXT,
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                });
                            });
                    }
                });
        }
    }

    /// Marker component used for cleanup.
    #[derive(Debug, Clone, Copy, Component)]
    pub struct OnLobbyScreen;
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct Colors;

impl Colors {
    pub const PRIMARY: Color = Color::rgb(0.2, 0.2, 0.2);
    pub const SECONDARY: Color = Color::rgb(0.5, 0.5, 0.5);
    pub const BACKGROUND: Color = Color::rgb(0.5, 0.5, 0.5);
    pub const TEXT: Color = Color::rgb(0.9, 0.9, 0.9);
}

fn spawn_button(parent: &mut ChildBuilder, text: &str, action: MenuButtonAction) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let text_style = TextStyle {
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
        ..default()
    };

    parent
        .spawn((
            ButtonBundle {
                style: button_style,
                ..default()
            },
            action,
        ))
        .with_children(|button| {
            button.spawn(TextBundle::from_section(text, text_style));
        });
}
