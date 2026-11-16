use bevy::prelude::*;

mod trees;

pub use trees::spawn_trees_for_chunk;
use trees::update_tree_lods;

pub struct VegetationPlugin;

impl Plugin for VegetationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_tree_lods);
    }
}

