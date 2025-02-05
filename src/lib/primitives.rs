use std::ops::MulAssign;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl fmt::Display for Point3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Point3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Default for Point3d {
    fn default() -> Self {
        Point3d::new(0.0, 0.0, 0.0)
    }
}

#[derive(Debug)]
pub struct IntYPoint3d {
    pub x: f64,
    pub y: i16,
    pub z: f64,
}

impl From<Point3d> for IntYPoint3d {
    fn from(point: Point3d) -> Self {
        Self {
            x: point.x,
            y: f64::round(point.y) as i16,
            z: point.z,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_pts(p1: &Point3d, p2: &Point3d) -> Self {
        Self {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
            z: p2.z - p1.z,
        }
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn len(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    #[allow(dead_code)]
    pub fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn scalar_mul(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Default for Vec3d {
    fn default() -> Self {
        Vec3d::new(0.0, 0.0, 0.0)
    }
}

pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone)]
pub struct Matrix4 {
    data: [[f64; 4]; 4],
}

impl Matrix4 {
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            data: [[0.0; 4]; 4],
        }
    }
}

impl From<[[f64; 4]; 4]> for Matrix4 {
    fn from(data: [[f64; 4]; 4]) -> Self {
        Self { data }
    }
}

impl MulAssign<Matrix4> for Matrix4 {
    fn mul_assign(&mut self, rhs: Matrix4) {
        let mut res = [[0_f64; 4]; 4];

        for (i, row) in res.iter_mut().enumerate() {
            for (j, elem) in row.iter_mut().enumerate() {
                for (k, lhs_k) in self.data[i].iter().enumerate() {
                    *elem += lhs_k * rhs.data[k][j];
                }
            }
        }

        self.data = res;
    }
}

pub trait Transformator {
    fn mov(&mut self, val: f64, axis: Axis);
    fn rotate(&mut self, angle: f64, axis: Axis);
    fn scale(&mut self, val: f64);
    fn apply_to_point(&self, point: &mut Point3d);
}

pub trait CenterTransformator: Transformator {
    fn rotate_center(&mut self, angle: f64, axis: Axis);
    fn scale_center(&mut self, val: f64);
}

impl Transformator for Matrix4 {
    fn apply_to_point(&self, point: &mut Point3d) {
        let old_coords = [point.x, point.y, point.z, 1_f64];
        let mut new_coords = [0_f64; 3];
        for (i, nc) in new_coords.iter_mut().enumerate() {
            for (j, oc) in old_coords.iter().enumerate() {
                *nc += *oc * unsafe { self.data.get_unchecked(j).get_unchecked(i) };
            }
        }

        point.x = new_coords[0];
        point.y = new_coords[1];
        point.z = new_coords[2];
    }

    fn mov(&mut self, val: f64, axis: Axis) {
        match axis {
            Axis::X => self.data[3][0] += val,
            Axis::Y => self.data[3][1] += val,
            Axis::Z => self.data[3][2] += val,
        }
    }

    fn rotate(&mut self, angle: f64, axis: Axis) {
        let rhs = match axis {
            Axis::Y => Matrix4::from([
                [f64::cos(angle), 0_f64, f64::sin(angle), 0_f64],
                [0_f64, 1_f64, 0_f64, 0_f64],
                [-f64::sin(angle), 0_f64, f64::cos(angle), 0_f64],
                [0_f64, 0_f64, 0_f64, 1_f64],
            ]),
            Axis::X => Matrix4::from([
                [1_f64, 0_f64, 0_f64, 0_f64],
                [0_f64, f64::cos(angle), -f64::sin(angle), 0_f64],
                [0_f64, f64::sin(angle), f64::cos(angle), 0_f64],
                [0_f64, 0_f64, 0_f64, 1_f64],
            ]),
            Axis::Z => Matrix4::from([
                [f64::cos(angle), -f64::sin(angle), 0_f64, 0_f64],
                [f64::sin(angle), f64::cos(angle), 0_f64, 0_f64],
                [0_f64, 0_f64, 1_f64, 0_f64],
                [0_f64, 0_f64, 0_f64, 0_f64],
            ]),
        };

        *self *= rhs;
    }

    fn scale(&mut self, val: f64) {
        let scale_matrix = Matrix4::from([
            [val, 0_f64, 0_f64, 0_f64],
            [0_f64, val, 0_f64, 0_f64],
            [0_f64, 0_f64, val, 0_f64],
            [0_f64, 0_f64, 0_f64, 1_f64],
        ]);

        *self *= scale_matrix;
    }
}

impl CenterTransformator for Matrix4 {
    fn rotate_center(&mut self, angle: f64, axis: Axis) {
        let pos = [self.data[3][0], self.data[3][1], self.data[3][2]];
        for elem in self.data[3][0..2].iter_mut() {
            *elem = 0_f64;
        }

        self.rotate(angle, axis);

        for (elem, copy) in self.data[3][0..2].iter_mut().zip(pos.iter()) {
            *elem = *copy;
        }
    }

    fn scale_center(&mut self, val: f64) {
        let pos = [self.data[3][0], self.data[3][1], self.data[3][2]];
        for elem in self.data[3][0..2].iter_mut() {
            *elem = 0_f64;
        }

        self.scale(val);

        for (elem, copy) in self.data[3][0..2].iter_mut().zip(pos.iter()) {
            *elem = *copy;
        }
    }
}

pub struct Section {
    pub y_start: i16,
    pub y_end: i16,
    pub x_start: f64,
    pub x_step: f64,
    pub z_start: f64,
    pub z_step: f64,
    pub br_start: f64,
    pub br_step: f64,
}

impl Section {
    pub fn new(from: &IntYPoint3d, to: &IntYPoint3d, from_br: f64, to_br: f64) -> Self {
        let diff_y = to.y - from.y;
        Self {
            y_start: from.y,
            y_end: to.y,
            x_start: from.x,
            z_start: from.z,
            x_step: (to.x - from.x) / diff_y as f64,
            z_step: (to.z - from.z) / diff_y as f64,
            br_start: from_br,
            br_step: (to_br - from_br) / diff_y as f64,
        }
    }
}
