//! Generation of default name.

use rand::seq::SliceRandom;

/// Returns random player name.
///
/// Name is formed in `adjective` + `animal` form, e.g. "Laid-back Rabbit".
pub fn generate_default_name() -> String {
    let mut rng = rand::thread_rng();
    let adjective = ADJECTIVES.choose(&mut rng).unwrap();
    let animal = ANIMALS.choose(&mut rng).unwrap();
    format!("{adjective} {animal}")
}

const ADJECTIVES: &[&str] = &[
    "Happy",
    "Creative",
    "Determined",
    "Fearless",
    "Energetic",
    "Gorgeous",
    "Thoughtful",
    "Optimistic",
    "Laid-back",
    "Organized",
    "Magnificent",
];

const ANIMALS: &[&str] = &[
    "Cat", "Dog", "Giraffe", "Horse", "Rabbit", "Capybara", "Raccoon", "Dolphin", "Koala", "Camel",
    "Frog", "Owl", "Hedgehog", "Crab",
];
