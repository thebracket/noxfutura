use crate::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[uuid = "5c7ec587-0072-4e6d-8af0-64ef4c54f809"]
pub struct VoxLayer {
    pub model: usize,
    pub tint: (f32, f32, f32),
}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[uuid = "fc422f00-9a1b-4d48-9d77-f620d880b439"]
pub struct CompositeRender {
    pub layers: Vec<VoxLayer>,
}
