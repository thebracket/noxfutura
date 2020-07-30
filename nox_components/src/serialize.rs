use legion::*;
use serde::de::DeserializeSeed;
use crate::*;

fn registry() -> Registry<String> {
    let mut registry = Registry::<String>::new();

    registry.register::<Position>("Position".to_string());
    registry.register::<CameraOptions>("CameraOptions".to_string());
    registry.register::<Calendar>("Calendar".to_string());
    registry.register::<VoxelModel>("VoxelModel".to_string());
    registry.register::<Name>("Name".to_string());
    registry.register::<Description>("Description".to_string());
    registry.register::<Tint>("Tint".to_string());
    registry.register::<Species>("Species".to_string());
    registry.register::<CompositeRender>("CompositeRender".to_string());
    registry.register::<Tagline>("Tagline".to_string());
    registry.register::<Attributes>("Attributes".to_string());
    registry.register::<Light>("Light".to_string());
    registry.register::<FieldOfView>("FieldOfView".to_string());
    registry.register::<Initiative>("Initiative".to_string());
    registry.register::<MyTurn>("MyTurn".to_string());
    registry.register::<Storage>("Storage".to_string());
    registry.register::<WorkSchedule>("WorkSchedule".to_string());
    registry.register::<Cordex>("Cordex".to_string());
    registry.register::<Building>("Building".to_string());
    registry.register::<Item>("Item".to_string());
    registry.register::<Sentient>("Sentient".to_string());
    registry.register::<Vegetation>("Vegetation".to_string());
    registry.register::<Tree>("Tree".to_string());
    registry.register::<IdentityTag>("IdentityTag".to_string());
    registry.register::<Terrain>("Terrain".to_string());
    registry.register::<Tag>("Tag".to_string());

    registry
}

pub fn serialize_world(world: &World) -> String {
    ron::to_string(&world.as_serializable(component::<IdentityTag>(), &registry())).expect("Unable to serialize")
}

pub fn deserialize_world(raw: String) -> World {
    let universe = Universe::new();
    //ron::from_str(&raw).expect("blah")
    //registry().as_deserialize(&universe).deserialize(&raw).expect("Boo")

    let reg = registry();
    let de = reg.as_deserialize(&universe);
    let mut ronnie = ron::Deserializer::from_str(&raw).unwrap();
    de.deserialize(&mut ronnie).unwrap()
}
