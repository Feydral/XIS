#![allow(dead_code)]

use crate::math::numerics::uint2::UInt2;

/// Returns the xy coordinate of a specific index in an imaginary 2D canvas.
#[inline]
pub fn index_to_xy(index: u32, width: u32, height: u32) -> UInt2 {
    if index > width * height {
        return UInt2::new(0, 0);
    }
    let x = index % width;
    let y = index / width;
    UInt2::new(x, y)
}

/// Returns the index of a specific xy coordinate in an imaginary 2D canvas.
#[inline]
pub fn xy_to_index(x: u32, y: u32, width: u32, height: u32) -> u32 {
    if x > width || y > height {
        return 0;
    }
    y * width + x
}

/// Turns a bool into an u32: false -> 0, true -> 1.
#[inline]
pub fn bool_to_int(b: bool) -> u32 {
    if b { 1 } else { 0 }
}

/// Clamps an u32 between 0 and 1 (returns 0 if a == 0, returns 1 if a != 0).
#[inline]
pub fn clamp01(value: u32) -> u32 {
    value.max(0).min(1)
}

/// Turns a u64 into a binary String.
#[inline]
pub fn int_to_binary_string(value: u64, len: usize) -> String {
    format!("{:0width$b}", value, width = len)
}

/// Turns a u64 into a hexadecimal String.
#[inline]
pub fn int_to_hexadecimal_string(value: u64, len: usize) -> String {
    format!("{:0width$x}", value, width = len)
}

/// Turns a hexadecimal String into a binary String.
#[inline]
pub fn hexadecimal_string_to_binary_string(hex: &str, len: usize) -> Result<String, std::num::ParseIntError> {
    let value = u64::from_str_radix(hex, 16)?;
    Ok(format!("{:0width$b}", value, width = len))
}

/// Turns a binary String into a hexadecimal String.
#[inline]
pub fn binary_string_to_hexadecimal_string(bin: &str, len: usize) -> Result<String, std::num::ParseIntError> {
    let value = u64::from_str_radix(bin, 2)?;
    Ok(format!("{:0width$x}", value, width = len))
}