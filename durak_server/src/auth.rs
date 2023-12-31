use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};
use durak_lib::{
    identifiers::{GameId, PlayerId},
    network::AuthHeader,
};

use crate::state::AppState;

/// Extractor that authenticates player.
pub struct Authenticate(pub AuthentificatedPlayer);

#[async_trait]
impl FromRequestParts<AppState> for Authenticate {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        const REJECTION: (StatusCode, &str) = (StatusCode::UNAUTHORIZED, "Authorization failed");

        let Some(header) = parts.headers.get(AUTHORIZATION) else {
            tracing::debug!("Failed auth");
            return Err(REJECTION);
        };
        let Ok(header) = header.to_str() else {
            tracing::debug!("Failed auth");
            return Err(REJECTION);
        };
        let Ok(AuthHeader { game_id, player_id, token }) = serde_json::from_str::<AuthHeader>(header) else {
            tracing::debug!("Failed auth");
            return Err(REJECTION);
        };
        if !state.auth.validate_session(token, game_id, player_id) {
            tracing::debug!("Failed auth for game `{game_id}`");
            return Err(REJECTION);
        }

        Ok(Authenticate(AuthentificatedPlayer { game_id, player_id }))
    }
}

/// Extractor that authenticates player and ensures that it is host of the game.
pub struct AuthenticateHost(pub AuthentificatedPlayer);

#[async_trait]
impl FromRequestParts<AppState> for AuthenticateHost {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        const REJECTION: (StatusCode, &str) = (StatusCode::UNAUTHORIZED, "Authorization failed");

        let Authenticate(authenticated) = Authenticate::from_request_parts(parts, state).await?;
        if authenticated.player_id.is_host() {
            Ok(AuthenticateHost(authenticated))
        } else {
            tracing::info!(
                "Failed auth for game `{}`: not a host",
                authenticated.game_id
            );
            Err(REJECTION)
        }
    }
}

#[derive(Debug)]
pub struct AuthentificatedPlayer {
    pub game_id: GameId,
    pub player_id: PlayerId,
}
