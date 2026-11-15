use noise::{NoiseFn, Perlin};

use crate::world::TarrainGenerator;

impl TarrainGenerator {
    pub fn calculate_surface_height(&self, world_x: f32, world_z: f32) -> f32 {
        let spread_tar = 0.01;
        let spread_det = 0.1;
        let tarrain_noise = self.tarrain_noise.get([world_x as f64 * spread_tar, world_z as f64 * spread_tar]) as f32;
        let detail_noise = self.detail_noise.get([world_x as f64 * spread_det, world_z as f64 * spread_det]) as f32;

        let final_heigt = tarrain_noise * 10.0 + detail_noise * 0.3 + 5.0;

        final_heigt
    }
}

impl Default for TarrainGenerator {
    fn default() -> Self {
        let seed = 42;
        Self {
            seed: seed,
            tarrain_noise: Perlin::new(seed),
            detail_noise: Perlin::new(seed + 42)
        }
    }
}