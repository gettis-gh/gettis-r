use glam::Vec3;
use crate::structs::geometry::Color;

fn estimate_normal(p: Vec3, sdf: &(impl Fn(Vec3) -> f32 + Sync + ?Sized)) -> Vec3 {
    let epsilon = 0.001;
    Vec3::new(
        sdf(Vec3::new(p.x + epsilon, p.y, p.z)) - sdf(Vec3::new(p.x - epsilon, p.y, p.z)),
        sdf(Vec3::new(p.x, p.y + epsilon, p.z)) - sdf(Vec3::new(p.x, p.y - epsilon, p.z)),
        sdf(Vec3::new(p.x, p.y, p.z + epsilon)) - sdf(Vec3::new(p.x, p.y, p.z - epsilon)),
    ).normalize()
}

pub fn trace(
    origin: Vec3,
    dir: Vec3,
    sdf: &(dyn Fn(Vec3) -> f32 + Sync)
) -> Color {
    let max_steps = 100;
    let max_distance = 100.0;
    let epsilon = 0.01;
    let light_dir = Vec3::new(1.0, 1.0, -1.0).normalize();

    let mut t = 0.0;
    for _ in 0..max_steps {
        let p = origin + dir * t;
        let dist = sdf(p); // 🔄 ya no iteras sobre múltiples SDFs

        if dist < epsilon {
            let normal = estimate_normal(p, sdf);
            let diffuse = normal.dot(light_dir).max(0.0);

            // Color base, verde, escalado por iluminación
            let intensity = (diffuse * 255.0) as u8;

            return Color { red: 0, green: intensity, blue: 0, alpha: 255 }; // hit
        }

        if t > max_distance {
            break;
        }

        t += dist;
    }

    Color { red: 25, green: 25, blue: 25, alpha: 255 } // fondo
}
