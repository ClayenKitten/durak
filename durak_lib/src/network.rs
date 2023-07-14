//! Request and responce data structures used by both server and client.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{PlayerId, Card};

/// Token used to uniquely identify each player session.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Token {
    game_id: u64,
    player_id: PlayerId,
    secret: u32,
}

impl Token {
    pub fn new(game_id: u64, player_id: PlayerId, secret: u32) -> Self {
        Self {
            game_id,
            player_id,
            secret,
        }
    }
}

/// Query parameters used to create new game.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGameData {
    pub password: String,
}

/// Responce for [CreateGameData] request.
#[derive(Debug, Serialize, Deserialize)]
pub enum CreateGameResponce {
    Ok { token: Token },
}

/// Query parameters used to join created game.
#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGameData {
    pub id: u64,
    pub password: String,
}

/// Responce for [JoinGameData] request.
#[derive(Debug, Serialize, Deserialize)]
pub enum JoinGameResponce {
    /// Joined successfully.
    Ok { token: Token },
    /// Game with provided id doesn't exist.
    NotFound,
    /// Password doesn't match.
    InvalidPassword,
    /// Game is already full.
    TooManyPlayers,
}

/// Error that occured when attempted to join the game.
#[derive(Debug, Error)]
pub enum JoinGameError {
    #[error("game not found")]
    NotFound,
    #[error("password doesn't match")]
    InvalidPassword,
    #[error("too many players")]
    TooManyPlayers,
}

/// Query parameters used to play card.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayCardDate {
    pub player: PlayerId,
    pub card: Card,
}
