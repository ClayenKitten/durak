use durak_lib::{identifiers::PlayerId, status::LobbyPlayerData};

#[derive(Debug)]
pub struct FinishedState {
    winner: PlayerId,
    players: Vec<LobbyPlayerData>,
}

impl FinishedState {
    pub fn winner(&self) -> LobbyPlayerData {
        self.players
            .iter()
            .find(|player| player.id == self.winner)
            .expect("Winner id should be found inside player")
            .clone()
    }
}
