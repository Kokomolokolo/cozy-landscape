use bevy::prelude::*;

use crate::{ChunkPos, player::Player, world::{CHUNK_SIZE, ChunkParams, TarrainGenerator, meshing::generate_ground_mesh}};

const RENDER_DIST: i32 = 20;

pub fn spawn_around_player(
    mut spawner: ChunkParams,
    player_q: Query<&Transform, With<Player>>,
    tarrain_generator: Res<TarrainGenerator>,
) {
    let Ok(player_t) = player_q.single() else {
        return;
    };
    
    for x in -RENDER_DIST..=RENDER_DIST {
        for z in -RENDER_DIST..=RENDER_DIST {
            let player_chunk = IVec2::new(
                (player_t.translation.x.floor() / CHUNK_SIZE as f32) as i32 + x, 
                (player_t.translation.z.floor() / CHUNK_SIZE as f32) as i32 + z,
            );
            spawn_chunk(ChunkPos(player_chunk), &mut spawner, &tarrain_generator);
        }
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
        Transform::from_translation(
            Vec3::new(
                pos.0.x as f32 * CHUNK_SIZE as f32, 
                0.0, 
                pos.0.y as f32 * CHUNK_SIZE as f32
            )
        )
    )).id();
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