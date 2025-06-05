use crate::structs::geometry::{Size, Triangle3, Color};
use crate::helper::rasterizer::create_spatial_grid;
use rayon::prelude::*;

pub fn rasterize_tb(triangles: &Vec<Triangle3>, frame_size: &Size, grid_size: Size) -> Vec<u8> {
    let grid = create_spatial_grid(triangles, &frame_size, &grid_size);

    let cell_width = frame_size.width as f32 / grid_size.width as f32;
    let cell_height = frame_size.height as f32 / grid_size.height as f32;

    let color_buffer: Vec<u8> = (0..(frame_size.width * frame_size.height))
        .into_par_iter() // usa rayon para paralelismo
        .flat_map_iter(|pixel_index| {
            let px = pixel_index % frame_size.width;
            let py = pixel_index / frame_size.width;

            let fx = px as f32 + 0.5;
            let fy = py as f32 + 0.5;

            let cell_x = (fx / cell_width).floor() as usize;
            let cell_y = (fy / cell_height).floor() as usize;

            let cell_x = cell_x.min(grid_size.width - 1);
            let cell_y = cell_y.min(grid_size.height - 1);
            let grid_index = cell_y * grid_size.width + cell_x;

            let mut final_color = Color { red: 25, green: 25, blue: 25, alpha: 255 };

            for tri in &grid[grid_index] {
                if tri.contains_point(fx, fy) {
                    final_color = tri.color;
                    break;
                } else {
                    final_color = Color {
                        red: 50,
                        green: 50,
                        blue: 50,
                        alpha: 255
                    };
                }
            }

            // Devolver los 4 bytes RGBA
            vec![final_color.red, final_color.green, final_color.blue, final_color.alpha]
        })
        .collect();

    color_buffer
}

// pub fn rasterize_deferred(triangles: &Vec<Triangle3>, frame_size: &Size) -> Vec<u8> {
//     let fragment_buffer = None;
//     let color_buffer = None;
// }