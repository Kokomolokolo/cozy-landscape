use bevy::prelude::*;

mod trees;
mod grass;

pub use trees::spawn_trees_for_chunk;
use trees::update_tree_lods;

use crate::{ChunkPos, vegetation::grass::{spawn_grass_for_chunk, update_grass_lods}, world::{ChunkParams, TarrainGenerator}};

pub struct VegetationPlugin;

impl Plugin for VegetationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (update_tree_lods, update_grass_lods));
    }
}

pub fn spawn_vegeation_for_chunk(
    chunk_entity: Entity,
    pos: ChunkPos,
    spawner: &mut ChunkParams,
    tarrain_generator: &Res<TarrainGenerator>,
) {
    spawn_trees_for_chunk(chunk_entity, pos, spawner, tarrain_generator);
    // spawn_grass_for_chunk(chunk_entity, pos, spawner, tarrain_generator);
}


fn hash2d(x: i32, z: i32, seed: u32) -> f32 {
    let mut h = (x as u32)
        .wrapping_mul(374761393)
        .wrapping_add((z as u32).wrapping_mul(668265263))
        .wrapping_add(seed);
    h = (h ^ (h >> 13)).wrapping_mul(1274126161);
    (h ^ (h >> 16)) as f32 / u32::MAX as f32
}