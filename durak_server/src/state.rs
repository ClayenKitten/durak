//! State of the server.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::extract::FromRef;
use durak_lib::{
    errors::{AccessError, GameNotFound},
    identifiers::{GameId, PlayerId},
    network::Token,
};
use rand::{thread_rng, Rng};

use crate::game::{lobby::LobbyState, round::RoundState, Game};

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
    pub fn create(&self, name: String) -> GameId {
        let id = GameId::new(rand::thread_rng().gen());
        let mut games = self.0.lock().unwrap();

        let mut game = Game::new();
        game.lobby_state().unwrap().add_player(name);

        games.insert(id, game);
        id
    }

    /// Evaluates provided function with mutable reference to game.
    pub fn with_game<T>(
        &self,
        id: GameId,
        func: impl FnOnce(&mut Game) -> T,
    ) -> Result<T, GameNotFound> {
        let mut games = self.0.lock().unwrap();
        let game = games.get_mut(&id).ok_or(GameNotFound)?;
        Ok(func(game))
    }

    /// Evaluates provided function with mutable reference to [LobbyState].
    pub fn with_lobby_game<T>(
        &self,
        id: GameId,
        func: impl FnOnce(&mut LobbyState) -> T,
    ) -> Result<T, AccessError> {
        let mut games = self.0.lock().unwrap();
        let game = games.get_mut(&id).ok_or(GameNotFound)?;
        let round = game.lobby_state()?;
        Ok(func(round))
    }

    /// Evaluates provided function with mutable reference to [RoundState].
    pub fn with_started_game<T>(
        &self,
        id: GameId,
        func: impl FnOnce(&mut RoundState) -> T,
    ) -> Result<T, AccessError> {
        let mut games = self.0.lock().unwrap();
        let game = games.get_mut(&id).ok_or(GameNotFound)?;
        let round = game.round_state()?;
        Ok(func(round))
    }
}

impl FromRef<AppState> for Games {
    fn from_ref(input: &AppState) -> Self {
        input.games.clone()
    }
}

/// State that stores all authentication tokens and associated data.
#[derive(Clone)]
pub struct Auth {
    passwords: Arc<Mutex<HashMap<GameId, String>>>,
    sessions: Arc<Mutex<HashMap<Token, (GameId, PlayerId)>>>,
}

impl Auth {
    /// Creates new auth state manager.
    fn new() -> Self {
        let passwords = HashMap::with_capacity(64);
        let sessions = HashMap::with_capacity(128);
        Self {
            passwords: Arc::new(Mutex::new(passwords)),
            sessions: Arc::new(Mutex::new(sessions)),
        }
    }

    /// Creates new password entry.
    ///
    /// # Panics
    ///
    /// Panics if called more than once for provided [GameId].
    pub fn store_password(&self, game_id: GameId, password: String) {
        let mut passwords = self.passwords.lock().unwrap();
        if passwords.insert(game_id, password).is_some() {
            panic!("Attempted to create new password entry when it is already present");
        }
    }

    /// Generates new token.
    pub fn generate_token(&self, game_id: GameId, player_id: PlayerId) -> Token {
        let mut sessions = self.sessions.lock().unwrap();
        let token = Token::new(thread_rng().gen());
        sessions.insert(token, (game_id, player_id));
        token
    }

    /// Returns `true` if provided password matches stored one.
    pub fn validate_password(&self, game_id: GameId, password: &str) -> bool {
        let passwords = self.passwords.lock().unwrap();
        if let Some(stored_password) = passwords.get(&game_id) {
            stored_password.as_str() == password
        } else {
            false
        }
    }

    /// Returns `true` if provided token exists and matches provided [GameId] and [PlayerId].
    pub fn validate_session(&self, token: Token, game_id: GameId, player_id: PlayerId) -> bool {
        let sessions = self.sessions.lock().unwrap();
        let Some(data) = sessions.get(&token) else {
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
