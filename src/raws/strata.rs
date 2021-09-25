use crate::raws::{MaterialLayer, RAWS};
use crate::simulation::terrain::PLANET_STORE;

/// Call this after the raw files have loaded.
pub fn verify_strata() {
    PLANET_STORE.write().strata = Some(StrataMaterials::read());
}

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
        //.for_each(|m| println!("{:#?}", m));
        .map(|(i, _)| i)
        .collect()
    //Vec::new()
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
