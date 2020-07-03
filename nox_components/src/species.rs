use super::prelude::*;

// TODO: Be more inclusive

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "0e9e3f7b-0769-4b4e-b332-d94861f6d20c"]
pub enum Gender {
    Male,
    Female,
}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "0e9e3f7b-0769-4b4e-b332-d94861f6d20c"]
pub enum GenderIdentity {
    Male,
    Female,
    Neutral,
}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "9494aa2b-56b5-4573-a5e1-68bfde781bd5"]
pub enum Sexuality {
    Heterosexual,
    Pansexual,
    Homosexual,
    ASexual,
}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "71e76ad4-17e6-4dc8-a129-568a6bdd14e8"]
pub enum HairStyle {
    Bald,
    Balding,
    Mohawk,
    ShortHair,
    LongHair,
    Pigtails,
    Triangle,
}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "a5a46181-6f8c-49a9-869d-f1367da09564"]
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
