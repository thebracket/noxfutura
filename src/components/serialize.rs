use crate::components::*;
use legion::*;

fn registry() -> Registry<String> {
    let mut registry = Registry::<String>::default();

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
    registry.register::<Material>("Material".to_string());

    registry
}

pub fn serialize_world(world: &World) -> String {
    ron::to_string(&world.as_serializable(component::<IdentityTag>(), &registry())).unwrap()
}

pub fn deserialize_world(raw: String) -> World {
    use serde::de::DeserializeSeed;
    let reg = registry();
    let de = reg.as_deserialize();
    let mut ronnie = ron::Deserializer::from_str(&raw).unwrap();
    let world = de.deserialize(&mut ronnie).unwrap();
    identity::rebuild_identity(&world);
    world
}
