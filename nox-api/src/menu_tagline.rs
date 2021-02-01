use bracket_random::prelude::RandomNumberGenerator;

const NOUNS: &'static [&'static str] = &[
    "Stupidity",
    "Idiocy",
    "Dullness",
    "Foolishness",
    "Futility",
    "Naievity",
    "Senselessness",
    "Shortsightedness",
    "Triviality",
    "Brainlessness",
    "Inanity",
    "Insensitivity",
    "Indiscretion",
    "Mindlessness",
    "Moronism",
    "Myopia",
    "Obtuseness",
    "Obliviousness",
    "Unthinkingness",
];

const DEDICATION: &'static str =
    "To Kylah of the West and Jakie Monster -\nThe Bravest Little Warriors of Them All.";

fn get_descriptive_noun(rng: &mut RandomNumberGenerator) -> String {
    rng.random_slice_entry(&NOUNS)
        .unwrap()
        .to_string()
}

pub fn new_menu_tagline() -> String {
    let mut rng = RandomNumberGenerator::new();
    let mut tagline = match rng.roll_dice(1, 8) {
        1 => "Histories",
        2 => "Chronicles",
        3 => "Sagas",
        4 => "Annals",
        5 => "Narratives",
        6 => "Recitals",
        7 => "Tales",
        8 => "Stories",
        _ => "",
    }
    .into();

    let first_noun = get_descriptive_noun(&mut rng);
    let mut second_noun = get_descriptive_noun(&mut rng);
    while first_noun == second_noun {
        second_noun = get_descriptive_noun(&mut rng);
    }

    tagline = format!("{} of {} and {}", tagline, first_noun, second_noun).to_string();
    tagline
}

pub fn get_dedication() -> &'static str {
    &DEDICATION
}