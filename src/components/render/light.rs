use crate::components::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "c149854b-f14e-4e97-9a2c-f1cbc7f68faa"]
pub struct Light {
    pub color: (f32, f32, f32),
    pub radius: usize
}
