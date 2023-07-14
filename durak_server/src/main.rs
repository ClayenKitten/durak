pub mod state;

use durak_lib::{
    common::{Card, PlayerId},
    network::{CreateGameData, JoinGameData, CreateGameResponce, Token, JoinGameResponce},
    CardRank, CardSuit,
};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};
use state::Games;
use std::net::SocketAddr;
use strum::IntoEnumIterator;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/create", post(create_game))
        .route("/join", post(join_game))
        .fallback(not_found)
        .with_state(Games::default());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}

async fn create_game(State(games): State<Games>, Query(data): Query<CreateGameData>) -> String {
    let game_id = games.create(data.password);
    tracing::debug!("created game `{game_id}`");
    let player_id = PlayerId::new(0);
    serde_json::to_string(
        &CreateGameResponce::Ok {
            token: generate_token(game_id, player_id)
        }
    ).unwrap()
}

async fn join_game(State(games): State<Games>, Query(data): Query<JoinGameData>) -> String {
    use state::JoinGameResult::*;
    let responce = match games.join(data.id, data.password) {
        Ok(player_id) => {
            tracing::debug!("joined game `{}`", data.id);
            JoinGameResponce::Ok { token: generate_token(data.id, player_id) }
        },
        NotFound => {
            tracing::debug!("attempted to join nonexisting game `{}`", data.id);
            JoinGameResponce::NotFound
        },
        InvalidPassword => {
            tracing::debug!("attempted to join with wrong password`{}`", data.id);
            JoinGameResponce::InvalidPassword
        },
        TooManyPlayers => {
            tracing::debug!("attempted to join full game `{}`", data.id);
            JoinGameResponce::TooManyPlayers
        },
    };
    serde_json::to_string(&responce).unwrap()
}

fn generate_token(game_id: u64, player_id: PlayerId) -> Token {
    Token::new(game_id, player_id, thread_rng().gen())
}

#[derive(Debug)]
pub struct Game {
    password: String,
    trump: CardSuit,
    state: GameState,
    deck: Deck,
    table: Vec<(Card, Option<Card>)>,
    players: Vec<Player>,
}

impl Game {
    /// Creates new game with provided password set.
    pub fn new(password: String) -> Self {
        Self {
            password,
            trump: Self::pick_trump(),
            state: GameState::Created,
            deck: Self::create_deck(),
            table: Vec::with_capacity(6),
            players: vec![Player {
                id: PlayerId::new(0),
                hand: Hand::default(),
            }],
        }
    }

    /// Adds new player to the game.
    ///
    /// Returns `None` if game is already full.
    /// Otherwise, returns [PlayerId] of the new player.
    pub fn add_player(&mut self) -> Option<PlayerId> {
        if self.players.len() > 1 {
            return None;
        }
        let id = PlayerId::new(self.players.len() as u8);
        self.players.push(Player {
            id,
            hand: Hand::default(),
        });
        Some(id)
    }

    fn pick_trump() -> CardSuit {
        match rand::thread_rng().gen_range(0..=3) {
            0 => CardSuit::Clover,
            1 => CardSuit::Diamond,
            2 => CardSuit::Heart,
            3 => CardSuit::Pike,
            _ => unreachable!(),
        }
    }

    fn create_deck() -> Deck {
        let mut cards = Vec::with_capacity(36);
        for suit in CardSuit::iter() {
            for rank in CardRank::iter() {
                cards.push(Card { suit, rank });
            }
        }
        Deck(cards.try_into().unwrap())
    }
}

/// State of the game.
#[derive(Debug, Serialize, Deserialize)]
pub enum GameState {
    /// Game is created, but not enough players connected.
    Created,
    /// Game is ready to start on host's command.
    ReadyToStart,
    /// Expecting specified player's action.
    ExpectAction(PlayerId),
    /// Game is ended.
    Completed { win: PlayerId },
}

#[derive(Debug)]
struct Deck(pub [Card; 36]);

#[derive(Debug)]
struct Player {
    pub id: PlayerId,
    pub hand: Hand,
}

#[derive(Debug, Default)]
struct Hand(pub Vec<Card>);
