use glam::Vec3;

pub fn sdf_union<'a>(
    a: impl Fn(Vec3) -> f32 + Sync + 'a,
    b: impl Fn(Vec3) -> f32 + Sync + 'a,
) -> Box<dyn Fn(Vec3) -> f32 + Sync + 'a> {
    Box::new(move |p| a(p).min(b(p)))
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn mix(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

pub fn sdf_smooth_union<'a>(
    d1: impl Fn(Vec3) -> f32 + Sync + 'a,
    d2: impl Fn(Vec3) -> f32 + Sync + 'a,
    k: f32,
) -> Box<dyn Fn(Vec3) -> f32 + Sync + 'a> {
    Box::new(move |p: Vec3| {
        let d1_val = d1(p);
        let d2_val = d2(p);
        let h = clamp(0.5 + 0.5 * (d2_val - d1_val) / k, 0.0, 1.0);
        mix(d2_val, d1_val, h) - k * h * (1.0 - h)
    })
}

pub fn sdf_sphere(center: Vec3, radius: f32) -> impl Fn(Vec3) -> f32 + Sync {
    move |p: Vec3| (p - center).length() - radius
}

pub fn sdf_box(center: Vec3, half_size: Vec3) -> impl Fn(Vec3) -> f32 + Sync {
    move |p: Vec3| {
        let d = (p - center).abs() - half_size;
        d.max(Vec3::ZERO).length() + d.min(Vec3::ZERO).max_element()
    }
}

pub trait Lerp {
    fn lerp(self, other: Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(self, other: f32, t: f32) -> f32 {
        self * (1.0 - t) + other * t
    }
}