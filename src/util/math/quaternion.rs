/// Description
///
/// author: "Triley"
/// copyright: "Copyright (c) 2024 Triley. All rights reserved."
/// credits: ["Triley"]
/// maintainer: "Triley"
/// version: "0.1.0"


// Built-in imports


// Third-party imports


// Local imports

use crate::util::math::functions;
use crate::util::math::vector::Vector3;


const QUATERNION_IDENTITY: Quaternion = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };


pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }

    pub fn conjugate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        if len > 0.0 {
            let inv_len = 1.0 / len;
            self.x *= inv_len;
            self.y *= inv_len;
            self.z *= inv_len;
            self.w *= inv_len;
        }
    }

    pub fn normalized(q: Quaternion) -> Self {
        let mut result = q;
        result.normalize();
        result
    }

    pub fn lerp(q1: Quaternion, q2: Quaternion, t: f32) -> Self {
        let mut result = Quaternion::identity();
        result.x = functions::lerp(q1.x, q2.x, t);
        result.y = functions::lerp(q1.y, q2.y, t);
        result.z = functions::lerp(q1.z, q2.z, t);
        result.w = functions::lerp(q1.w, q2.w, t);
        result.normalize();
        result
    }

    pub fn dot(q1: &Quaternion, q2: &Quaternion) -> f32 {
        q1.x * q2.x + q1.y * q2.y + q1.z * q2.z + q1.w * q2.w
    }

    pub fn slerp(q1: Quaternion, q2: Quaternion, t: f32) -> Self {
        let mut result = Quaternion::identity();
        let mut cos_half_theta = Quaternion::dot(&q1, &q2);
        if cos_half_theta < 0.0 {
            cos_half_theta = -cos_half_theta;
            result.x = -q2.x;
            result.y = -q2.y;
            result.z = -q2.z;
            result.w = -q2.w;
        } else {
            result = q2;
        }

        if cos_half_theta >= 1.0 {
            return q1;
        }

        let sin_half_theta = (1.0 - cos_half_theta * cos_half_theta).sqrt();
        if sin_half_theta.abs() < 0.001 {
            return Quaternion::lerp(q1, result, t);
        }

        let half_theta = cos_half_theta.acos();
        let a = ((1.0 - t) * half_theta).sin() / sin_half_theta;
        let b = (t * half_theta).sin() / sin_half_theta;

        let mut final_result = Quaternion::identity();
        final_result.x = q1.x * a + result.x * b;
        final_result.y = q1.y * a + result.y * b;
        final_result.z = q1.z * a + result.z * b;
        final_result.w = q1.w * a + result.w * b;
        final_result.normalize();
        final_result
    }

    pub fn concat(q1: Quaternion, q2: Quaternion) -> Self {
        let mut result = Quaternion::identity();

        let qv1 = Vector3 { x: q1.x, y: q1.y, z: q1.z };
        let qv2 = Vector3 { x: q2.x, y: q2.y, z: q2.z };
        let new_vec = qv2 * q1.w + qv1 * q2.w + Vector3::cross(&qv1, &qv2);
        result.x = new_vec.x;
        result.y = new_vec.y;
        result.z = new_vec.z;
        result.w = q1.w * q2.w - Vector3::dot(&qv1, &qv2);

        result
    }
}