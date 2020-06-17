mod camera;
pub use camera::*;
mod position;
pub use position::*;
mod tags;
pub use tags::*;
mod world_position;
pub use world_position::*;
mod calendar;
pub use calendar::*;
mod dimensions;
pub use dimensions::*;
mod voxel_model;
pub use voxel_model::*;
mod name;
pub use name::*;
mod description;
pub use description::*;
mod tint;
pub use tint::*;
mod species;
pub use species::*;
mod composite_render;
pub use composite_render::*;

pub mod spawner;

mod serialize;
pub use serialize::{deserialize_world, serialize_world};

pub(crate) mod prelude {
    pub use serde::{
        de::{self, DeserializeSeed, IgnoredAny, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };
    pub use std::{
        any::TypeId, cell::RefCell, collections::HashMap, marker::PhantomData, ptr::NonNull,
    };
    pub use type_uuid::TypeUuid;
}

use serialize::{ComponentRegistration, TagRegistration};

fn component_registration() -> (Vec<ComponentRegistration>, Vec<TagRegistration>) {
    let comp_registrations = vec![
        ComponentRegistration::of::<Position>(),
        ComponentRegistration::of::<CameraOptions>(),
        ComponentRegistration::of::<Calendar>(),
        ComponentRegistration::of::<Dimensions>(),
        ComponentRegistration::of::<VoxelModel>(),
        ComponentRegistration::of::<WorldPosition>(),
        ComponentRegistration::of::<Name>(),
        ComponentRegistration::of::<Description>(),
        ComponentRegistration::of::<Tint>(),
        ComponentRegistration::of::<Species>(),
        ComponentRegistration::of::<CompositeRender>(),
    ];
    let tag_registrations = vec![
        TagRegistration::of::<Cordex>(),
        TagRegistration::of::<Building>(),
        TagRegistration::of::<Item>(),
        TagRegistration::of::<Sentient>(),
    ];
    (comp_registrations, tag_registrations)
}