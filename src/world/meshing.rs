use bevy::{asset::RenderAssetUsages, mesh::Indices, prelude::*};

use crate::{ChunkPos, world::{CHUNK_SIZE, TarrainGenerator, biomes::Biome}};

pub fn generate_ground_mesh(
    pos: ChunkPos, 
    tarrain_generator: &TarrainGenerator,
) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();

    let chunk_offset = pos.to_world();

    let epsilon = 0.1; // Die Schrittgröße, sehr klei

    for x in 0..=CHUNK_SIZE {
        for z in 0..=CHUNK_SIZE {
            let x_f = x as f32;
            let z_f = z as f32;

            let world_x = chunk_offset.x + x as f32;
            let world_z = chunk_offset.y + z as f32;

            let get_height = |wx: f32, wz: f32| -> f32 {
                let (height, biom) = tarrain_generator.get_height_and_biom(wx, wz);
                height
            };

            // Höhen für die Vertecies
            // hier noise für y
            let y = get_height(world_x, world_z);
            positions.push([x_f, y, z_f]);

            // Berechnung für die Normalen
            // Schritte in jede Richtung gehen und die höhenunterschiede als Vektoren nehmen.
            // Daraus dann das Kreuzprodukt
            let h_left = get_height(world_x - epsilon, world_z);
            let h_right = get_height(world_x + epsilon, world_z);
            
            let h_up = get_height(world_x, world_z + epsilon);
            let h_down = get_height(world_x, world_z - epsilon);

            let dx = (h_left - h_right) / (2.0 * epsilon);
            let dz = (h_up - h_down) / (2.0 * epsilon);

            let nx = -dx;
            let ny = 1.0;
            let nz = -dz;

            // Normalisieren
            let length = (nx * nx + ny * ny + nz * nz).sqrt();
            if length > 0.0 {
                let norm_x = nx / length;
                let norm_z = nz / length;
                normals.push([norm_x, ny, norm_z]);
            } else { // Fallback
                normals.push([0.0, 1.0, 0.0]);
            }
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