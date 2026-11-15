use bevy::prelude::*;

mod trees;

pub use trees::spawn_trees_for_chunk;

pub struct VegetationPlugin;

impl Plugin for VegetationPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

