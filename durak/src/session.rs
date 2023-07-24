use bevy::prelude::*;
use durak_lib::{
    identifiers::{GameId, PlayerId},
    network::{AuthHeader, Token},
};

#[derive(Resource, Debug)]
pub struct Session {
    pub name: String,
    pub id: PlayerId,
    pub game: GameId,
    pub token: Token,
    pub is_host: bool,
}

impl Session {
    pub fn into_header(&self) -> AuthHeader {
        AuthHeader {
            game_id: self.game,
            player_id: self.id,
            token: self.token,
        }
    }
}
