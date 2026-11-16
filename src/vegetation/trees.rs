use bevy::prelude::*;

use crate::{
    ChunkPos, player::Player, world::{CHUNK_SIZE, ChunkParams, TarrainGenerator},
};

#[derive(Component)]
pub struct TreeRoot;

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

                let lod3 = spawner.assets.assets.get("tree_large_3.glb").unwrap().clone();
                let lod2 = spawner.assets.assets.get("tree_large_2.glb").unwrap().clone();
                let lod1 = spawner.assets.assets.get("tree_large_1.glb").unwrap().clone();
                let lod0 = spawner.assets.assets.get("tree_large_0.glb").unwrap().clone();

                spawner.commands.entity(chunk_entity).with_children(|parent| {
                    parent.spawn((
                        Transform::from_translation(local_pos),
                        Visibility::Inherited,
                        TreeRoot,
                    )).with_children(|tree_parent| {
                        // Geringster LOD, sichtbar wird aber direkt geupdatet
                        tree_parent.spawn((
                            WorldAssetRoot(lod0),
                            Visibility::Hidden
                        ));
                        tree_parent.spawn((
                            WorldAssetRoot(lod1),
                            Visibility::Hidden
                        ));
                        tree_parent.spawn((
                            WorldAssetRoot(lod2),
                            Visibility::Hidden
                        ));
                        tree_parent.spawn((
                            WorldAssetRoot(lod3),
                            Visibility::Inherited
                        ));
                    });

                });
            }
        }
    }
}

fn should_spawn_tree(x: f32, z: f32) -> bool {
    // Simulierter Check (kann durch echten Noise-Wert ersetzt werden)
    (x.sin() * z.cos()).abs() > 0.99
}

pub fn update_tree_lods(
    q_player: Query<&Transform, With<Player>>,
    q_tree: Query<(&GlobalTransform, &Children), With<TreeRoot>>,
    mut visibility_q: Query<&mut Visibility>
) {
    let Ok(player_t) = q_player.single() else {
        return;
    };
    let player_pos = player_t.translation;

    const LOD0_DIST: f32 = 20.0 * 20.0;
    const LOD1_DIST: f32 = 50.0 * 50.0;
    const LOD2_DIST: f32 = 100.0 * 100.0;

    for (tree_gt, children) in &q_tree {
        let dist_sq = player_pos.distance_squared(tree_gt.translation());

        let active_idx = if dist_sq < LOD0_DIST {
            0
        } else if dist_sq < LOD1_DIST {
            1
        } else if dist_sq < LOD2_DIST {
            2
        } else {
            3
        };

        for (i, child_entity) in children.iter().enumerate() {
            if let Ok(mut visibility) = visibility_q.get_mut(child_entity) {
                if i == active_idx {
                    *visibility = Visibility::Inherited;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}