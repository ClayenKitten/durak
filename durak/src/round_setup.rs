//! Contains all data and logic used to setup new round.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use durak_lib::game::card::{CardRank, CardSuit};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::{
    card::{Card, Covered},
    round::{Table, Trump},
    Deck, GameScreen, Hand, Player,
};

pub struct RoundSetupPlugin;

impl Plugin for RoundSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AdvanceSetupPhase>()
            .add_state::<SetupPhase>()
            .add_systems(
                Update,
                (
                    (
                        (spawn_deck, spawn_table).run_if(in_state(SetupPhase::CreateDeck)),
                        shuffle_deck.run_if(in_state(SetupPhase::ShuffleDeck)),
                        pick_trump.run_if(in_state(SetupPhase::PickTrump)),
                        deal_cards.run_if(in_state(SetupPhase::DealCards)),
                        uncover_player_cards.run_if(in_state(SetupPhase::UncoverPlayerCards)),
                    ),
                    advance_setup_phase,
                )
                    .chain()
                    .run_if(in_state(GameScreen::RoundSetup)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum SetupPhase {
    #[default]
    CreateDeck,
    ShuffleDeck,
    DealCards,
    PickTrump,
    UncoverPlayerCards,
}

#[derive(Event)]
struct AdvanceSetupPhase;

fn advance_setup_phase(
    event_reader: EventReader<AdvanceSetupPhase>,
    mut next_game_phase: ResMut<NextState<GameScreen>>,
    current_setup_phase: Res<State<SetupPhase>>,
    mut next_setup_phase: ResMut<NextState<SetupPhase>>,
) {
    use SetupPhase::*;
    if !event_reader.is_empty() {
        let next_setup = match current_setup_phase.get() {
            CreateDeck => ShuffleDeck,
            ShuffleDeck => DealCards,
            DealCards => PickTrump,
            PickTrump => UncoverPlayerCards,
            UncoverPlayerCards => {
                next_game_phase.0 = Some(GameScreen::Round);
                SetupPhase::default()
            }
        };
        next_setup_phase.0 = Some(next_setup);
    }
}

fn spawn_deck(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    camera: Query<&OrthographicProjection>,
    mut advance: EventWriter<AdvanceSetupPhase>,
) {
    let texture_handle = asset_server.load("cards.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(Card::PIXEL_WIDTH, Card::PIXEL_HEIGHT),
        14,
        4,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let deck_position = Vec3 {
        x: camera.single().area.min.x + Card::WIDTH / 2. + 16.,
        z: 1.,
        ..default()
    };

    let mut entities = Vec::with_capacity(36);
    for suit in CardSuit::iter() {
        for rank in CardRank::iter() {
            let entity = commands
                .spawn((
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
                ))
                .id();
            entities.push(entity);
        }
    }
    commands.spawn(Deck(entities));
    advance.send(AdvanceSetupPhase);
}

fn spawn_table(mut commands: Commands, mut advance: EventWriter<AdvanceSetupPhase>) {
    commands.spawn(Table::default());
    advance.send(AdvanceSetupPhase);
}

fn shuffle_deck(
    mut deck: Query<&mut Deck, Added<Deck>>,
    mut advance: EventWriter<AdvanceSetupPhase>,
) {
    if deck.is_empty() {
        return;
    }
    let deck = &mut deck.single_mut().0;
    deck.shuffle(&mut rand::thread_rng());
    advance.send(AdvanceSetupPhase);
}

fn pick_trump(
    mut commands: Commands,
    mut deck: Query<&mut Deck, Added<Deck>>,
    mut card: Query<(&mut Transform, &CardSuit)>,
    mut advance: EventWriter<AdvanceSetupPhase>,
) {
    if deck.is_empty() {
        return;
    }
    let deck = &mut deck.single_mut().0;
    let trump_card = *deck
        .first()
        .expect("deck shouldn't be empty at the moment of choosing trump card");
    let (mut trump_transform, trump) = card
        .get_mut(trump_card)
        .expect("trump card should have transform and suit");
    trump_transform.rotate_z(FRAC_PI_2);
    trump_transform.translation.x += (Card::HEIGHT - Card::WIDTH) / 2.;
    commands.entity(trump_card).remove::<Covered>();
    commands.spawn(Trump(*trump));
    advance.send(AdvanceSetupPhase);
}

/// Give cards to players at the beginning of the round.
fn deal_cards(
    mut hands: Query<&mut Hand>,
    mut deck: Query<&mut Deck, Added<Deck>>,
    mut advance: EventWriter<AdvanceSetupPhase>,
) {
    if deck.is_empty() {
        return;
    }
    let deck = &mut deck.single_mut().0;
    let mut hands: Vec<Mut<Hand>> = hands.iter_mut().collect();
    for _ in 0..6 {
        for hand in hands.iter_mut() {
            let card = deck.pop().expect("deck shouldn't empty during dealing");
            hand.add(card);
        }
    }
    advance.send(AdvanceSetupPhase);
}

fn uncover_player_cards(
    mut commands: Commands,
    player: Query<(&Player, &Hand)>,
    mut advance: EventWriter<AdvanceSetupPhase>,
) {
    for (player, hand) in player.iter() {
        if !player.is_controlled {
            continue;
        }
        for entity in hand.0.iter() {
            commands.entity(*entity).remove::<Covered>();
        }
    }
    advance.send(AdvanceSetupPhase);
}
