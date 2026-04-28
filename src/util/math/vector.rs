#![allow(dead_code)]
use std::ops;



pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}


impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2{
            x,
            y
        }
    }

    pub fn zero() -> Self {
        Vector2{
            x: 0.0,
            y: 0.0
        }
    }

    pub fn one() -> Self {
        Vector2{
            x: 1.0,
            y: 1.0
        }
    }

    pub fn up() -> Self {
        Vector2{
            x: 0.0,
            y: 1.0
        }
    }

    pub fn down() -> Self {
        Vector2{
            x: 0.0,
            y: -1.0
        }
    }

    pub fn left() -> Self {
        Vector2{
            x: -1.0,
            y: 0.0
        }
    }

    pub fn right() -> Self {
        Vector2{
            x: 1.0,
            y: 0.0
        }
    }

    pub fn cross(&self, other: &Vector2) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length != 0.0 {
            Vector2{
                x: self.x / length,
                y: self.y / length
            }
        } else {
            Vector2::zero()
        }
    }

    pub fn distance(&self, other: &Vector2) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

}

impl From<(f32, f32)> for Vector2 {
    fn from(tuple: (f32, f32)) -> Self {
        Vector2{
            x: tuple.0,
            y: tuple.1
        }
    }

}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2{
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }

}

impl ops::Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2{
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }

}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2{
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl ops::Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2{
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }

}

impl ops::Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        Vector2{
            x: self.x / rhs,
            y: self.y / rhs
        }
    }

}


impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self{
            x,
            y,
            z
        }
    }

    pub fn as_tuple(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    pub fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn as_str(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }

    pub fn zero() -> Self {
        Self{
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn one() -> Self {
        Self{
            x: 1.0,
            y: 1.0,
            z: 1.0
        }
    }

    pub fn up() -> Self {
        Self{
            x: 0.0,
            y: 0.0,
            z: 1.0
        }
    }

    pub fn down() -> Self {
        Self{
            x: 0.0,
            y: 0.0,
            z: -1.0
        }
    }

    pub fn left() -> Self {
        Self{
            x: -1.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn right() -> Self {
        Self{
            x: 1.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn forward() -> Self {
        Self{
            x: 0.0,
            y: -1.0,
            z: 0.0
        }
    }

    pub fn backward() -> Self {
        Self{
            x: 0.0,
            y: 1.0,
            z: 0.0
        }
    }

    /// Negates the vector, returning a new vector with all components inverted.
    pub fn negate(&self) -> Self {
        Self{
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }

    /// Computes the cross product of this vector with another vector, returning a new vector that is perpendicular to both.
    pub fn cross(q: &Vector3, p: &Vector3) -> Self {
        Self{
            x: q.y * p.z - q.z * p.y,
            y: q.z * p.x - q.x * p.z,
            z: q.x * p.y - q.y * p.x
        }
    }

    /// Computes the dot product of this vector with another vector, returning a scalar value.
    pub fn dot(q: &Vector3, p: &Vector3) -> f32 {
        q.x * p.x + q.y * p.y + q.z * p.z
    }
    /// Computes the length (magnitude) of the vector, returning a scalar value.
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt().abs()
    }

    /// Normalizes the vector, returning a new vector with a length of 1.
    /// If the original vector has a length of 0, returns a zero vector to avoid division by zero.
    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length != 0.0 {
            Self{
                x: self.x / length,
                y: self.y / length,
                z: self.z / length
            }
        } else {
            Self::zero()
        }

    }

    /// Computes the distance between this vector and another vector, returning a scalar value.
    pub fn distance(&self, other: &Vector3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(tuple: (f32, f32, f32)) -> Self {
        Vector3{
            x: tuple.0,
            y: tuple.1,
            z: tuple.2
        }
    }

}

impl From<(Vector2, f32)> for Vector3 {
    fn from(tuple: (Vector2, f32)) -> Self {
        Vector3{
            x: tuple.0.x,
            y: tuple.0.y,
            z: tuple.1
        }
    }

}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }

}

impl ops::Add<f32> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: f32) -> Self::Output {
        let rhs_f32 = rhs as f32;
        Vector3{
            x: self.x + rhs_f32,
            y: self.y + rhs_f32,
            z: self.z + rhs_f32
        }
    }

}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }

}

impl ops::Sub<f32> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: f32) -> Self::Output {
        let rhs_f32 = rhs as f32;
        Vector3{
            x: self.x - rhs_f32,
            y: self.y - rhs_f32,
            z: self.z - rhs_f32
        }
    }

}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }

}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }

}


impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self{
            x,
            y,
            z,
            w
        }
    }

    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    pub fn zero() -> Self {
        Self{
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0
        }
    }

    pub fn one() -> Self {
        Self{
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0
        }
    }


}

impl From<(f32, f32, f32, f32)> for Vector4 {
    fn from(tuple: (f32, f32, f32, f32)) -> Self {
        Vector4{
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
            w: tuple.3
        }
    }

}

impl From<(Vector2, f32, f32)> for Vector4 {
    fn from(tuple: (Vector2, f32, f32)) -> Self {
        Vector4{
            x: tuple.0.x,
            y: tuple.0.y,
            z: tuple.1,
            w: tuple.2
        }
    }

}

impl From<(Vector3, f32)> for Vector4 {
    fn from(tuple: (Vector3, f32)) -> Self {
        Vector4{
            x: tuple.0.x,
            y: tuple.0.y,
            z: tuple.0.z,
            w: tuple.1
        }
    }

}

impl ops::Add<Vector4> for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Vector4) -> Self::Output {
        Vector4{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }

}

impl ops::Sub<Vector4> for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Vector4) -> Self::Output {
        Vector4{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }

}

impl ops::Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector4{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }

}

impl ops::Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: f32) -> Self::Output {
        Vector4{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs
        }
    }

}

