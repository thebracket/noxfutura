use super::super::super::RNG;
use legion::*;
use nox_components::*;

pub(crate) fn skill_check(ecs: &World, settler_id: usize, skill: Skill, difficulty: i32) -> i32 {
    let (skill_value, attr_bonus) = if let Some((skill, attr)) =
        <(&IdentityTag, &Skills, &Attributes)>::query()
            .iter(ecs)
            .filter(|(id, _skills, _attrib)| id.0 == settler_id)
            .map(|(_id, skills, attrib)| (skills.get_skill(skill), attribute_bonus(skill, attrib)))
            .nth(0)
    {
        (skill, attr)
    } else {
        (0, 0)
    };

    let die_roll = RNG.lock().roll_dice(1, 20);
    let modified_roll = die_roll + attr_bonus + skill_value;
    modified_roll - difficulty
}

pub(crate) fn attribute_bonus(skill: Skill, attribute: &Attributes) -> i32 {
    match skill {
        Skill::Lumberjack => raw_attribute_bonus(attribute.str),
        Skill::Mining => raw_attribute_bonus(attribute.str),
    }
}

fn raw_attribute_bonus(n: i32) -> i32 {
    (n - 10) / 2
}
