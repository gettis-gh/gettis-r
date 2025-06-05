pub struct Size {
    pub width: usize,
    pub height: usize
}

impl Size {
    pub fn to_u32(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }
}

#[derive(Clone)]
pub struct Triangle3 {
    pub point_a: Point3,
    pub point_b: Point3,
    pub point_c: Point3,
    pub color: Color
}

impl Triangle3 {
    pub fn bounding_box(&self) -> BoundingBox {
        let xs = [self.point_a.pos_x, self.point_b.pos_x, self.point_c.pos_x];
        let ys = [self.point_a.pos_y, self.point_b.pos_y, self.point_c.pos_y];

        BoundingBox {
            left: xs.iter().cloned().fold(f32::INFINITY, f32::min),
            right: xs.iter().cloned().fold(f32::NEG_INFINITY, f32::max),
            bottom: ys.iter().cloned().fold(f32::INFINITY, f32::min),
            top: ys.iter().cloned().fold(f32::NEG_INFINITY, f32::max),
        }
    }

    pub fn contains_point(&self, fx: f32, fy: f32) -> bool {
        let (ax, ay) = (self.point_a.pos_x, self.point_a.pos_y);
        let (bx, by) = (self.point_b.pos_x, self.point_b.pos_y);
        let (cx, cy) = (self.point_c.pos_x, self.point_c.pos_y);

        let v0x = cx - ax;
        let v0y = cy - ay;
        let v1x = bx - ax;
        let v1y = by - ay;
        let v2x = fx - ax;
        let v2y = fy - ay;

        let dot00 = v0x * v0x + v0y * v0y;
        let dot01 = v0x * v1x + v0y * v1y;
        let dot02 = v0x * v2x + v0y * v2y;
        let dot11 = v1x * v1x + v1y * v1y;
        let dot12 = v1x * v2x + v1y * v2y;

        let denom = dot00 * dot11 - dot01 * dot01;
        if denom == 0.0 {
            return false; // Triángulo degenerado
        }

        let inv_denom = 1.0 / denom;
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        (u >= 0.0) && (v >= 0.0) && (u + v <= 1.0)
    }
}

pub struct BoundingBox {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

#[derive(Copy, Clone)]
pub struct Point3 {
    pub pos_x: f32,
    pub pos_y: f32,
    #[allow(dead_code)]
    pub pos_z: f32,
}

#[derive(Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

pub struct Mesh {
    pub vertices: Vec<Point3>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn to_triangles(&self) -> Vec<Triangle3> {
        let mut triangles = Vec::new();
        for chunk in self.indices.chunks(3) {
            if chunk.len() == 3 {
                let point_a = self.vertices[chunk[0] as usize].clone();
                let point_b = self.vertices[chunk[1] as usize].clone();
                let point_c = self.vertices[chunk[2] as usize].clone();
    
                triangles.push(Triangle3 {
                    point_a,
                    point_b,
                    point_c,
                    color: Color { red: 255, green: 255, blue: 255, alpha: 255 },
                });
            }
        }
        triangles
    }
}