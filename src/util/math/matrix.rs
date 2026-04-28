#![allow(dead_code)]
// This module defines matrix types and operations for 3D graphics transformations.

// built-in imports
use std::convert::From;
use std::ops::{Add, Div, Mul, MulAssign, Sub};
use std::result;


// third-party imports

// local imports

use crate::util::math::vector::{Vector3, Vector4};
use crate::util::math::quaternion::Quaternion;



pub struct Matrix2 {
    pub data: [[f32; 2]; 2],
}

impl Matrix2 {
    pub fn new(data: [[f32; 2]; 2]) -> Self {
        Self { data }
    }
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0],
                [0.0, 1.0],
            ],
        }
    }

}

impl Add<Matrix2> for Matrix2 {
    type Output = Self;

    fn add(self, rhs: Matrix2) -> Self::Output {
        let mut result = Self::identity();
        for i in 0..2 {
            for j in 0..2 {
                result.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        result
    }

}

impl Sub<Matrix2> for Matrix2 {
    type Output = Self;

    fn sub(self, rhs: Matrix2) -> Self::Output {
        let mut result = Self::identity();
        for i in 0..2 {
            for j in 0..2 {
                result.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        result
    }
}

impl Mul<f32> for Matrix2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = Self::identity();
        for i in 0..2 {
            for j in 0..2 {
                result.data[i][j] = self.data[i][j] * rhs;
            }
        }
        result
    }

}

impl Mul<Matrix2> for Matrix2 {
    type Output = Self;

    fn mul(self, rhs: Matrix2) -> Self::Output {
        let mut result = Self::identity();
        for i in 0..2 {
            for j in 0..2 {
                result.data[i][j] = self.data[i][0] * rhs.data[0][j]
                    + self.data[i][1] * rhs.data[1][j];
            }
        }
        result
    }
}

impl Div<f32> for Matrix2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = Self::identity();
        for i in 0..2 {
            for j in 0..2 {
                result.data[i][j] = self.data[i][j] / rhs;
            }
        }
        result
    }

}

impl Div<Matrix2> for Matrix2 {
    type Output = Self;

    fn div(self, rhs: Matrix2) -> Self::Output {
        let mut result = Self::identity();
        for i in 0..2 {
            for j in 0..2 {
                result.data[i][j] = self.data[i][0] / rhs.data[0][j]
                    + self.data[i][1] / rhs.data[1][j];
            }
        }
        result
    }
}


pub struct Matrix3 {
    pub data: [[f32; 3]; 3],
}

impl Matrix3 {
    pub fn new(data: [[f32; 3]; 3]) -> Self {
        Self { data }
    }
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }


}

#[derive(Copy, Clone)]
pub struct Matrix4 {
    pub data: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Self { data }
    }
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

    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr() as *const f32
    }

        pub fn as_slice(&self) -> &[f32] {
            unsafe {
                std::slice::from_raw_parts(self.as_ptr(), 16)
            }
        }

    pub fn transpose(&self) -> Self {
        let mut result = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[j][i];
            }
        }
        result
    }

    pub fn translate(mat4: Matrix4, trans: Vector3) -> Vector3{
        let mut result: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        result.x = mat4.data[0][3] + trans.x;
        result.y = mat4.data[1][3] + trans.y;
        result.z = mat4.data[2][3] + trans.z;
        result
    }

     pub fn rotate(&mut self, angle: f32, x: f32, y: f32, z: f32) {
        let mut result = Self::identity();
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;

        result.data[0][0] = t * x * x + c;
        result.data[0][1] = t * x * y - s * z;
        result.data[0][2] = t * x * z + s * y;

        result.data[1][0] = t * x * y + s * z;
        result.data[1][1] = t * y * y + c;
        result.data[1][2] = t * y * z - s * x;

        result.data[2][0] = t * x * z - s * y;
        result.data[2][1] = t * y * z + s * x;
        result.data[2][2] = t * z * z + c;
    }

     pub fn create_scale(x: f32, y: f32, z: f32) -> Matrix4{
        let mut result = Matrix4::identity();
        result.data[0][0] = x;
        result.data[1][1] = y;
        result.data[2][2] = z;
        result
    }

    pub fn create_scale_vector(scale: Vector3) -> Matrix4{
        Matrix4::create_scale(scale.x, scale.y, scale.z)
    }

   pub fn create_scale_uniform(scale: f32) -> Matrix4{
        Matrix4::create_scale(scale, scale, scale)
    }


    pub fn create_rotation_x(angle: f32) -> Matrix4 {
        let mut result = Matrix4::identity();
        let c = angle.cos();
        let s = angle.sin();

        result.data[1][1] = c;
        result.data[1][2] = -s;
        result.data[2][1] = s;
        result.data[2][2] = c;

        result
    }

    pub fn create_rotation_y(angle: f32) -> Matrix4 {
        let mut result = Matrix4::identity();
        let c = angle.cos();
        let s = angle.sin();

        result.data[0][0] = c;
        result.data[0][2] = s;
        result.data[2][0] = -s;
        result.data[2][2] = c;

        result
    }

    pub fn create_rotation_z(angle: f32) -> Matrix4 {
        let mut result = Matrix4::identity();
        let c = angle.cos();
        let s = angle.sin();

        result.data[0][0] = c;
        result.data[0][1] = -s;
        result.data[1][0] = s;
        result.data[1][1] = c;

        result
    }

    pub fn create_translation(translation: Vector3) -> Matrix4 {
        let mut result = Matrix4::identity();
        result.data[3][0] = translation.x;
        result.data[3][1] = translation.y;
        result.data[3][2] = translation.z;
        result
    }

    pub fn create_from_quaternion(q: Quaternion) -> Matrix4 {
        let mut result = Matrix4::identity();

        result.data[0][0] = 1.0 - 2.0 * q.y * q.y - 2.0 * q.z * q.z;
        result.data[0][1] = 2.0 * q.x * q.y - 2.0 * q.w * q.z;
        result.data[0][2] = 2.0 * q.x * q.z + 2.0 * q.w * q.y;

        result.data[1][0] = 2.0 * q.x * q.y - 2.0 * q.w * q.z;
        result.data[1][1] = 1.0 - 2.0 * q.x * q.x - 2.0 * q.z * q.z;
        result.data[1][2] = 2.0 * q.y * q.z + 2.0 * q.w * q.x;

        result.data[2][0] = 2.0 * q.x * q.z + 2.0 * q.w * q.y;
        result.data[2][1] = 2.0 * q.y * q.z - 2.0 * q.w * q.x;
        result.data[2][2] = 1.0 - 2.0 * q.x * q.x - 2.0 * q.y * q.y;

        result
    }

}

impl Add<Matrix4> for Matrix4 {
    type Output = Self;

    fn add(self, rhs: Matrix4) -> Self::Output {
        let mut result = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        result
    }
}

impl Sub<Matrix4> for Matrix4 {
    type Output = Self;

    fn sub(self, rhs: Matrix4) -> Self::Output {
        let mut result = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        result
    }
}

impl Mul<f32> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][j] * rhs;
            }
        }
        result
    }

}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        let x = self.data[0][0] * rhs.x + self.data[0][1] * rhs.y + self.data[0][2] * rhs.z + self.data[0][3];
        let y = self.data[1][0] * rhs.x + self.data[1][1] * rhs.y + self.data[1][2] * rhs.z + self.data[1][3];
        let z = self.data[2][0] * rhs.x + self.data[2][1] * rhs.y + self.data[2][2] * rhs.z + self.data[2][3];
        Vector3 { x, y, z }
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut result = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][0] * rhs.data[0][j]
                    + self.data[i][1] * rhs.data[1][j]
                    + self.data[i][2] * rhs.data[2][j]
                    + self.data[i][3] * rhs.data[3][j];
            }
        }
        result
    }
}

impl MulAssign<Matrix4> for Matrix4 {
    fn mul_assign(&mut self, rhs: Matrix4) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Matrix4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][j] / rhs;
            }
        }
        result
    }

}

