//! Request and responce data structures used by both server and client.

use serde::{Deserialize, Serialize};

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

/// Query parameters used to join created game.
#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGameData {
    pub id: u64,
    pub password: String,
}
