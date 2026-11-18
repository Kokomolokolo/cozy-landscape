use std::collections::HashMap;

use bevy::{ecs::system::SystemParam, prelude::*};
use noise::Perlin;

use crate::{AppState, ChunkPos, preloader::GameAssets, world::{biomes::BiomeRegistry, spawning::{ChunkTasks, despawn_chunks, handle_spawn_tasks, spawn_around_player}}};

pub const CHUNK_SIZE: i32 = 32;
pub use spawning::RENDER_DIST;

mod spawning;
mod meshing;
mod tarrain;
mod biomes;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>();
        app.init_resource::<TarrainGenerator>();
        app.init_resource::<ChunkTasks>();
        app.add_systems(Update, (spawn_around_player, handle_spawn_tasks).run_if(in_state(AppState::InGame)));
        app.add_systems(Update, (despawn_chunks).run_if(in_state(AppState::InGame)));
    }
}
#[derive(Resource, Default)]
pub struct ChunkMap(
    HashMap<ChunkPos, Entity>
);

#[derive(SystemParam)]
pub struct ChunkParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub chunk_map: ResMut<'w, ChunkMap>,
    pub material: ResMut<'w, Assets<StandardMaterial>>,
    // Die assets für die Chunks
    pub assets: Res<'w, GameAssets>,
}

#[derive(Resource, Clone)]
pub struct TarrainGenerator {
    pub seed: u32,
    tarrain_noise: Perlin,
    pub detail_noise: Perlin,
    biom_noise: Perlin,
    registry: BiomeRegistry,
}
