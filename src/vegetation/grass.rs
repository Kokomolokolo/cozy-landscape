use bevy::prelude::*;

use crate::{ChunkPos, player::Player, vegetation::hash2d, world::{CHUNK_SIZE, ChunkParams, TarrainGenerator}};

#[derive(Component)]
pub struct GrassRoot;

pub fn spawn_grass_for_chunk(
    chunk_entity: Entity,
    pos: ChunkPos,
    spawner: &mut ChunkParams,
    tarrain_generator: &Res<TarrainGenerator>,
) {
    for x in (0..CHUNK_SIZE).step_by(1) {
        for z in (0..CHUNK_SIZE).step_by(1) {
            let world_x = pos.0.x as f32 * CHUNK_SIZE as f32 + x as f32;
            let world_z = pos.0.y as f32 * CHUNK_SIZE as f32 + z as f32;

            if should_spawn_grass(x, z, tarrain_generator.seed) {
                let (y, _biome) = tarrain_generator.get_height_and_biom(world_x, world_z);

                // Lokal, da es ein Kind des Chunks wird
                let local_pos = Vec3::new(x as f32, y, z as f32);

                let lod3 = spawner.assets.assets.get("low_grass_3.glb").unwrap().clone();
                let lod2 = spawner.assets.assets.get("low_grass_2.glb").unwrap().clone();
                let lod1 = spawner.assets.assets.get("low_grass_1.glb").unwrap().clone();
                let lod0 = spawner.assets.assets.get("low_grass_0.glb").unwrap().clone();

                spawner.commands.entity(chunk_entity).with_children(|parent| {
                    parent.spawn((
                        Transform::from_translation(local_pos),
                        Visibility::Inherited,
                        GrassRoot,
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
                            Visibility::Hidden
                        ));
                    });

                });
            }
        }
    }
}

fn should_spawn_grass(x: i32, z: i32, seed: u32) -> bool {
    hash2d(x, z, seed) < 0.1
}


pub fn update_grass_lods(
    q_player: Query<&Transform, With<Player>>,
    q_tree: Query<(&GlobalTransform, &Children), With<GrassRoot>>,
    mut visibility_q: Query<&mut Visibility>
) {
    let Ok(player_t) = q_player.single() else {
        return;
    };
    let player_pos = player_t.translation;

    const LOD0_DIST: f32 = 3.0 * 3.0;
    const LOD1_DIST: f32 = 5.0 * 5.0;
    const LOD2_DIST: f32 = 10.0 * 10.0;
    const LOD3_DIST: f32 = 50.0 * 50.0;

    for (tree_gt, children) in &q_tree {
        let dist_sq = player_pos.distance_squared(tree_gt.translation());

        let active_idx = if dist_sq < LOD0_DIST {
            0
        } else if dist_sq < LOD1_DIST {
            1
        } else if dist_sq < LOD2_DIST {
            2
        } else if dist_sq < LOD3_DIST{
            3
        } else {
            67
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