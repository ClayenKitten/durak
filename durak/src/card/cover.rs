//! Systems that control cover of cards.

use bevy::prelude::*;
use durak_lib::game::card::Card;

use super::CardData;

pub struct CardCoverPlugin;

impl Plugin for CardCoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (cover_cards, uncover_cards));
    }
}

/// Marker component for card that is covered.
#[derive(Component, Debug, Clone, Copy)]
pub struct Covered;

/// Updates texture for every newly covered card.
fn cover_cards(mut query: Query<&mut TextureAtlasSprite, Added<Covered>>) {
    for mut texture in query.iter_mut() {
        texture.index = CardData::BACK_SPRITE_ID;
    }
}

/// Updates texture for every newly uncovered card.
fn uncover_cards(
    mut query: Query<(&mut TextureAtlasSprite, &Card)>,
    mut removed: RemovedComponents<Covered>,
) {
    for entity in &mut removed {
        if let Ok((mut texture, &card)) = query.get_mut(entity) {
            texture.index = CardData::sprite_atlas_id(card);
        }
    }
}
