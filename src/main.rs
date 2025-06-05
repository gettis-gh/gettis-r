mod helper;
mod rasterizer;
mod structs;
mod spheretracer;

use crate::spheretracer::render as st_render;
use crate::structs::geometry::Size;
use crate::helper::ascii::render_ascii_art;
use crate::helper::bmp::save_bmp;
use crate::helper::global::{
    sdf_union, sdf_box, sdf_sphere, sdf_smooth_union
};

use std::f32::consts::PI;
use rand::Rng;
use glam::Vec3;

fn rotate(p: Vec3, angles: Vec3) -> Vec3 {
    let (sin_x, cos_x) = angles.x.sin_cos();
    let (sin_y, cos_y) = angles.y.sin_cos();
    let (sin_z, cos_z) = angles.z.sin_cos();

    // Rotación en X
    let mut p = Vec3::new(
        p.x,
        cos_x * p.y - sin_x * p.z,
        sin_x * p.y + cos_x * p.z,
    );

    // Rotación en Y
    p = Vec3::new(
        cos_y * p.x + sin_y * p.z,
        p.y,
        -sin_y * p.x + cos_y * p.z,
    );

    // Rotación en Z
    p = Vec3::new(
        cos_z * p.x - sin_z * p.y,
        sin_z * p.x + cos_z * p.y,
        p.z,
    );

    p
}

fn rotate_sdf(
    sdf: impl Fn(Vec3) -> f32 + Sync + 'static,
    angles: Vec3,
    pivot: Vec3, // <-- añadido
) -> impl Fn(Vec3) -> f32 + Sync {
    move |p: Vec3| {
        // Restamos el pivot para rotar alrededor de él
        let local_p = p - pivot;
        let rotated_p = rotate(local_p, -angles);
        let final_p = rotated_p + pivot; // Volvemos al sistema global
        sdf(final_p)
    }
}

fn infinite_repeat_sdf(
    sdf: impl Fn(Vec3) -> f32 + Sync + 'static,
    spacing: Vec3,
) -> impl Fn(Vec3) -> f32 + Sync {
    move |p: Vec3| {
        let repeat_idx = Vec3::new(
            (p.x / spacing.x).floor().max(0.0),
            (p.y / spacing.y).floor().max(0.0),
            (p.z / spacing.z).floor().max(0.0),
        );
        let q = p - spacing * repeat_idx;
        sdf(q)
    }
}

fn main() {
    println!("Welcome to Getti's Renderer");

    let size = Size { width: 800, height: 600 };

    // Centrar el cubo más cerca para que quede dentro del tile base
    let cube_center = Vec3::new(0.0, 0.0, 0.0);
    let cube = sdf_box(cube_center, Vec3::new(1.0, 1.0, 1.0));
    let spacing = Vec3::new(3.0, 3.0, 3.0);
    let pre_scene = infinite_repeat_sdf(cube, spacing);
    let rotation_angles = Vec3::new(-PI / 1.5, PI / 1.5, -PI / 1.5);
    let pivot = Vec3::new(0.0, 0.0, 0.0);
    let scene = rotate_sdf(pre_scene, rotation_angles, pivot);

    let color_buffer = st_render(&scene, &size);

    let (width, height) = size.to_u32();
    save_bmp("output.bmp", width, height, &color_buffer);

    let ascii = render_ascii_art(&color_buffer, &size);
    println!("{}", ascii);
}