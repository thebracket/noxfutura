use super::{ChunkLocation, PlanetLocation, TileType};
use crate::simulation::{idxmap, terrain::REGIONS, CHUNK_SIZE};
use lazy_static::*;
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};

pub enum ChangeRequest {
    RevealTile { idx: usize },
    SolidTile { idx: usize, material: usize },
    EmptyTile { idx: usize },
    Floor { idx: usize, material: usize },
}

pub struct MapChangeBatch {
    region: PlanetLocation,
    changes: Vec<ChangeRequest>,
}

impl MapChangeBatch {
    pub fn new(region: PlanetLocation) -> Self {
        Self {
            region,
            changes: Vec::new(),
        }
    }

    pub fn enqueue_change(&mut self, change: ChangeRequest) {
        self.changes.push(change);
    }
}

lazy_static! {
    static ref CHANGE_BATCHES: RwLock<Vec<MapChangeBatch>> = RwLock::new(Vec::new());
}

pub fn submit_change_batch(batch: MapChangeBatch) {
    CHANGE_BATCHES.write().push(batch);
}

pub fn terrain_changes_requested() -> bool {
    !CHANGE_BATCHES.read().is_empty()
}

pub fn process_terrain_changes() -> HashMap<PlanetLocation, HashSet<ChunkLocation>> {
    let mut refresh_chunks: HashMap<PlanetLocation, HashSet<ChunkLocation>> = HashMap::new();
    while terrain_changes_requested() {
        let batch = CHANGE_BATCHES.write().pop().unwrap();
        let region_id = batch.region;
        let dirty_chunks = process_batch(batch);
        if let Some(r) = refresh_chunks.get_mut(&region_id) {
            for d in dirty_chunks.iter() {
                r.insert(d.clone());
            }
        } else {
            refresh_chunks.insert(region_id, dirty_chunks);
        }
    }
    refresh_chunks
}

fn process_batch(batch: MapChangeBatch) -> HashSet<ChunkLocation> {
    let mut refresh_chunks = HashSet::new();

    let mut region_lock = REGIONS.write();
    if let Some(region) = region_lock.regions.get_mut(&batch.region.to_region_index()) {
        for cmd in batch.changes.iter() {
            match *cmd {
                ChangeRequest::RevealTile { idx } => {
                    region.revealed[idx] = true;
                    add_chunk(&mut refresh_chunks, idx);
                }
                ChangeRequest::SolidTile { idx, material } => {
                    region.tile_types[idx] = TileType::Solid;
                    region.material[idx] = material;
                    add_chunk(&mut refresh_chunks, idx);
                }
                ChangeRequest::EmptyTile{idx} => {
                    region.tile_types[idx] = TileType::Empty;
                    add_chunk(&mut refresh_chunks, idx);
                }
                ChangeRequest::Floor{idx, material} => {
                    region.tile_types[idx] = TileType::Floor;
                    region.material[idx] = material;
                    add_chunk(&mut refresh_chunks, idx);
                }
            }
        }
    }

    refresh_chunks
}

fn add_chunk(chunks: &mut HashSet<ChunkLocation>, tile_id: usize) {
    let (x, y, z) = idxmap(tile_id);
    let cx = (x / CHUNK_SIZE) * CHUNK_SIZE;
    let cy = (y / CHUNK_SIZE) * CHUNK_SIZE;
    let cz = (z / CHUNK_SIZE) * CHUNK_SIZE;
    chunks.insert(ChunkLocation {
        x: cx,
        y: cy,
        z: cz,
    });
}
