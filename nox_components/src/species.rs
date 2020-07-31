use super::prelude::*;

// TODO: Be more inclusive

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum GenderIdentity {
    Male,
    Female,
    Neutral,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Sexuality {
    Heterosexual,
    Pansexual,
    Homosexual,
    ASexual,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum HairStyle {
    Bald,
    Balding,
    Mohawk,
    ShortHair,
    LongHair,
    Pigtails,
    Triangle,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Species {
    pub gender: Gender,
    pub gender_identity: GenderIdentity,
    pub sexuality: Sexuality,
    pub height_cm: f32,
    pub weight_kg: f32,
    pub bearded: bool,
    pub skin_color: (f32, f32, f32),
    pub hair_color: (f32, f32, f32),
    pub hair_style: HairStyle,
}
