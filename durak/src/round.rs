mod card;
mod deck;
mod setup;

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use durak_lib::game::{card::CardSuit, hand::Hand, table::Table};

use crate::{
    network::{OnResponse, StatusRequest},
    session::Session,
    ui::game::display_ui,
    GameScreen,
};

use self::deck::Deck;

/// Plugin that handles ongoing game management.
pub struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((setup::RoundSetupPlugin, card::CardPlugin, deck::DeckPlugin))
            .add_systems(
                Update,
                (
                    request_status.run_if(on_timer(Duration::from_secs_f32(0.25))),
                    on_status_response,
                    display_ui,
                )
                    .run_if(in_state(GameScreen::Round)),
            );
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
    let Some(OnResponse(status)) = response.iter().next() else {
        return;
    };

    let mut hand = hand.single_mut();
    *hand = status.hand.clone();

    let mut table = table.single_mut();
    *table = status.table.clone();

    let mut deck = deck.single_mut();
    deck.left = status.deck_size;
}

/// Trump suit for a round.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trump(pub CardSuit);
