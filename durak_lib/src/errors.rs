use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Failed to access some data.
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AccessError {
    #[error("authorization failed: {0}")]
    AuthFailed(#[from] AuthFailed),
    #[error("{0}")]
    GameNotFound(#[from] GameNotFound),
    #[error("{0}")]
    InvalidPhase(#[from] InvalidPhase),
}

/// Token authorization failed
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AuthFailed {
    #[error("request didn't contain Authorization header")]
    NoAuthHeader,
    #[error("malformed Authorization header")]
    InvalidAuthHeader,
    #[error("provided token doesn't match")]
    TokenDoesntMatch,
}

/// Game was not found.
#[derive(Debug, Error, Serialize, Deserialize)]
#[error("game with provided id wasn't found")]
pub struct GameNotFound;

/// Invalid game phase.
#[derive(Debug, Error, Serialize, Deserialize)]
#[error("requested endpoint is not accessible during current phase")]
pub struct InvalidPhase;
