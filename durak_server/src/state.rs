//! State of the server.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::extract::FromRef;
use durak_lib::{
    common::{GameId, PlayerId},
    network::Token,
};
use rand::{thread_rng, Rng};

use crate::game::Game;

#[derive(Clone)]
pub struct AppState {
    pub(crate) auth: Auth,
    pub(crate) games: Games,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            auth: Auth::new(),
            games: Games::new(),
        }
    }
}

/// Collection of all ongoing games.
#[derive(Debug, Clone)]
pub struct Games(Arc<Mutex<HashMap<GameId, Game>>>);

impl Games {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::with_capacity(128))))
    }

    /// Creates new game with provided password.
    pub fn create(&self, password: String) -> GameId {
        let id = GameId::new(rand::thread_rng().gen());
        let mut games = self.0.lock().unwrap();
        games.insert(id, Game::new(password));
        id
    }

    /// Evalutes provided function with mutable reference to game.
    pub fn with_game<T>(&self, id: GameId, func: impl FnOnce(&mut Game) -> T) -> Option<T> {
        let mut games = self.0.lock().unwrap();
        let game = games.get_mut(&id)?;
        Some(func(game))
    }
}

impl FromRef<AppState> for Games {
    fn from_ref(input: &AppState) -> Self {
        input.games.clone()
    }
}

/// State that stores all authentication tokens and associated data.
#[derive(Clone)]
pub struct Auth(Arc<Mutex<HashMap<Token, (GameId, PlayerId)>>>);

impl Auth {
    fn new() -> Self {
        let map = HashMap::with_capacity(1024);
        Self(Arc::new(Mutex::new(map)))
    }

    /// Generates new token.
    pub fn generate_token(&self, game_id: GameId, player_id: PlayerId) -> Token {
        let mut auth = self.0.lock().unwrap();
        let token = Token::new(thread_rng().gen());
        auth.insert(token, (game_id, player_id));
        token
    }

    /// Returns `true` if provided token exists and matches provided [GameId] and [PlayerId].
    pub fn validate(&self, token: Token, game_id: GameId, player_id: PlayerId) -> bool {
        let auth = self.0.lock().unwrap();
        let Some(data) = auth.get(&token) else {
            return false;
        };
        data.0 == game_id && data.1 == player_id
    }
}

impl FromRef<AppState> for Auth {
    fn from_ref(input: &AppState) -> Self {
        input.auth.clone()
    }
}
