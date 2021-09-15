use crate::raws::{MaterialLayer, RAWS};

fn get_strata_indices(st: MaterialLayer) -> Vec<usize> {
    let mlock = RAWS.read();
    mlock
        .materials
        .materials
        .iter()
        .enumerate()
        .filter(|(_, m)| m.layer == st)
        .map(|(i, _)| i)
        .collect()
}

fn get_soil_indices() -> Vec<usize> {
    let mlock = RAWS.read();
    mlock
        .materials
        .materials
        .iter()
        .enumerate()
        .filter(|(_, m)| match m.layer {
            MaterialLayer::Soil { .. } => true,
            _ => false,
        })
        .map(|(i, _)| i)
        .collect()
}

pub struct StrataMaterials {
    pub soils: Vec<usize>,
    pub sand: Vec<usize>,
    pub sedimentary: Vec<usize>,
    pub igneous: Vec<usize>,
}

impl StrataMaterials {
    pub fn read() -> Self {
        Self {
            soils: get_soil_indices(),
            sand: get_strata_indices(MaterialLayer::Sand),
            sedimentary: get_strata_indices(MaterialLayer::Sedimentary),
            igneous: get_strata_indices(MaterialLayer::Igneous),
        }
    }
}
