use std::ops::BitXorAssign;

use num_traits::Unsigned;

/// A trait for converting a type to a mask.
pub trait ConvertibleToMask<M>
where
    M: Unsigned + BitXorAssign,
{
    fn to_mask(&self) -> M;
}

impl ConvertibleToMask<u32> for char {
    /// Convert a character to a mask.
    ///
    /// When using [`u32`], only case-insensitive alphabet characters are
    /// supported; otherwise, some characters may not return unique masks
    /// due to the limited range of [`u32`].
    fn to_mask(&self) -> u32 {
        1 << ((*self as u32 & 31) - 1)
    }
}

impl ConvertibleToMask<u64> for char {
    /// Convert a character to a mask.
    ///
    /// When using [`u64`], all alphanumerics are supported case-sensitively.
    /// All other characters are treated as identical and return the same mask.
    fn to_mask(&self) -> u64 {
        1 << match self {
            'A'..='Z' => *self as u64 - 'A' as u64,
            'a'..='z' => *self as u64 - 'a' as u64 + 26,
            '0'..='9' => *self as u64 - '0' as u64 + 52,
            // This is a placeholder for unsupported characters.
            _ => 63,
        }
    }
}
