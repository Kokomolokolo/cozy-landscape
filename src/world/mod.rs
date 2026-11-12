use std::collections::HashMap;

use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{AppState, world::chunk::spawn_around_player};

mod chunk;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>();
        app.add_systems(OnEnter(AppState::InGame), spawn_around_player);
    }
}
#[derive(Resource, Default)]
pub struct ChunkMap(
    HashMap<[i32; 2], Mesh>
);

#[derive(SystemParam)]
pub struct ChunkParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub chunk_map: ResMut<'w, ChunkMap>,
    pub material: ResMut<'w, Assets<StandardMaterial>>,
}