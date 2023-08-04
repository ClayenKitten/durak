mod card;
mod deck;

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use durak_lib::game::{card::CardSuit, hand::Hand, player::Opponent, table::Table};

use crate::{
    network::{OnResponse, StatusRequest},
    session::Session,
    ui::game::display_ui,
    GameScreen,
};

use self::{card::CardData, deck::Deck};

/// Plugin that handles ongoing game management.
pub struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((card::CardPlugin, deck::DeckPlugin))
            .add_systems(OnEnter(GameScreen::Round), setup)
            .add_systems(
                Update,
                ((
                    request_status.run_if(on_timer(Duration::from_secs_f32(0.25))),
                    on_status_response,
                    display_ui,
                )
                    .run_if(in_state(GameScreen::Round)),),
            )
            .add_systems(OnExit(GameScreen::Round), cleanup);
    }
}

fn setup(mut commands: Commands, camera: Query<&OrthographicProjection>) {
    let area = camera.single().area;
    let hand_y = area.min.y + CardData::HEIGHT / 2. - CardData::HEIGHT / 3.;
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0., hand_y, 0.)),
            ..default()
        },
        Hand::default(),
    ));
    commands.spawn((SpatialBundle::default(), Table::default()));
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

/// Despawns everything connected to round.
fn cleanup(
    mut commands: Commands,
    deck: Query<Entity, With<Deck>>,
    hand: Query<Entity, With<Hand>>,
    table: Query<Entity, With<Table>>,
    opponents: Query<Entity, With<Opponent>>,
) {
    if let Ok(deck) = deck.get_single() {
        commands.entity(deck).despawn();
    }
    if let Ok(hand) = hand.get_single() {
        commands.entity(hand).despawn();
    }
    if let Ok(table) = table.get_single() {
        commands.entity(table).despawn();
    }
    for opponent in opponents.iter() {
        commands.entity(opponent).despawn_recursive();
    }
}

/// Trump suit for a round.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trump(pub CardSuit);
