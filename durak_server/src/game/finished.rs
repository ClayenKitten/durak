use durak_lib::{
    identifiers::PlayerId,
    status::{finished::FinishedStatus, PlayerData},
};

#[derive(Debug)]
pub struct FinishedState {
    pub winner: PlayerId,
    pub players: Vec<PlayerData>,
}

impl FinishedState {
    /// Generates status report.
    pub fn status(&self) -> FinishedStatus {
        FinishedStatus {
            winner: self.winner,
            players: self.players.clone(),
        }
    }

    pub fn winner(&self) -> PlayerData {
        self.players
            .iter()
            .find(|player| player.id == self.winner)
            .expect("Winner id should be found inside player")
            .clone()
    }
}
