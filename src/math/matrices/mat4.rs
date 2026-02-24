#![allow(dead_code)]

use std::ops::*;
use std::fmt;

use crate::math::numerics::float3::Float3;
use crate::math::numerics::float4::Float4;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat4 {
    pub rows: [Float4; 4],
}

impl Mat4 {
    pub const fn new(r0: Float4, r1: Float4, r2: Float4, r3: Float4) -> Self {
        Self { rows: [r0, r1, r2, r3] }
    }

    pub const fn from_array(m: [f32; 16]) -> Self {
        Self {
            rows: [
                Float4::new(m[0],  m[1],  m[2],  m[3]),
                Float4::new(m[4],  m[5],  m[6],  m[7]),
                Float4::new(m[8],  m[9],  m[10], m[11]),
                Float4::new(m[12], m[13], m[14], m[15]),
            ]
        }
    }

    pub const fn identity() -> Self {
        Self::new(
            Float4::new(1.0, 0.0, 0.0, 0.0),
            Float4::new(0.0, 1.0, 0.0, 0.0),
            Float4::new(0.0, 0.0, 1.0, 0.0),
            Float4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub const fn zero() -> Self {
        Self::new(Float4::ZERO, Float4::ZERO, Float4::ZERO, Float4::ZERO)
    }

    pub fn translation(t: Float3) -> Self {
        Self::new(
            Float4::new(1.0, 0.0, 0.0, t.x),
            Float4::new(0.0, 1.0, 0.0, t.y),
            Float4::new(0.0, 0.0, 1.0, t.z),
            Float4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn rotation(rot: Float3) -> Self {
        let (sx, cx) = rot.x.sin_cos();
        let (sy, cy) = rot.y.sin_cos();
        let (sz, cz) = rot.z.sin_cos();

        let rx = Self::new(
            Float4::new(1.0, 0.0, 0.0, 0.0),
            Float4::new(0.0, cx, -sx, 0.0),
            Float4::new(0.0, sx, cx, 0.0),
            Float4::new(0.0, 0.0, 0.0, 1.0),
        );

        let ry = Self::new(
            Float4::new(cy, 0.0, sy, 0.0),
            Float4::new(0.0, 1.0, 0.0, 0.0),
            Float4::new(-sy, 0.0, cy, 0.0),
            Float4::new(0.0, 0.0, 0.0, 1.0),
        );

        let rz = Self::new(
            Float4::new(cz, -sz, 0.0, 0.0),
            Float4::new(sz, cz, 0.0, 0.0),
            Float4::new(0.0, 0.0, 1.0, 0.0),
            Float4::new(0.0, 0.0, 0.0, 1.0),
        );

        rz * ry * rx
    }

    pub fn scale(s: Float3) -> Self {
        Self::new(
            Float4::new(s.x, 0.0, 0.0, 0.0),
            Float4::new(0.0, s.y, 0.0, 0.0),
            Float4::new(0.0, 0.0, s.z, 0.0),
            Float4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    #[inline(always)]
    pub fn transpose(self) -> Self {
        let m = self.rows;
        Self::new(
            Float4::new(m[0].x, m[1].x, m[2].x, m[3].x),
            Float4::new(m[0].y, m[1].y, m[2].y, m[3].y),
            Float4::new(m[0].z, m[1].z, m[2].z, m[3].z),
            Float4::new(m[0].w, m[1].w, m[2].w, m[3].w),
        )
    }

    #[inline(always)]
    pub fn col(self, i: usize) -> Float4 {
        let m = self.rows;
        match i {
            0 => Float4::new(m[0].x, m[1].x, m[2].x, m[3].x),
            1 => Float4::new(m[0].y, m[1].y, m[2].y, m[3].y),
            2 => Float4::new(m[0].z, m[1].z, m[2].z, m[3].z),
            3 => Float4::new(m[0].w, m[1].w, m[2].w, m[3].w),
            _ => panic!("column out of range"),
        }
    }

    #[inline(always)]
    pub fn mul_vec4(self, v: Float4) -> Float4 {
        Float4::new(
            self.rows[0].dot(v),
            self.rows[1].dot(v),
            self.rows[2].dot(v),
            self.rows[3].dot(v),
        )
    }

    #[inline(always)]
    pub fn mul_mat4(self, rhs: Self) -> Self {
        let rhs_t = rhs.transpose();

        Self::new(
            Float4::new(
                self.rows[0].dot(rhs_t.rows[0]),
                self.rows[0].dot(rhs_t.rows[1]),
                self.rows[0].dot(rhs_t.rows[2]),
                self.rows[0].dot(rhs_t.rows[3]),
            ),
            Float4::new(
                self.rows[1].dot(rhs_t.rows[0]),
                self.rows[1].dot(rhs_t.rows[1]),
                self.rows[1].dot(rhs_t.rows[2]),
                self.rows[1].dot(rhs_t.rows[3]),
            ),
            Float4::new(
                self.rows[2].dot(rhs_t.rows[0]),
                self.rows[2].dot(rhs_t.rows[1]),
                self.rows[2].dot(rhs_t.rows[2]),
                self.rows[2].dot(rhs_t.rows[3]),
            ),
            Float4::new(
                self.rows[3].dot(rhs_t.rows[0]),
                self.rows[3].dot(rhs_t.rows[1]),
                self.rows[3].dot(rhs_t.rows[2]),
                self.rows[3].dot(rhs_t.rows[3]),
            ),
        )
    }

    pub fn inverse(self) -> Option<Self> {
        let m = self.rows;
        let mut inv = [0.0f32; 16];
        let m = [
            m[0].x, m[0].y, m[0].z, m[0].w,
            m[1].x, m[1].y, m[1].z, m[1].w,
            m[2].x, m[2].y, m[2].z, m[2].w,
            m[3].x, m[3].y, m[3].z, m[3].w,
        ];

        inv[0] = m[5]  * m[10] * m[15] -
                 m[5]  * m[11] * m[14] -
                 m[9]  * m[6]  * m[15] +
                 m[9]  * m[7]  * m[14] +
                 m[13] * m[6]  * m[11] -
                 m[13] * m[7]  * m[10];

        inv[4] = -m[4]  * m[10] * m[15] +
                  m[4]  * m[11] * m[14] +
                  m[8]  * m[6]  * m[15] -
                  m[8]  * m[7]  * m[14] -
                  m[12] * m[6]  * m[11] +
                  m[12] * m[7]  * m[10];

        inv[8] = m[4]  * m[9] * m[15] -
                 m[4]  * m[11] * m[13] -
                 m[8]  * m[5] * m[15] +
                 m[8]  * m[7] * m[13] +
                 m[12] * m[5] * m[11] -
                 m[12] * m[7] * m[9];

        inv[12] = -m[4]  * m[9] * m[14] +
                   m[4]  * m[10] * m[13] +
                   m[8]  * m[5] * m[14] -
                   m[8]  * m[6] * m[13] -
                   m[12] * m[5] * m[10] +
                   m[12] * m[6] * m[9];

        inv[1] = -m[1]  * m[10] * m[15] +
                  m[1]  * m[11] * m[14] +
                  m[9]  * m[2] * m[15] -
                  m[9]  * m[3] * m[14] -
                  m[13] * m[2] * m[11] +
                  m[13] * m[3] * m[10];

        inv[5] = m[0]  * m[10] * m[15] -
                 m[0]  * m[11] * m[14] -
                 m[8]  * m[2] * m[15] +
                 m[8]  * m[3] * m[14] +
                 m[12] * m[2] * m[11] -
                 m[12] * m[3] * m[10];

        inv[9] = -m[0]  * m[9] * m[15] +
                  m[0]  * m[11] * m[13] +
                  m[8]  * m[1] * m[15] -
                  m[8]  * m[3] * m[13] -
                  m[12] * m[1] * m[11] +
                  m[12] * m[3] * m[9];

        inv[13] = m[0]  * m[9] * m[14] -
                  m[0]  * m[10] * m[13] -
                  m[8]  * m[1] * m[14] +
                  m[8]  * m[2] * m[13] +
                  m[12] * m[1] * m[10] -
                  m[12] * m[2] * m[9];

        inv[2] = m[1]  * m[6] * m[15] -
                 m[1]  * m[7] * m[14] -
                 m[5]  * m[2] * m[15] +
                 m[5]  * m[3] * m[14] +
                 m[13] * m[2] * m[7] -
                 m[13] * m[3] * m[6];

        inv[6] = -m[0]  * m[6] * m[15] +
                  m[0]  * m[7] * m[14] +
                  m[4]  * m[2] * m[15] -
                  m[4]  * m[3] * m[14] -
                  m[12] * m[2] * m[7] +
                  m[12] * m[3] * m[6];

        inv[10] = m[0]  * m[5] * m[15] -
                  m[0]  * m[7] * m[13] -
                  m[4]  * m[1] * m[15] +
                  m[4]  * m[3] * m[13] +
                  m[12] * m[1] * m[7] -
                  m[12] * m[3] * m[5];

        inv[14] = -m[0]  * m[5] * m[14] +
                   m[0]  * m[6] * m[13] +
                   m[4]  * m[1] * m[14] -
                   m[4]  * m[2] * m[13] -
                   m[12] * m[1] * m[6] +
                   m[12] * m[2] * m[5];

        inv[3] = -m[1] * m[6] * m[11] +
                  m[1] * m[7] * m[10] +
                  m[5] * m[2] * m[11] -
                  m[5] * m[3] * m[10] -
                  m[9] * m[2] * m[7] +
                  m[9] * m[3] * m[6];

        inv[7] = m[0] * m[6] * m[11] -
                 m[0] * m[7] * m[10] -
                 m[4] * m[2] * m[11] +
                 m[4] * m[3] * m[10] +
                 m[8] * m[2] * m[7] -
                 m[8] * m[3] * m[6];

        inv[11] = -m[0] * m[5] * m[11] +
                   m[0] * m[7] * m[9] +
                   m[4] * m[1] * m[11] -
                   m[4] * m[3] * m[9] -
                   m[8] * m[1] * m[7] +
                   m[8] * m[3] * m[5];

        inv[15] = m[0] * m[5] * m[10] -
                  m[0] * m[6] * m[9] -
                  m[4] * m[1] * m[10] +
                  m[4] * m[2] * m[9] +
                  m[8] * m[1] * m[6] -
                  m[8] * m[2] * m[5];

        let det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];

        if det == 0.0 {
            return None;
        }

        let inv_det = 1.0 / det;
        Some(Self::from_array([
            inv[0]*inv_det, inv[1]*inv_det, inv[2]*inv_det, inv[3]*inv_det,
            inv[4]*inv_det, inv[5]*inv_det, inv[6]*inv_det, inv[7]*inv_det,
            inv[8]*inv_det, inv[9]*inv_det, inv[10]*inv_det, inv[11]*inv_det,
            inv[12]*inv_det, inv[13]*inv_det, inv[14]*inv_det, inv[15]*inv_det,
        ]))
    }

    pub fn look_at(eye: Float3, target: Float3, up: Float3) -> Mat4 {
        let f = (target - eye).normalize();
        let r = f.cross(up).normalize();
        let u = r.cross(f);

        Mat4::new(
            Float4::new(r.x, u.x, -f.x, 0.0),
            Float4::new(r.y, u.y, -f.y, 0.0),
            Float4::new(r.z, u.z, -f.z, 0.0),
            Float4::new(-r.dot(eye), -u.dot(eye), f.dot(eye), 1.0),
        )
    }
}

impl Mul<Float4> for Mat4 {
    type Output = Float4;
    #[inline(always)]
    fn mul(self, rhs: Float4) -> Float4 {
        self.mul_vec4(rhs)
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    #[inline(always)]
    fn mul(self, rhs: Mat4) -> Mat4 {
        self.mul_mat4(rhs)
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..4 {
            let row = self.rows[r];
            writeln!(f, "[{:>8.4} {:>8.4} {:>8.4} {:>8.4}]", row.x, row.y, row.z, row.w)?;
        }
        Ok(())
    }
}
