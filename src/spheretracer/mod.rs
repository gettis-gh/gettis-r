use crate::helper::spheretracer::trace;
use rayon::prelude::*;
use crate::structs::geometry::Size;
use glam::Vec3;

pub fn render(
    sdf: &(dyn Fn(Vec3) -> f32 + Sync),
    frame_size: &Size
) -> Vec<u8> {
    let color_buffer: Vec<u8> = (0..(frame_size.width * frame_size.height))
        .into_par_iter()
        .flat_map(|pixel_index| {
            let pixel_pos_x = pixel_index % frame_size.width;
            let pixel_pos_y = pixel_index / frame_size.width;

            let aspect_ratio = frame_size.width as f32 / frame_size.height as f32;

            let normalized_pixel_pos_x = (pixel_pos_x as f32 / frame_size.width as f32 - 0.5) * 10.0 * aspect_ratio;
            let normalized_pixel_pos_y = (pixel_pos_y as f32 / frame_size.height as f32 - 0.5) * 10.0;

            let ray_origin = Vec3::new(normalized_pixel_pos_x, normalized_pixel_pos_y, -10.0);
            let ray_direction = Vec3::new(0.0, 0.0, 1.0);

            let traced_color = trace(ray_origin, ray_direction, sdf);

            vec![
                traced_color.red,
                traced_color.green,
                traced_color.blue,
                traced_color.alpha,
            ]
        })
        .collect();

    color_buffer
}
