use crate::*;
use legion::*;

// I didn't like repeating myself in boilerplate, so this macro
// turns a type list into a bunch of component names.
macro_rules! register_component_types {
    ($registry:expr, $( $type:ty),*) => {
        $(
            $registry.register::<$type>(stringify!($type).to_string());
        )*
    };
}

fn registry() -> Registry<String> {
    let mut registry = Registry::default();

    register_component_types!(
        registry,
        Attributes,
        Initiative,
        Material,
        Storage,
        Tool,
        CameraOptions,
        VoxLayer,
        CompositeRender,
        Light,
        Tint,
        VoxelModel,
        Position,
        Cordex,
        Building,
        Item,
        Sentient,
        Settler,
        Vegetation,
        Tree,
        Terrain,
        Tag,
        Calendar,
        MyTurn,
        Species,
        Name,
        Description,
        Tagline,
        FieldOfView,
        WorkSchedule,
        IdentityTag,
        ObjModel
    );

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
