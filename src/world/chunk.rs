use bevy::{asset::RenderAssetUsages, mesh::Indices, prelude::*};

use crate::{Pos, world::ChunkParams};

const CHUNK_SIZE: i32 = 16;

pub fn spawn_around_player(mut spawner: ChunkParams) {
    for x in -5..=5 {
        for z in -5..=5 {
            spawn_chunk(Pos(IVec2::new(x, z)), &mut spawner);
        }
    }
}

// Generiert alles am Chunk, wie boden bäume etc.
pub fn spawn_chunk(
    pos: Pos,
    mut spawner: &mut ChunkParams,
) {
    let chunk_mesh = generate_ground_mesh();


    spawner.commands.spawn((
        Mesh3d(
            spawner.meshes.add(chunk_mesh)
        ),
        MeshMaterial3d(
            spawner.material.add(StandardMaterial {
                base_color: Color::srgba(0.1, 0.8, 0.1, 1.0),
                
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
    ));
}


pub fn generate_ground_mesh() -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

    for x in 0..=CHUNK_SIZE {
        for z in 0..=CHUNK_SIZE {
            let x_f = x as f32;
            let z_f = z as f32;

            // hier noise für y
            let y = x_f.sin() * z_f.cos();
            positions.push([x_f, y, z_f]);

            // Hier später kreuzprodukt
            normals.push([0.0, 1.0, 0.0]);
        }
    }

    let vertex_per_row = CHUNK_SIZE + 1;
    
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let top_left = z * vertex_per_row + x;
            let top_right = z * vertex_per_row + (x + 1);

            let bottom_left = (z + 1) * vertex_per_row + x;
            let bottom_right = (z + 1) * vertex_per_row + (x + 1);

            // 1. Dreieck
            indices.push(top_left as u32);
            indices.push(top_right as u32);
            indices.push(bottom_left as u32);

            // 2. Dreieck
            indices.push(top_right as u32);
            indices.push(bottom_right as u32);
            indices.push(bottom_left as u32);
        }
    }

    let mut mesh = Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList, 
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}