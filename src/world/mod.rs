use std::collections::HashMap;

use bevy::{ecs::system::SystemParam, prelude::*};
use noise::Perlin;

use crate::{AppState, ChunkPos, preloader::GameAssets, world::spawning::{despawn_chunks, spawn_around_player}};

pub const CHUNK_SIZE: i32 = 16;

mod spawning;
mod meshing;
mod tarrain;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>();
        app.init_resource::<TarrainGenerator>();
        app.add_systems(Update, (spawn_around_player).run_if(in_state(AppState::InGame)));
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

#[derive(Resource)]
pub struct TarrainGenerator {
    seed: u32,
    tarrain_noise: Perlin,
    detail_noise: Perlin,
    // Gff. Biom Noise, obwohl das eigentlich woander her kommen sollte denke ich
}
