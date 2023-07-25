mod card;
mod deck;

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use durak_lib::{
    game::{card::CardSuit, hand::Hand, table::Table},
    status::GameState,
};

use crate::{
    network::{OnResponse, StateRequest, StatusRequest},
    session::Session,
    ui::game::display_ui,
    GameEnded, GameScreen,
};

use self::deck::Deck;

/// Plugin that handles ongoing game management.
pub struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((card::CardPlugin, deck::DeckPlugin))
            .add_systems(OnEnter(GameScreen::Round), setup)
            .add_systems(
                Update,
                (
                    request_status.run_if(on_timer(Duration::from_secs_f32(0.25))),
                    request_state.run_if(on_timer(Duration::from_secs_f32(2.0))),
                    on_status_response,
                    on_state_response,
                    display_ui,
                )
                    .run_if(in_state(GameScreen::Round)),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Hand::default());
    commands.spawn(Table::default());
}

fn request_state(session: Res<Session>, mut commands: Commands) {
    commands.spawn(StateRequest(session.into_header()));
}

fn on_state_response(
    mut response: EventReader<OnResponse<StateRequest>>,
    mut writer: EventWriter<GameEnded>,
) {
    let Some(OnResponse(state)) = response.iter().last() else {
        return;
    };
    match state {
        GameState::Completed {
            winner_id,
            winner_name,
        } => {
            writer.send(GameEnded {
                winner_id: *winner_id,
                winner_name: winner_name.clone(),
            });
        }
        _ => {}
    }
}

fn request_status(session: Res<Session>, mut commands: Commands) {
    commands.spawn(StatusRequest(session.into_header()));
}

fn on_status_response(
    mut response: EventReader<OnResponse<StatusRequest>>,
    mut table: Query<&mut Table>,
    mut hand: Query<&mut Hand>,
    mut deck: Query<&mut Deck>,
) {
    let Some(OnResponse(status)) = response.iter().last() else {
        return;
    };

    let mut hand = hand.single_mut();
    *hand = status.hand.clone();

    let mut table = table.single_mut();
    *table = status.table.clone();

    let mut deck = deck.single_mut();
    if deck.left != status.deck_size {
        deck.left = status.deck_size;
    }
}

/// Trump suit for a round.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trump(pub CardSuit);
