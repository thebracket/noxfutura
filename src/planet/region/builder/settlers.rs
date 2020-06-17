use crate::components::*;
use bracket_geometry::prelude::*;
use bracket_random::prelude::*;
use legion::prelude::*;

pub fn spawn_settlers(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    crash_site: &Point,
    crash_z: usize,
) {
    let spawn_points = vec![
        (crash_site.x - 3, crash_site.y - 2, crash_z + 3),
        (crash_site.x - 2, crash_site.y - 2, crash_z + 3),
        (crash_site.x - 1, crash_site.y - 2, crash_z + 3),
        (crash_site.x, crash_site.y - 2, crash_z + 3),
        (crash_site.x + 1, crash_site.y - 2, crash_z + 3),
        (crash_site.x - 3, crash_site.y, crash_z + 3),
        (crash_site.x - 2, crash_site.y, crash_z + 3),
        (crash_site.x - 1, crash_site.y, crash_z + 3),
        (crash_site.x, crash_site.y, crash_z + 3),
        (crash_site.x + 1, crash_site.y, crash_z + 3),
    ];

    for spawn in spawn_points.iter() {
        spawn_settler(
            ecs,
            rng,
            spawn.0 as usize,
            spawn.1 as usize,
            spawn.2 as usize,
        );
    }
}

fn spawn_settler(ecs: &mut World, rng: &mut RandomNumberGenerator, x: usize, y: usize, z: usize) {
    let species_def = crate::raws::RAWS.read().species.species[0].clone();

    let gender = if rng.roll_dice(1, 20) < 11 {
        Gender::Male
    } else {
        Gender::Female
    };

    let gender_identity = if rng.roll_dice(1, 100) < 10 {
        match rng.roll_dice(1, 3) {
            1 => GenderIdentity::Female,
            2 => GenderIdentity::Neutral,
            _ => GenderIdentity::Male,
        }
    } else {
        if gender == Gender::Male {
            GenderIdentity::Male
        } else {
            GenderIdentity::Female
        }
    };

    let sexuality = match rng.roll_dice(1, 10) {
        1 => Sexuality::Homosexual,
        2 => Sexuality::Pansexual,
        3 => Sexuality::ASexual,
        _ => Sexuality::Heterosexual,
    };

    let height_cm = if gender == Gender::Male {
        147.0 + (rng.roll_dice(2, 10) as f32 * 2.5)
    } else {
        134.0 + (rng.roll_dice(2, 10) as f32 * 2.5)
    };

    let weight_kg = if gender == Gender::Male {
        54.0 + (rng.roll_dice(2, 8) as f32 * 0.45)
    } else {
        38.0 + (rng.roll_dice(2, 8) as f32 * 0.45)
    };

    let bearded = gender == Gender::Male && rng.roll_dice(1, 10) < 7;

    let skin_color_def = rng.random_slice_entry(&species_def.skin_colors).unwrap();
    let hair_color_def = rng.random_slice_entry(&species_def.hair_colors).unwrap();

    let hair_style = if gender_identity == GenderIdentity::Male {
        match rng.roll_dice(1, 5) {
            1 => HairStyle::Balding,
            2 => HairStyle::Mohawk,
            3 => HairStyle::ShortHair,
            4 => HairStyle::LongHair,
            _ => HairStyle::Bald,
        }
    } else {
        match rng.roll_dice(1, 4) {
            1 => HairStyle::ShortHair,
            2 => HairStyle::LongHair,
            3 => HairStyle::Pigtails,
            _ => HairStyle::Triangle,
        }
    };

    let species = Species {
        gender,
        gender_identity,
        sexuality,
        height_cm,
        weight_kg,
        bearded,
        skin_color: (
            skin_color_def.r as f32 / 255.0,
            skin_color_def.g as f32 / 255.0,
            skin_color_def.b as f32 / 255.0,
        ),
        hair_color: (
            hair_color_def.r as f32 / 255.0,
            hair_color_def.g as f32 / 255.0,
            hair_color_def.b as f32 / 255.0,
        ),
        hair_style,
    };

    let rlock = crate::raws::RAWS.read();
    let mut composite = CompositeRender { layers: Vec::new() };
    composite.layers.push(VoxLayer {
        model: rlock.vox.get_model_idx("person_base"),
        tint: species.skin_color,
    });
    if species.hair_style != HairStyle::Bald {
        composite.layers.push(VoxLayer {
            model: match species.hair_style {
                HairStyle::Balding => rlock.vox.get_model_idx("person_hair_balding"),
                HairStyle::Mohawk => rlock.vox.get_model_idx("person_hair_mohawk"),
                HairStyle::ShortHair => rlock.vox.get_model_idx("person_hair_short"),
                HairStyle::LongHair => rlock.vox.get_model_idx("person_hair_long"),
                HairStyle::Pigtails => rlock.vox.get_model_idx("person_hair_pigtails"),
                HairStyle::Triangle => rlock.vox.get_model_idx("person_hair_triangle"),
                HairStyle::Bald => 0,
            },
            tint: species.hair_color,
        });
    }

    let name = Name{ name: rlock.names.random_settler_name(rng, species.gender_identity) };

    let profession_def = rng.random_slice_entry(&rlock.professions.professions).unwrap();
    println!("{} ({})", name.name, profession_def.name);

    let attr = Attributes{
        str: rng.roll_dice(3, 6) + profession_def.modifiers.str.unwrap_or(0),
        dex: rng.roll_dice(3, 6) + profession_def.modifiers.dex.unwrap_or(0),
        con: rng.roll_dice(3, 6) + profession_def.modifiers.con.unwrap_or(0),
        int: rng.roll_dice(3, 6) + profession_def.modifiers.int.unwrap_or(0),
        wis: rng.roll_dice(3, 6) + profession_def.modifiers.wis.unwrap_or(0),
        cha: rng.roll_dice(3, 6) + profession_def.modifiers.cha.unwrap_or(0),
    };

    let entity = ecs.insert(
        (Building {},),
        vec![(
            Dimensions {
                width: 1,
                height: 1,
            },
            Position { x, y, z },
            species,
            composite,
            name,
            Tagline{name: profession_def.name.clone()},
            attr
        )],
    );

    // Spawning clothing and equipment goes here
}
