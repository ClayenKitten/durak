//! State of the server.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use durak_lib::{common::PlayerId, network::JoinGameError};
use rand::Rng;

use crate::game::Game;

/// Collection of all ongoing games.
#[derive(Debug, Clone, Default)]
pub struct Games(Arc<Mutex<HashMap<u64, Game>>>);

impl Games {
    /// Creates new game with provided password.
    pub fn create(&self, password: String) -> u64 {
        let id = rand::thread_rng().gen();
        let mut games = self.0.lock().unwrap();
        games.insert(id, Game::new(password));
        id
    }

    /// Evalutes provided function with mutable reference to game.
    pub fn with_game<T>(&self, id: u64, func: impl FnOnce(&mut Game) -> T) -> Option<T> {
        let mut games = self.0.lock().unwrap();
        let game = games.get_mut(&id)?;
        Some(func(game))
    }
}
