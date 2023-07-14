//! Request and responce data structures used by both server and client.

use serde::{Deserialize, Serialize};

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
