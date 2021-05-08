use super::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawBundle {
    pub biomes: Option<Vec<BiomeType>>,
    pub buildings: Option<Vec<BuildingDef>>,
    pub clothing: Option<Vec<ClothingDef>>,
    pub items: Option<Vec<ItemDef>>,
    pub materials: Option<Vec<MaterialDef>>,
    pub plants: Option<Vec<PlantDef>>,
    pub professions: Option<Vec<ProfessionDef>>,
    pub reactions: Option<Vec<ReactionDef>>,
    pub species: Option<Vec<SpeciesDef>>,
    pub vox: Option<Vec<VoxelModel>>,
    pub models: Option<Vec<WavefrontObj>>,
    pub colors: Option<Vec<MappedColor>>,
}

impl RawBundle {
    pub fn load(filename: &str) -> Self {
        let f = File::open(filename).expect("Failed opening file");
        let bundle: RawBundle = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load bundle list: {}: {:?}", filename, e);
                std::process::exit(1);
            }
        };
        bundle
    }

    pub fn merge(&self, raws: &mut crate::Raws) {
        if let Some(biomes) = &self.biomes {
            raws.biomes.areas.extend_from_slice(&biomes);
        }
        if let Some(buildings) = &self.buildings {
            raws.buildings.buildings.extend_from_slice(&buildings);
        }
        if let Some(clothing) = &self.clothing {
            raws.clothing.clothing.extend_from_slice(&clothing);
        }
        if let Some(items) = &self.items {
            raws.items.items.extend_from_slice(&items);
        }
        if let Some(materials) = &self.materials {
            raws.materials.materials.extend_from_slice(&materials);
        }
        if let Some(plants) = &self.plants {
            raws.plants.plants.extend_from_slice(&plants);
        }
        if let Some(professions) = &self.professions {
            raws.professions.professions.extend_from_slice(&professions);
        }
        if let Some(reactions) = &self.reactions {
            raws.reactions.reactions.extend_from_slice(&reactions);
        }
        if let Some(species) = &self.species {
            raws.species.species.extend_from_slice(&species);
        }
        if let Some(vox) = &self.vox {
            raws.vox.vox.extend_from_slice(&vox);
        }
        if let Some(models) = &self.models {
            raws.obj_models.models.extend_from_slice(&models);
        }
        if let Some(colors) = &self.colors {
            raws.obj_models.colors.extend_from_slice(&colors);
        }
    }
}
