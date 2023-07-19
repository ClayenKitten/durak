//! Contains all data and logic used to setup new round.

use bevy::prelude::*;
use durak_lib::game::{card::CardSuit, deck::Deck, table::Table};

use crate::{
    card::{cover::Covered, CardData, CardMapping},
    GameScreen, Hand, Player,
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
    if !event_reader.is_empty() {
        let next_setup = match current_setup_phase.get() {
            SetupPhase::CreateDeck => SetupPhase::UncoverPlayerCards,
            SetupPhase::UncoverPlayerCards => {
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
        Vec2::new(CardData::PIXEL_WIDTH, CardData::PIXEL_HEIGHT),
        14,
        4,
        None,
        None,
    );
    let texture_atlas = texture_atlases.add(texture_atlas);

    let deck_position = Vec3 {
        x: camera.single().area.min.x + CardData::WIDTH / 2. + 16.,
        z: 1.,
        ..default()
    };

    commands.spawn((
        Deck::new(),
        SpriteSheetBundle {
            transform: Transform::from_translation(deck_position),
            texture_atlas,
            sprite: TextureAtlasSprite::new(CardData::BACK_SPRITE_ID),
            ..default()
        },
    ));
    advance.send(AdvanceSetupPhase);
}

fn spawn_table(mut commands: Commands, mut advance: EventWriter<AdvanceSetupPhase>) {
    commands.spawn(Table::default());
    advance.send(AdvanceSetupPhase);
}

fn uncover_player_cards(
    mut commands: Commands,
    player: Query<(&Player, &Hand)>,
    card_mapping: Res<CardMapping>,
    mut advance: EventWriter<AdvanceSetupPhase>,
) {
    for (player, hand) in player.iter() {
        if !player.is_controlled {
            continue;
        }
        for card in hand.iter() {
            let entity = card_mapping.get(card);
            commands.entity(entity).remove::<Covered>();
        }
    }
    advance.send(AdvanceSetupPhase);
}

/// Trump suit for a round.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trump(pub CardSuit);
