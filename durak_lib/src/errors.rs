use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
#[error("failed to play card")]
pub enum PlayCardError {
    #[error("too much attacking cards: only six cards can attack at time")]
    TooMuchAttackingCards,
    #[error("it is turn of another player")]
    InvalidTurn,
    #[error("that card can't be placed at the table")]
    CantPlace,
    #[error("that card is not in hand")]
    NotInHand,
}

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

impl AccessError {
    pub fn status_code(&self) -> http::StatusCode {
        use http::StatusCode;
        match self {
            AccessError::AuthFailed(_) => StatusCode::UNAUTHORIZED,
            AccessError::GameNotFound(_) => StatusCode::NOT_FOUND,
            AccessError::InvalidPhase(_) => StatusCode::BAD_REQUEST,
        }
    }
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
