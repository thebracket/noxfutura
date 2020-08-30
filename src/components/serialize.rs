use crate::components::*;
use legion::*;

pub fn serialize_world(world: &World) -> String {
    //ron::to_string(&world.as_serializable(component::<IdentityTag>(), &registry()))
    //    .expect("Unable to serialize")
    "".to_string()
}

pub fn deserialize_world(raw: String) -> World {
    //let universe = Universe::new();
    //let reg = registry();
    //let de = reg.as_deserialize(&universe);
    //let mut ronnie = ron::Deserializer::from_str(&raw).unwrap();
    //let world = de.deserialize(&mut ronnie).unwrap();
    //identity::rebuild_identity(&world);
    World::default()
}
