use std::collections::HashMap;

use durak_lib::{
    game::{
        card::{Card, CardRank, CardSuit},
        deck::Deck,
        hand::Hand,
        player::Opponent,
        table::Table,
    },
    identifiers::PlayerId,
    status::{round::RoundStatus, LobbyPlayerData},
};

/// State of started game.
#[derive(Debug)]
pub struct RoundState {
    pub deck: Deck,
    pub trump: Card,
    pub table: Table,
    pub attacker: PlayerId,
    pub defender: PlayerId,
    pub players: Vec<LobbyPlayerData>,
    pub hands: HashMap<PlayerId, Hand>,
}

/// New round creation.
impl RoundState {
    pub fn new(players: Vec<LobbyPlayerData>) -> Self {
        let mut deck = Self::create_deck();
        let hands = Self::create_hands(&mut deck, players.iter().map(|p| p.id).collect());
        let trump = Self::pick_trump(&mut deck);
        let attacker = Self::define_first_attacker(&hands, trump.suit);
        // TODO: allow more than two players
        let defender = if attacker == PlayerId::new(0) {
            PlayerId::new(1)
        } else {
            PlayerId::new(0)
        };

        Self {
            deck,
            trump,
            table: Table::new(),
            attacker,
            defender,
            hands,
            players,
        }
    }

    /// Creates new shuffled deck.
    fn create_deck() -> Deck {
        let mut deck = Deck::new();
        deck.shuffle();
        deck
    }

    /// Creates new hands and deals cards.
    fn create_hands(deck: &mut Deck, players: Vec<PlayerId>) -> HashMap<PlayerId, Hand> {
        players
            .into_iter()
            .map(|p| {
                let mut hand = Hand::new();
                for _ in 0..6 {
                    match deck.take() {
                        Some(card) => hand.add(card),
                        None => break,
                    }
                }
                (p, hand)
            })
            .collect()
    }

    /// Picks trump from the top of the deck and puts it under all other cards.
    ///
    /// Should be called after the first call to [deal_cards].
    fn pick_trump(deck: &mut Deck) -> Card {
        let trump = deck.take().unwrap();
        deck.insert_bottom(trump);
        trump
    }

    /// Defines first attacker for the round.
    ///
    /// The initial attacker is the player possessing the smallest trump card.
    // TODO: announce smallest trump holder to other players.
    fn define_first_attacker(hands: &HashMap<PlayerId, Hand>, trump: CardSuit) -> PlayerId {
        let mut min = (PlayerId::new(0), CardRank::Ace);
        for (player_id, hand) in hands.iter() {
            for card in hand.iter() {
                if card.suit == trump && card.rank <= min.1 {
                    min = (*player_id, card.rank);
                }
            }
        }
        min.0
    }
}

/// Player actions.
impl RoundState {
    /// Places card on the table.
    ///
    /// Returns `true` if played successfully.
    pub fn play_card(&mut self, player_id: PlayerId, card: Card) -> bool {
        let Some((&player_id, hand)) = self.hands.iter_mut().find(|(id, _)| **id == player_id) else {
            return false;
        };

        if self.attacker == player_id {
            if hand.contains(card) && self.table.attack(card) {
                hand.remove(card);
                true
            } else {
                false
            }
        } else if self.defender == player_id {
            if hand.contains(card) && self.table.defend(card, self.trump.suit) {
                hand.remove(card);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Attacker decided to stop an attack.
    pub fn retreat(&mut self, player_id: PlayerId) -> bool {
        if self.attacker != player_id {
            return false;
        }
        if self.table.retreat() {
            self.swap_players();
            self.deal_cards();
            true
        } else {
            false
        }
    }

    /// Defender decided to take all cards from the table.
    pub fn take(&mut self, player_id: PlayerId) -> bool {
        let Some((&player_id, hand)) = self.hands.iter_mut().find(|(id, _)| **id == player_id) else {
            return false;
        };

        if self.defender != player_id {
            return false;
        }
        let Some(cards) = self.table.take() else {
            return false;
        };
        for card in cards {
            hand.add(card);
        }
        self.deal_cards();
        true
    }

    /// Generates status report for specific player.
    pub fn status(&self, player: PlayerId) -> Option<RoundStatus> {
        Some(RoundStatus {
            turn: self.turn(),
            attacker: self.attacker,
            defender: self.defender,
            table: self.table.clone(),
            deck_size: self.deck.count() as u8,
            opponents: self
                .players
                .iter()
                .filter(|p| p.id != player)
                .cloned()
                .map(|LobbyPlayerData { id, name }| Opponent {
                    id,
                    name,
                    cards_number: self.hands.get(&id).unwrap().count() as u8,
                })
                .collect(),
            hand: self.hands.get(&player).unwrap().clone(),
        })
    }
}

/// Round utility.
impl RoundState {
    /// Checks if any player won the game.
    ///
    /// Returns `None` if no one won yet. Otherwise, returns [PlayerId] of the winner.
    fn check_winner(&self) -> Option<PlayerId> {
        if !self.deck.is_empty() {
            return None;
        }
        for (player_id, hand) in self.hands.iter() {
            if hand.is_empty() {
                return Some(*player_id);
            }
        }
        None
    }

    /// Deals cards to players.
    ///
    /// Players holding fewer than 6 cards receive additional cards to bring their total back to 6.
    fn deal_cards(&mut self) {
        for player in self.iter_players(self.attacker) {
            let hand = self.hands.get_mut(&player).unwrap();
            let additional_cards_number = 6usize.saturating_sub(hand.count());
            for _ in 0..additional_cards_number {
                let Some(card) = self.deck.take() else {
                    break;
                };
                hand.add(card);
            }
        }
    }

    /// Returns mutable iterator of player ids starting with provided.
    fn iter_players(&mut self, start: PlayerId) -> impl Iterator<Item = PlayerId> {
        let start = self
            .hands
            .iter()
            .position(|(player_id, _)| *player_id == start)
            .unwrap();
        let count = self.hands.len();
        (start..count)
            .chain(0..start)
            .map(|i| PlayerId::new(i as u8))
    }

    /// Swaps attacker and defender.
    // TODO: allow more than two players.
    fn swap_players(&mut self) {
        std::mem::swap(&mut self.attacker, &mut self.defender)
    }

    /// Returns id of the player whose turn is it to play.
    pub fn turn(&self) -> PlayerId {
        if self.table.all_attacks_answered() {
            self.attacker
        } else {
            self.defender
        }
    }
}
