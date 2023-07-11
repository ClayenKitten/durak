mod card;
mod collider;
mod round;

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use card::{Card, CardInteractionPlugin, CardRank, CardSuit, Covered};
use collider::Collider;
use rand::seq::SliceRandom;
use round::Trump;
use strum::IntoEnumIterator;

#[derive(Debug, Component)]
struct Player {
    pub name: String,
    pub is_controlled: bool,
}

/// Hand of the player containing all cards the player has.
#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
struct Hand(Vec<Entity>);

/// List of cards that are still in deck.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
struct Deck(Vec<Entity>);

/// Marker component for card that is discarded.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct Discarded;

// Our Bevy app's entry point
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Durak"),
                        resolution: (1280., 800.).into(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .add_state::<GameScreen>()
        .add_plugins(CardInteractionPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update,
            (
                spawn_deck,
                shuffle_deck,
                pick_trump,
                deal_cards,
            )
                .chain()
                .run_if(in_state(GameScreen::RoundSetup))
        )
        .add_systems(Update, display_hand.run_if(in_state(GameScreen::Round)))
        .add_systems(
            OnExit(GameScreen::Round),
            cleanup_round,
        )
        .run();
}

fn startup(
    mut commands: Commands,
    mut state: ResMut<NextState<GameScreen>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player { name: String::from("Me"), is_controlled: true },
        Hand::default(),
    ));
    commands.spawn((
        Player { name: String::from("Not me"), is_controlled: false },
        Hand::default(),
    ));
    state.0 = Some(GameScreen::RoundSetup);
}

fn spawn_deck(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    camera: Query<&OrthographicProjection>,
) {
    let texture_handle = asset_server.load("cards.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(Card::PIXEL_WIDTH, Card::PIXEL_HEIGHT), 14, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let deck_position = Vec3 {
        x: camera.single().area.min.x + Card::WIDTH / 2. + 16.,
        z: 1.,
        ..default()
    };

    let mut entities = Vec::with_capacity(36);
    for suit in CardSuit::iter() {
        for rank in CardRank::iter() {
            let entity = commands.spawn((
                suit,
                rank,
                Covered,
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(Card::BACK_SPRITE_ID),
                    transform: Transform {
                        translation: deck_position,
                        rotation: Quat::default(),
                        scale: Vec3::splat(Card::SCALE),
                    },
                    ..default()
                },
            )).id();
            entities.push(entity);
        }
    }
    commands.spawn(Deck(entities));
}

fn shuffle_deck(mut deck: Query<&mut Deck, Added<Deck>>) {
    if deck.is_empty() {
        return;
    }
    let deck = &mut deck.single_mut().0;
    deck.shuffle(&mut rand::thread_rng());
}

fn pick_trump(
    mut commands: Commands,
    mut deck: Query<&mut Deck, Added<Deck>>,
    mut card: Query<(&mut Transform, &CardSuit)>,
) {
    if deck.is_empty() {
        return;
    }
    let deck = &mut deck.single_mut().0;
    let trump_card = *deck.first()
        .expect("deck shouldn't be empty at the moment of choosing trump card");
    let (mut trump_transform, trump) = card.get_mut(trump_card)
        .expect("trump card should have transform and suit");
    trump_transform.rotate_z(FRAC_PI_2);
    trump_transform.translation.x += (Card::HEIGHT - Card::WIDTH) / 2.;
    commands.entity(trump_card)
        .remove::<Covered>();
    commands.spawn(Trump(*trump));
}

fn cleanup_round() {

}

/// Give cards to players at the beginning of the round.
fn deal_cards(
    mut hands: Query<&mut Hand>,
    mut deck: Query<&mut Deck, Added<Deck>>,
    mut state: ResMut<NextState<GameScreen>>,
) {
    if deck.is_empty() {
        return;
    }
    let deck = &mut deck.single_mut().0;
    let mut hands: Vec<Mut<Hand>> = hands.iter_mut().collect();
    for _ in 0..6 {
        for hand in hands.iter_mut() {
            let card = deck.pop()
                .expect("deck shouldn't empty during dealing");
            hand.0.push(card);
        }
    }
    state.0 = Some(GameScreen::Round);
}

fn display_hand(
    mut commands: Commands,
    mut cards: Query<&mut Transform, (With<CardRank>, With<CardSuit>)>,
    hands: Query<(&Player, &Hand), Changed<Hand>>,
    camera: Query<&OrthographicProjection>,
) {
    const HORIZONTAL_GAP: f32 = 10.;

    for (player, hand) in hands.iter() {
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
            if player.is_controlled {
                commands.entity(*entity).remove::<Covered>();
            }
        }
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameScreen {
    #[default]
    MainMenu,
    RoundSetup,
    Round,
}
