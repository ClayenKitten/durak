use bevy::prelude::*;
use strum::EnumIter;

use crate::{collider::cursor_system, GameScreen};

use self::events::CardClicked;

/// Plugin that handles cards logic.
pub struct CardInteractionPlugin;

impl Plugin for CardInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::CardClicked>()
            .add_event::<events::CardHoverStarted>()
            .add_event::<events::CardHoverEnded>()
            .add_plugins(movement::CardMovementPlugin)
            .add_systems(
                Update,
                (cursor_system, card_click, (cover_cards, uncover_cards))
                    .chain()
                    .run_if(in_state(GameScreen::Round)),
            );
    }
}

/// Handles clicks on cards.
fn card_click(
    mut commands: Commands,
    mut event_reader: EventReader<CardClicked>,
    cards: Query<Option<&Covered>, (With<CardRank>, With<CardSuit>)>,
) {
    for CardClicked(entity) in event_reader.iter() {
        match cards.get(*entity) {
            Ok(Some(_)) => {
                commands.entity(*entity).remove::<Covered>();
            }
            Ok(None) => {
                commands.entity(*entity).insert(Covered);
            }
            Err(_) => continue,
        }
    }
}

/// Updates texture for every newly covered card.
fn cover_cards(mut query: Query<&mut TextureAtlasSprite, Added<Covered>>) {
    for mut texture in query.iter_mut() {
        texture.index = Card::BACK_SPRITE_ID;
    }
}

/// Updates texture for every newly uncovered card.
fn uncover_cards(
    mut query: Query<(&mut TextureAtlasSprite, &CardRank, &CardSuit)>,
    mut removed: RemovedComponents<Covered>,
) {
    for entity in &mut removed {
        if let Ok((mut texture, &rank, &suit)) = query.get_mut(entity) {
            texture.index = Card::sprite_atlas_id(suit, rank);
        }
    }
}

/// Set of static data associated with cards.
pub struct Card;

impl Card {
    pub const PIXEL_WIDTH: f32 = 42.;
    pub const PIXEL_HEIGHT: f32 = 60.;

    pub const SCALE: f32 = 3.;
    pub const WIDTH: f32 = Self::PIXEL_WIDTH * Self::SCALE;
    pub const HEIGHT: f32 = Self::PIXEL_HEIGHT * Self::SCALE;

    /// Sprite id for the back side of the card.
    pub const BACK_SPRITE_ID: usize = 27;

    pub fn sprite_atlas_id(suit: CardSuit, rank: CardRank) -> usize {
        let row = match suit {
            CardSuit::Heart => 0,
            CardSuit::Diamond => 1,
            CardSuit::Clover => 2,
            CardSuit::Pike => 3,
        };
        let column = match rank {
            CardRank::Ace => 0,
            CardRank::Six => 5,
            CardRank::Seven => 6,
            CardRank::Eight => 7,
            CardRank::Nine => 8,
            CardRank::Ten => 9,
            CardRank::Jack => 10,
            CardRank::Queen => 11,
            CardRank::King => 12,
        };
        row * 14 + column
    }
}

#[derive(Component, EnumIter, Debug, PartialEq, Eq, Clone, Copy)]
pub enum CardSuit {
    Clover,
    Diamond,
    Heart,
    Pike,
}

#[derive(Component, EnumIter, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardRank {
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

/// Marker component for card that is covered.
#[derive(Component, Debug, Clone, Copy)]
pub struct Covered;

pub mod movement {
    use bevy::prelude::*;

    use crate::{card::Card, collider::Collider, GameScreen, Hand, Player};

    use super::{CardRank, CardSuit};

    /// Plugin that update location of cards.
    pub struct CardMovementPlugin;

    impl Plugin for CardMovementPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(
                Update,
                (move_to_hand, move_to_table, move_to_discard).run_if(in_state(GameScreen::Round)),
            );
        }
    }

    pub fn move_to_hand(
        mut commands: Commands,
        mut cards: Query<&mut Transform, (With<CardRank>, With<CardSuit>)>,
        hands: Query<(&Player, &Hand), Changed<Hand>>,
        camera: Query<&OrthographicProjection>,
    ) {
        const HORIZONTAL_GAP: f32 = 10.;

        for (player, hand) in hands.iter() {
            if hand.0.is_empty() {
                continue;
            }

            let area = camera.single().area;
            let y = match player.is_controlled {
                true => area.min.y + Card::HEIGHT / 2. - Card::HEIGHT / 3.,
                false => area.max.y - Card::HEIGHT / 2. + Card::HEIGHT / 3.,
            };
            let max_offset = {
                let number_of_cards = (hand.0.len() - 1) as f32;
                number_of_cards * Card::WIDTH + number_of_cards * HORIZONTAL_GAP
            };
            for (number, entity) in hand.0.iter().enumerate() {
                let x = {
                    let number = number as f32;
                    let offset = number * Card::WIDTH + number * HORIZONTAL_GAP;
                    offset - max_offset / 2.
                };
                let collider = Collider(
                    Rect::from_center_size(
                        Vec2 { x, y },
                        Vec2 { x: Card::WIDTH, y: Card::HEIGHT }
                    )
                );
                let mut card_transform = cards.get_mut(*entity)
                    .expect("card should exist");
                card_transform.translation = Vec3::new(x, y, 0.0);
                commands.entity(*entity).insert(collider);
            }
        }
    }

    fn move_to_table() {}

    fn move_to_discard() {}
}

pub mod events {
    use bevy::prelude::*;

    #[derive(Event, Clone, Copy, PartialEq, Eq)]
    pub struct CardClicked(pub Entity);

    #[derive(Event, Clone, Copy, PartialEq, Eq)]
    pub struct CardHoverStarted(pub Entity);

    #[derive(Event, Clone, Copy, PartialEq, Eq)]
    pub struct CardHoverEnded(pub Entity);
}
