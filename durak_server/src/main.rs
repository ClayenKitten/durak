pub mod state;

use durak_lib::{
    common::{Card, PlayerId},
    network::{CreateGameData, JoinGameData},
    CardRank, CardSuit,
};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use rand::Rng;
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
    let id = games.create(data.password);
    tracing::debug!("created game `{id}`");
    id.to_string()
}

async fn join_game(State(games): State<Games>, Query(data): Query<JoinGameData>) -> &'static str {
    if games.join(data.id, data.password) {
        tracing::debug!("joined game `{}`", data.id);
        "Joined successfully"
    } else {
        tracing::debug!("failed to join game `{}`", data.id);
        "Error"
    }
}

/// Collection of all ongoing games.
#[derive(Debug, Clone, Default)]
struct Games(Arc<Mutex<HashMap<u64, Game>>>);

impl Games {
    /// Creates new game with provided password.
    pub fn create(&self, password: String) -> u64 {
        let id = rand::thread_rng().gen();
        let mut games = self.0.lock().unwrap();
        games.insert(id, Game::new(password));
        id
    }

    /// Attempts to join existing game with id and password.
    ///
    /// Returns `true` if joined successfuly.
    pub fn join(&self, id: u64, password: String) -> bool {
        let mut games = self.0.lock().unwrap();
        let Some(game) = games.get_mut(&id) else {
            return false;
        };
        if game.password != password {
            return false;
        }
        game.add_player().is_some()
    }
}

#[derive(Debug)]
struct Game {
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
