use crate::prelude::*;
use bracket_random::prelude::*;
use legion::prelude::*;
use nox_raws::*;

pub fn spawn_clothing_from_raws_worn(
    ecs: &mut World,
    tag: &str,
    wearer: usize,
    rng: &mut RandomNumberGenerator,
) -> Vec<(usize, (f32, f32, f32))> {
    let mut result = Vec::new();
    println!("Spawning: {}", tag);

    let cd = RAWS.read().clothing.clothing_by_tag(tag);
    if let Some(cd) = cd {
        let index = RAWS.read().vox.get_model_idx(&cd.model);

        let color = get_color(rng.random_slice_entry(&cd.colors));

        ecs.insert(
            (Item {},),
            vec![(
                Identity::new(),
                Position::worn(wearer),
                Name { name: cd.name },
                Description {
                    desc: cd.description,
                },
                crate::VoxelModel {
                    index,
                    rotation_radians: 0.0,
                },
                Tint { color },
            )],
        );

        result.push((index, color));
    } else {
        println!("Clothing item not found: {}", tag);
    }

    result
}

fn get_color(c: Option<&String>) -> (f32, f32, f32) {
    if let Some(c) = c {
        match c.as_str() {
            "white" => (1.0, 1.0, 1.0),
            "black" => (0.1, 0.1, 0.1),
            "blue" => (0.1, 0.1, 1.0),
            "grey" => (0.5, 0.5, 0.5),
            "red" => (1.0, 0.1, 0.1),
            "green" => (0.1, 1.0, 0.1),
            "yellow" => (1.0, 1.0, 0.1),
            "navy" => (0.1, 0.1, 0.7),
            "khaki" => (0.7, 0.7, 0.1),
            "brown" => (0.5, 0.3, 0.3),
            _ => {
                println!("No color match for {}", c);
                (1.0, 1.0, 1.0)
            }
        }
    } else {
        (1.0, 1.0, 1.0)
    }
}
