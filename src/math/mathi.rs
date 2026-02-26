#![allow(dead_code)]

use crate::math::numerics::uint2::UInt2;

#[doc = "Returns the xy coordinate of a specific index in an imaginary 2D canvas."]
#[inline]
pub fn index_to_xy(index: u32, width: u32, height: u32) -> UInt2 {
    if index > width * height {
        return UInt2::new(0, 0);
    }
    let x = index % width;
    let y = index / width;
    UInt2::new(x, y)
}

#[doc = "Returns the index of a specific xy coordinate in an imaginary 2D canvas."]
#[inline]
pub fn xy_to_index(x: u32, y: u32, width: u32, height: u32) -> u32 {
    if x > width || y > height {
        return 0;
    }
    y * width + x
}

#[doc = "Turns a bool into an u32: false -> 0, true -> 1."]
#[inline]
pub fn bool_to_int(b: bool) -> u32 {
    if b { 1 } else { 0 }
}

#[doc = "Clamps an u32 between 0 and 1 (returns 0 if a == 0, returns 1 if a != 0)."]
#[inline]
pub fn clamp01(a: u32) -> u32 {
    a.max(0).min(1)
}