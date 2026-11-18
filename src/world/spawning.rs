use std::collections::HashMap;

use bevy::{prelude::*, tasks::{AsyncComputeTaskPool, Task, futures_lite::future}};
use avian3d::prelude::*;

use crate::{ChunkPos, player::Player, vegetation::spawn_vegeation_for_chunk, world::{CHUNK_SIZE, ChunkParams, TarrainGenerator, meshing::generate_ground_mesh}};
use crate::vegetation::spawn_trees_for_chunk;

#[derive(Resource, Default)]
pub struct ChunkTasks {
    pub tasks: HashMap<IVec2, Task<(Mesh, Collider)>>
}

pub const RENDER_DIST: i32 = 20;
const MAX_NEW_TAKS_PER_FRAME: i32 = 8;

pub fn spawn_around_player(
    mut spawner: ChunkParams,
    player_q: Query<&Transform, With<Player>>,
    tarrain_generator: Res<TarrainGenerator>,
    mut chunk_tasks: ResMut<ChunkTasks>,
) {
    let Ok(player_t) = player_q.single() else {
        return;
    };

    let pool = AsyncComputeTaskPool::get();

    let avail = MAX_NEW_TAKS_PER_FRAME.saturating_sub(chunk_tasks.tasks.len() as i32) as usize;

    if avail == 0 {
        return;
    }
    let mut candidates = Vec::new();
    
    for x in -RENDER_DIST..=RENDER_DIST {
        for z in -RENDER_DIST..=RENDER_DIST {
            let player_chunk = IVec2::new(
                (player_t.translation.x.floor() / CHUNK_SIZE as f32) as i32 + x, 
                (player_t.translation.z.floor() / CHUNK_SIZE as f32) as i32 + z,
            );
            
            if spawner.chunk_map.0.contains_key(&ChunkPos(player_chunk)) {
                continue;
            }
            if chunk_tasks.tasks.contains_key(&player_chunk) {
                continue;
            }

            candidates.push(player_chunk);
        }
    }

    candidates.sort_by_key(|pos| pos.length_squared());

    for pos in candidates.into_iter().take(avail) {
        let generator = tarrain_generator.as_ref().clone();
        let task = pool.spawn(async move {
            let mesh = generate_ground_mesh(ChunkPos(pos), &generator);

            let collider = Collider::trimesh_from_mesh(&mesh).unwrap();
            (mesh, collider)
        });

        chunk_tasks.tasks.insert(pos, task);
    }
}

pub fn handle_spawn_tasks(
    mut chunk_tasks: ResMut<ChunkTasks>,
    mut spawner: ChunkParams,
    tarrain_generator: Res<TarrainGenerator>,
) {
    let mut completed = Vec::new();
    
    for (&pos, task) in chunk_tasks.tasks.iter_mut() {
        if let Some(mesh) = future::block_on(
            future::poll_once(task)
        ) {
            completed.push((pos, mesh));
        }
    }

    for (pos, (chunk_mesh, chunk_collider)) in completed {
        chunk_tasks.tasks.remove(&pos);

        let chunk_pos = ChunkPos(pos);

        let entity = spawner.commands.spawn((
            Mesh3d(
                spawner.meshes.add(chunk_mesh)
            ),
            MeshMaterial3d(
                spawner.material.add(StandardMaterial {
                    base_color: Color::srgb(0.2, 0.6, 0.2),
                    ..default()
                })
            ),
            chunk_collider,
            RigidBody::Static,
            Transform::from_translation(
                Vec3::new(
                    pos.x as f32 * CHUNK_SIZE as f32, 
                    0.0, 
                    pos.y as f32 * CHUNK_SIZE as f32
                )
            ),
        )).id();

        spawn_vegeation_for_chunk(entity, chunk_pos, &mut spawner, &tarrain_generator);

        spawner.chunk_map.0.insert(ChunkPos(pos), entity);
    }
}

// Generiert alles am Chunk, wie boden bäume etc.
pub fn spawn_chunk(
    pos: ChunkPos,
    mut spawner: &mut ChunkParams,
    tarrain_generator: &Res<TarrainGenerator>,
) {
    if spawner.chunk_map.0.contains_key(&pos) {
        return;
    }
    
    let chunk_mesh = generate_ground_mesh(pos, tarrain_generator);

    let collider = Collider::trimesh_from_mesh(&chunk_mesh).unwrap();

    let entity = spawner.commands.spawn((
        Mesh3d(
            spawner.meshes.add(chunk_mesh)
        ),
        MeshMaterial3d(
            spawner.material.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.6, 0.2),
                ..default()
            })
        ),
        collider,
        RigidBody::Static,
        Transform::from_translation(
            Vec3::new(
                pos.0.x as f32 * CHUNK_SIZE as f32, 
                0.0, 
                pos.0.y as f32 * CHUNK_SIZE as f32
            )
        ),
    )).id();
    spawn_trees_for_chunk(entity, pos, spawner, tarrain_generator);
    spawner.chunk_map.0.insert(pos, entity);
}


pub fn despawn_chunks(
    q_player: Query<&Transform, With<Player>>,
    mut spawner: ChunkParams,
) {
    let Ok(t_player) = q_player.single() else {
        return;
    };

    let player_chunk = IVec2::new(
        (t_player.translation.x.floor() / CHUNK_SIZE as f32) as i32, 
        (t_player.translation.z.floor() / CHUNK_SIZE as f32) as i32,
    );

    let keep_dist = RENDER_DIST + 2;
    
    let outsite_chunks = spawner.chunk_map.0.extract_if(|pos, _| {
        (pos.0.x - player_chunk.x).abs() > keep_dist
        || (pos.0.y - player_chunk.y).abs() > keep_dist
    });

    for (_, entity) in outsite_chunks {
        spawner.commands.entity(entity).despawn();
    }
}