// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

#[derive(Debug, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(data: u16) -> Self {
        SaturatingU16 { value: data }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(data: u8) -> Self {
        SaturatingU16 { value: data as u16 }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(data: &u16) -> Self {
        SaturatingU16 { value: *data }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(data: &u8) -> Self {
        SaturatingU16 {
            value: *data as u16,
        }
    }
}

use std::ops::Add;

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            value: self.value.saturating_add(other.value),
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: u16) -> Self::Output {
        Self {
            value: self.value.saturating_add(other),
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &u16) -> Self::Output {
        Self {
            value: self.value.saturating_add(*other),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &SaturatingU16) -> Self::Output {
        Self {
            value: self.value.saturating_add((*other).value),
        }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

impl PartialEq<SaturatingU16> for u16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        *self == other.value
    }
}

impl PartialEq<SaturatingU16> for SaturatingU16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        *self == other.value
    }
}
