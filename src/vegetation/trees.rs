use bevy::prelude::*;

use crate::{
    ChunkPos,
    world::{CHUNK_SIZE, ChunkParams, TarrainGenerator},
};

pub fn spawn_trees_for_chunk(
    chunk_entity: Entity,
    pos: ChunkPos,
    spawner: &mut ChunkParams,
    tarrain_generator: &Res<TarrainGenerator>,
) {
    for x in (0..CHUNK_SIZE).step_by(4) {
        for z in (0..CHUNK_SIZE).step_by(4) {
            let world_x = pos.0.x as f32 * CHUNK_SIZE as f32 + x as f32;
            let world_z = pos.0.y as f32 * CHUNK_SIZE as f32 + z as f32;

            if should_spawn_tree(world_x, world_z) {
                let y = tarrain_generator.calculate_surface_height(world_x, world_z);

                // Lokal, da es ein Kind des Chunks wird
                let local_pos = Vec3::new(x as f32, y, z as f32);

                let tree_mesh = spawner.assets.assets.get("tree_large_3.glb").unwrap().clone();

                spawner.commands.entity(chunk_entity).with_children(|parent| {
                    parent.spawn((
                        WorldAssetRoot(
                            tree_mesh
                        ),
                        Transform::from_translation(local_pos),
                        Visibility::Inherited,
                    ));
                });
            }
        }
    }
}

fn should_spawn_tree(x: f32, z: f32) -> bool {
    // Simulierter Check (kann durch echten Noise-Wert ersetzt werden)
    (x.sin() * z.cos()).abs() > 0.7
}
