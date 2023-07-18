//! Identifiers used to distinguish different entities.

use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

/// A unique identificator of the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GameId(pub u64);

impl GameId {
    pub fn new(id: u64) -> Self {
        GameId(id)
    }
}

impl FromStr for GameId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }
        let mut result = 0;
        for (index, char) in s.chars().rev().enumerate() {
            let Some(digit) = char.to_digit(16) else {
                return Err(());
            };
            result += digit as u64 * 16u64.pow(index as u32);
        }
        Ok(GameId(result))
    }
}

impl Display for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

/// The unique identifier of the player within the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayerId(pub u8);

impl PlayerId {
    pub fn new(id: u8) -> Self {
        PlayerId(id)
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::identifiers::GameId;

    #[test]
    fn test_game_id_decoding() {
        let game_id = GameId::new(25);
        let s = game_id.to_string();
        let parsed = GameId::from_str(&s).unwrap();
        assert_eq!(game_id, parsed);
    }
}