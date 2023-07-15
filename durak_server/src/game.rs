//! Game state and logic.

use durak_lib::{
    common::{Card, PlayerId},
    network::{GameState, JoinGameError},
    CardRank, CardSuit,
};
use rand::{seq::SliceRandom, thread_rng, Rng};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Game {
    pub host: PlayerId,
    pub password: String,
    pub state: GameState,
    trump: CardSuit,
    deck: Deck,
    players: Vec<Player>,
    round: Option<RoundState>,
}

impl Game {
    /// Creates new game with provided password set.
    pub fn new(password: String) -> Self {
        Self {
            host: PlayerId::new(0),
            password,
            trump: Self::pick_trump(),
            state: GameState::Created,
            deck: Deck::new(),
            players: vec![Player {
                id: PlayerId::new(0),
                hand: Hand::default(),
            }],
            round: None,
        }
    }

    /// Attempts to join existing game with id and password.
    ///
    /// Returns [PlayerId] if successful.
    pub fn join(&mut self, password: String) -> Result<PlayerId, JoinGameError> {
        if self.password != password {
            return Err(JoinGameError::InvalidPassword);
        }
        self.add_player().ok_or(JoinGameError::TooManyPlayers)
    }

    /// Adds new player to the game.
    ///
    /// Returns `None` if game is already full.
    /// Otherwise, returns [PlayerId] of the new player.
    pub fn add_player(&mut self) -> Option<PlayerId> {
        if self.players.len() > 1 {
            return None;
        }
        let id = PlayerId::new(self.players.len() as u8);
        self.players.push(Player {
            id,
            hand: Hand::default(),
        });
        Some(id)
    }

    /// Starts the game.
    ///
    /// Deck is shuffled and cards are dealed.
    pub fn start(&mut self) {
        self.deck.shuffle();
        for player in self.players.iter_mut() {
            for _ in 0..6 {
                let card = self.deck.take().unwrap();
                player.hand.add(card)
            }
        }

        let mut players = self.players.iter();
        let attacker = players
            .next()
            .expect("at least two players are required to start the game")
            .id;
        let defender = players
            .next()
            .expect("at least two players are required to start the game")
            .id;

        self.round = Some(RoundState {
            table: Table::new(),
            attacker,
            defender,
        });
        // TODO: follow game's rules about first player.
        self.state = GameState::ExpectAction {
            table: Vec::new(),
            player: attacker,
        };
    }

    /// Places card on the table.
    ///
    /// Returns `true` if played successfully.
    pub fn play_card(&mut self, player_id: PlayerId, card: Card) -> bool {
        let Some(ref mut round) = self.round else {
            return false;
        };
        let Some(player) = self.players.iter_mut().find(|player| player.id == player_id) else {
            return false;
        };

        if round.attacker == player_id {
            player.hand.remove(card);
            round.table.attack(card)
        } else if round.defender == player_id {
            player.hand.remove(card);
            round.table.defend(card, self.trump)
        } else {
            false
        }
    }

    pub fn retreat(&mut self, player_id: PlayerId) -> bool {
        let Some(ref mut round) = self.round else {
            return false;
        };
        if round.attacker != player_id {
            return false;
        }
        if round.table.retreat() {
            round.swap_players();
            self.state = GameState::ExpectAction {
                player: round.defender,
                table: round.table.0.clone(),
            };
            true
        } else {
            false
        }
    }

    pub fn take(&mut self, player_id: PlayerId) -> bool {
        let Some(ref mut round) = self.round else {
            return false;
        };
        let Some(player) = self.players.iter_mut().find(|player| player.id == player_id) else {
            return false;
        };

        if round.defender != player_id {
            return false;
        }
        let Some(cards) = round.table.take() else {
            return false;
        };
        for card in cards {
            player.hand.add(card);
        }
        self.state = GameState::ExpectAction {
            player: round.attacker,
            table: round.table.0.clone(),
        };
        true
    }

    fn pick_trump() -> CardSuit {
        match rand::thread_rng().gen_range(0..=3) {
            0 => CardSuit::Clover,
            1 => CardSuit::Diamond,
            2 => CardSuit::Heart,
            3 => CardSuit::Pike,
            _ => unreachable!(),
        }
    }
}

/// State of the game that is unique to the round.
#[derive(Debug)]
struct RoundState {
    pub table: Table,
    pub attacker: PlayerId,
    pub defender: PlayerId,
}

impl RoundState {
    /// Swaps attacker and defender.
    // TODO: allow more than two players.
    pub fn swap_players(&mut self) {
        std::mem::swap(&mut self.attacker, &mut self.defender)
    }
}

#[derive(Debug)]
struct Deck(Vec<Card>);

impl Deck {
    /// Creates new (not shuffled) deck.
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(36);
        for suit in CardSuit::iter() {
            for rank in CardRank::iter() {
                cards.push(Card { suit, rank });
            }
        }
        Self(cards)
    }

    /// Shuffles all cards in the deck.
    pub fn shuffle(&mut self) {
        self.0.shuffle(&mut thread_rng());
    }

    /// Takes card from the top of the deck.
    pub fn take(&mut self) -> Option<Card> {
        self.0.pop()
    }

    /// Returns `true` if deck is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug)]
struct Player {
    pub id: PlayerId,
    pub hand: Hand,
}

#[derive(Debug)]
struct Hand(Vec<Card>);

impl Hand {
    /// Creates new empty hand.
    pub fn new() -> Self {
        Hand(Vec::with_capacity(6))
    }

    /// Adds card to the hand.
    pub fn add(&mut self, card: Card) {
        self.0.push(card);
    }

    /// Removes card from the hand.
    ///
    /// Returns `true` if card was in the hand.
    pub fn remove(&mut self, card: Card) -> bool {
        match self.0.iter().position(|card_| *card_ == card) {
            Some(index) => {
                self.0.remove(index);
                true
            }
            None => false,
        }
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

/// Table is the main location where game is played.
#[derive(Debug)]
struct Table(Vec<(Card, Option<Card>)>);

impl Table {
    pub fn new() -> Self {
        Table(Vec::with_capacity(6))
    }

    pub fn count(&self) -> usize {
        debug_assert!(self.0.len() <= 6, "table shouldn't have more than 6 cards");
        self.0.len()
    }

    /// Places provided card as attacking.
    ///
    /// Returns `true` if card placed successfully.
    // TODO: allow attacking by multiple cards at once.
    pub fn attack(&mut self, card: Card) -> bool {
        if self.count() >= 6 || !self.all_attacks_answered() {
            return false;
        }
        for (attacking, defending) in self.0.iter() {
            if attacking.rank == card.rank {
                self.0.push((card, None));
                return true;
            }
            if defending.is_some_and(|def| def.rank == card.rank) {
                self.0.push((card, None));
                return true;
            }
        }
        false
    }

    /// Places provided card as defending.
    ///
    /// Returns `true` if card placed successfully.
    // TODO: allow specifying which card to defend against.
    pub fn defend(&mut self, card: Card, trump: CardSuit) -> bool {
        if self.all_attacks_answered() {
            return false;
        }
        let Some((attacking, defending @ None)) = self.0.last_mut() else {
            return false;
        };
        if !card.can_beat(*attacking, trump) {
            return false;
        }
        *defending = Some(card);
        true
    }

    /// Takes all cards from the table. Table is cleared.
    ///
    /// Returns all cards that were on the table.
    /// Returns `None` if taking cards is against rules.
    pub fn take(&mut self) -> Option<Vec<Card>> {
        if self.all_attacks_answered() {
            return None;
        }
        let cards = std::mem::replace(&mut self.0, Vec::with_capacity(6));
        let mut result = Vec::with_capacity(cards.len() * 2);
        for (attacking, defending) in cards {
            result.push(attacking);
            if let Some(defending) = defending {
                result.push(defending);
            }
        }
        Some(result)
    }

    /// Removes all cards from the table.
    ///
    /// Returns `true` if retreat was successful.
    pub fn retreat(&mut self) -> bool {
        if !self.all_attacks_answered() {
            return false;
        }
        self.0.clear();
        true
    }

    /// Returns `true` if all attacks on the table are answered.
    fn all_attacks_answered(&self) -> bool {
        self.0.iter().all(|(_, defending)| defending.is_some())
    }
}
