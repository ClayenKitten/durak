use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_systems(Update, button_system)
            .add_systems(OnEnter(MainMenuState::Main), main::setup)
            .add_systems(
                Update,
                main::menu_action.run_if(in_state(MainMenuState::Main)),
            )
            .add_systems(OnEnter(MainMenuState::CreateGame), create_game::setup)
            .add_systems(OnEnter(MainMenuState::JoinGame), join_game::setup)
            .add_systems(OnEnter(MainMenuState::Lobby), lobby::setup);
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MainMenuState {
    #[default]
    Main,
    CreateGame,
    JoinGame,
    Lobby,
}

mod main {
    use bevy::{app::AppExit, prelude::*};

    use super::MainMenuState;

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

        let mut container = commands.spawn(NodeBundle {
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
        });

        container.with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        ..default()
                    },
                    MenuButtonAction::Create,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section("Create", text_style.clone()));
                });
        });

        container.with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        ..default()
                    },
                    MenuButtonAction::Join,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section("Join", text_style.clone()));
                });
        });

        container.with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style,
                        ..default()
                    },
                    MenuButtonAction::Quit,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section("Quit", text_style));
                });
        });
    }

    pub fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut menu_state: ResMut<NextState<MainMenuState>>,
        mut exit: EventWriter<AppExit>,
    ) {
        for (interaction, action) in interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                match action {
                    MenuButtonAction::Create => todo!(),
                    MenuButtonAction::Join => todo!(),
                    MenuButtonAction::Quit => exit.send(AppExit),
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, Component)]
    pub enum MenuButtonAction {
        Create,
        Join,
        Quit,
    }
}

mod create_game {
    use bevy::prelude::*;

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {}
}

mod join_game {
    use bevy::prelude::*;

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {}
}

mod lobby {
    use bevy::prelude::*;

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {}
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
