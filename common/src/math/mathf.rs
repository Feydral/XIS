#![allow(dead_code)]

use crate::math::numerics::{float2::Float2, float3::Float3, float4::Float4};

/// Tests if a point p is inside the triangle defined by points a, b, and c. 
/// If the point is inside the triangle, it also calculates the barycentric weights 
/// for each vertex of the triangle and returns them through the weight_a, weight_b, 
/// and weight_c parameters. The function returns true if the point is inside the triangle 
/// and false otherwise.
#[inline(always)]
pub fn point_in_triangle(a: Float2, b: Float2, c: Float2, p: Float2, weight_a: &mut f32, weight_b: &mut f32, weight_c: &mut f32) -> bool {
    let e0 = (b - a).cross(p - a);
    let e1 = (c - b).cross(p - b);
    let e2 = (a - c).cross(p - c);

    let has_neg = (e0 < 0.0) | (e1 < 0.0) | (e2 < 0.0);
    let has_pos = (e0 > 0.0) | (e1 > 0.0) | (e2 > 0.0);

    if has_neg & has_pos {
        return false;
    }

    let area = (b - a).cross(c - a);
    if area == 0.0 {
        return false;
    }

    let inv_area = 1.0 / area;

    *weight_a = e1 * inv_area;
    *weight_b = e2 * inv_area;
    *weight_c = e0 * inv_area;

    true
}

/// Converts a Float4 color to a u32 value representing an RGBA color.
#[inline(always)]
pub fn float4_to_u32_rgba(color: Float4) -> u32 {
    let r = (color.x.clamp(0.0, 1.0) * 255.0).round() as u32;
    let g = (color.y.clamp(0.0, 1.0) * 255.0).round() as u32;
    let b = (color.z.clamp(0.0, 1.0) * 255.0).round() as u32;
    let a = (color.w.clamp(0.0, 1.0) * 255.0).round() as u32;
    (r << 24) | (g << 16) | (b << 8) | a
}

/// Converts a u32 value representing an RGBA color to a Float4.
#[inline(always)]
pub fn u32_to_float4_rgba(value: u32) -> Float4 {
    let r = ((value >> 24) & 0xFF) as f32 / 255.0;
    let g = ((value >> 16) & 0xFF) as f32 / 255.0;
    let b = ((value >> 8) & 0xFF) as f32 / 255.0;
    let a = (value & 0xFF) as f32 / 255.0;
    Float4::new(r, g, b, a)
}

/// Converts a Float3 color to a u32 value representing an RGB color. The alpha channel is set to 255 (fully opaque).
#[inline(always)]
pub fn float3_to_u32_rgb(color: Float3) -> u32 {
    let r = (color.x.clamp(0.0, 1.0) * 255.0).round() as u32;
    let g = (color.y.clamp(0.0, 1.0) * 255.0).round() as u32;
    let b = (color.z.clamp(0.0, 1.0) * 255.0).round() as u32;
    (r << 24) | (g << 16) | (b << 8) | 0xFF
}

/// Converts a u32 value representing an RGB color to a Float3.
#[inline(always)]
pub fn u32_to_float3_rgb(value: u32) -> Float3 {
    let r = ((value >> 24) & 0xFF) as f32 / 255.0;
    let g = ((value >> 16) & 0xFF) as f32 / 255.0;
    let b = ((value >> 8) & 0xFF) as f32 / 255.0;
    Float3::new(r, g, b)
}

/// Rounds a floating-point number up to the nearest integer and returns it as an i32.
#[inline(always)]
pub fn round_to_int(a: f32) -> i32 {
    a.round() as i32
}

/// Returns the ceiling of a floating-point number as an integer.
#[inline(always)]
pub fn ceil_to_int(a: f32) -> i32 {
    a.ceil() as i32
}

/// Returns the floor of a floating-point number as an integer.
#[inline(always)]
pub fn floor_to_int(a: f32) -> i32 {
    a.floor() as i32
}

/// Performs linear interpolation between two f32 values based on a parameter t, which is clamped between 0.0 and 1.0.
#[inline(always)]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
	a + (b - a) * t.clamp(0.0, 1.0)
}

/// Performs linear interpolation between two Float2 values based on a parameter t, which is clamped between 0.0 and 1.0.
#[inline(always)]
pub fn lerp_float2(a: Float2, b: Float2, t: f32) -> Float2 {
	a + (b - a) * t.clamp(0.0, 1.0)
}

/// Performs linear interpolation between two Float3 values based on a parameter t, which is clamped between 0.0 and 1.0.
#[inline(always)]
pub fn lerp_float3(a: Float3, b: Float3, t: f32) -> Float3 {
	a + (b - a) * t.clamp(0.0, 1.0)
}