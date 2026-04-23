#![allow(dead_code)]
// This module defines matrix types and operations for 3D graphics transformations.

// built-in imports
use std::convert::From;
use std::ops::{Add, Sub, Mul, Div};

// third-party imports

// local imports


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

