mod card;
mod collider;
mod round;

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use card::{Card, CardInteractionPlugin, CardRank, CardSuit, Covered};
use rand::seq::SliceRandom;
use round::Trump;
use strum::IntoEnumIterator;

#[derive(Debug, Component)]
pub struct Player {
    pub _name: String,
    pub is_controlled: bool,
}

/// Hand of the player containing all cards the player has.
#[derive(Component, Debug, Clone, PartialEq, Eq, Default)]
pub struct Hand(Vec<Entity>);

/// List of cards that are still in deck.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Deck(Vec<Entity>);

/// Marker component for card that is discarded.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Discarded;

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
                }),
        )
        .add_state::<GameScreen>()
        .add_state::<SetupPhase>()
        .add_event::<AdvanceSetupPhase>()
        .add_plugins(CardInteractionPlugin)
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                (
                    spawn_deck.run_if(in_state(SetupPhase::CreateDeck)),
                    shuffle_deck.run_if(in_state(SetupPhase::ShuffleDeck)),
                    pick_trump.run_if(in_state(SetupPhase::PickTrump)),
                    deal_cards.run_if(in_state(SetupPhase::DealCards)),
                    uncover_player_cards.run_if(in_state(SetupPhase::UncoverPlayerCards)),
                ),
                advance_setup_phase,
            )
                .chain()
                .run_if(in_state(GameScreen::RoundSetup)),
        )
        .run();
}

fn startup(
    mut commands: Commands,
    mut state: ResMut<NextState<GameScreen>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player { _name: String::from("Me"), is_controlled: true },
        Hand::default(),
    ));
    commands.spawn((
        Player { _name: String::from("Not me"), is_controlled: false },
        Hand::default(),
    ));
    state.0 = Some(GameScreen::RoundSetup);
}

fn spawn_deck(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    camera: Query<&OrthographicProjection>,
    mut advance: EventWriter<AdvanceSetupPhase>,
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
    let trump_card = *deck.first()
        .expect("deck shouldn't be empty at the moment of choosing trump card");
    let (mut trump_transform, trump) = card.get_mut(trump_card)
        .expect("trump card should have transform and suit");
    trump_transform.rotate_z(FRAC_PI_2);
    trump_transform.translation.x += (Card::HEIGHT - Card::WIDTH) / 2.;
    commands.entity(trump_card)
        .remove::<Covered>();
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
            let card = deck.pop()
                .expect("deck shouldn't empty during dealing");
            hand.0.push(card);
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

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameScreen {
    #[default]
    MainMenu,
    RoundSetup,
    Round,
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
pub struct AdvanceSetupPhase;

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
