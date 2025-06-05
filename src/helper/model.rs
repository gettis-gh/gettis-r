use nalgebra::Vector3;
use crate::structs::geometry::{Mesh, Point3};

pub fn load_to_meshes(path: &str) -> Vec<Mesh> {
    let (gltf, buffers, _) = gltf::import(path).expect("Failed to load GLTF file");

    let mut meshes = Vec::new();

    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let positions: Vec<Point3> = reader
                .read_positions()
                .unwrap()
                .map(|v| Point3 {
                    pos_x: v[0],
                    pos_y: v[1],
                    pos_z: v[2],
                })
                .collect();

            let indices = reader
                .read_indices()
                .map(|read_indices| read_indices.into_u32().collect::<Vec<_>>())
                .unwrap_or_else(|| (0..positions.len() as u32).collect());

            meshes.push(Mesh {
                vertices: positions,
                indices,
            });
        }
    }

    meshes
}