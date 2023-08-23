use durak_lib::{identifiers::PlayerId, status::PlayerData};

#[derive(Debug)]
pub struct FinishedState {
    pub winner: PlayerId,
    pub players: Vec<PlayerData>,
}

impl FinishedState {
    pub fn winner(&self) -> PlayerData {
        self.players
            .iter()
            .find(|player| player.id == self.winner)
            .expect("Winner id should be found inside player")
            .clone()
    }
}
