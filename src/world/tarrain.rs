use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

use crate::{ChunkPos, world::{CHUNK_SIZE, TarrainGenerator, biomes::{Biome, BiomeRegistry, lerp}}};

impl TarrainGenerator {
    pub fn get_height_and_biom(&self, world_x: f32, world_z: f32) -> (f32, &Biome) {
        let biom_val = self.biom_noise.get([
            world_x as f64 * 0.005,
            world_z as f64 * 0.005
        ]) as f32;
        
        // Große Landschaftsformen
        let large_noise = self.tarrain_noise.get([
            world_x as f64 * 0.0015,
            world_z as f64 * 0.0015
        ]) as f32;
        
        // Mittlere Strukturen
        let base_noise = self.tarrain_noise.get([
            world_x as f64 * 0.006,
            world_z as f64 * 0.006
        ]) as f32;
        
        // Kleine Details
        let detail_noise = self.detail_noise.get([
            world_x as f64 * 0.025,
            world_z as f64 * 0.025
        ]) as f32;

        let (b1, b2, t) = self.registry.sample(biom_val as f32);

        let h1 = b1.calculate_height(base_noise, detail_noise, large_noise);
        let h2 = b2.calculate_height(base_noise, detail_noise, large_noise);

        let final_height = lerp(h1, h2, t);
        let primary_biom = if t < 0.5 { b1 } else { b1 }; 

        (final_height as f32, primary_biom)
    }
    pub fn generate_heightmap_for_chunk(
        &self, pos: ChunkPos,
    ) -> Vec<f32> {
        let chunk_offset = pos.to_world();
        let mut height_map = Vec::new();
        
        for x in 0..=CHUNK_SIZE {
            for z in 0..=CHUNK_SIZE {
                let pos = chunk_offset + Vec2::new(x as f32, z as f32);

                let (height, _biom) = self.get_height_and_biom(pos.x, pos.y);

                height_map.push(height);
            }
        }
        height_map
    }
    //pub fn calculate_surface_height(&self, world_x: f32, world_z: f32) -> f32 {
    //    let spread_tar = 0.01;
    //    let spread_det = 0.1;
    //    let tarrain_noise = self.tarrain_noise.get([world_x as f64 * spread_tar, world_z as f64 * spread_tar]) as f32;
    //    let detail_noise = self.detail_noise.get([world_x as f64 * spread_det, world_z as f64 * spread_det]) as f32;

    //    let final_heigt = tarrain_noise * 10.0 + detail_noise * 0.3 + 5.0;

    //    final_heigt
    //}
}

impl Default for TarrainGenerator {
    fn default() -> Self {
        let seed = 42;
        Self {
            seed: seed,
            tarrain_noise: Perlin::new(seed),
            detail_noise: Perlin::new(seed + 42),
            biom_noise: Perlin::new(seed + 420),
            registry: BiomeRegistry::new(),
        }
    }
}