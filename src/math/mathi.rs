#![allow(dead_code)]

use crate::math::numerics::uint2::UInt2;

#[inline]
pub fn index_to_xy(index: u32, width: u32, height: u32) -> UInt2 {
    if index > width * height {
        return UInt2::new(0, 0);
    }
    let x = index % width;
    let y = index / width;
    UInt2::new(x, y)
}

#[inline]
pub fn xy_to_index(x: u32, y: u32, width: u32, height: u32) -> u32 {
    if x > width || y > height {
        return 0;
    }
    y * width + x
}

#[inline]
pub fn bool_to_int(b: bool) -> i32 {
    if b { 1 } else { 0 }
}

#[inline]
pub fn clamp01(a: u32) -> u32 {
    a.max(0).min(1)
}