//! Traits for iterating over the digits of any primitive integer or float

use traits::{Digits, FloatDigits};

pub mod traits;

/// Expands to the default implementation of a trait on all the types provided
macro_rules! default_impl {
    ($tr: ty,$($x: ty),*) => {
        $(
            impl $tr for $x {}
        )*
    };
}

/// Generates an implementation of Digits for `i*` using the corresponding `u*`'s implementation
macro_rules! piggyback {
    ($($x: ty),*) => {
        $(
            impl Digits for $x {
                fn digits(&self) -> Box<[u32]> {
                    self.unsigned_abs().digits()
                }
            }
        )*
    };
}

/// Unsafely casts the characters in `x`'s string representation to u32's.
/// Should only be used on unsigned or positive numeric types.
#[inline(always)]
pub fn stringable_to_digits<T: ToString>(x: T) -> Box<[u32]> {
    x.to_string()
        .chars()
        .map(|c| unsafe { c.to_digit(10).unwrap_unchecked() })
        .collect()
}

default_impl!(Digits, u8, u16, u32, u64, u128, usize);
piggyback!(i8, i16, i32, i64, i128, isize);

impl FloatDigits for f64 {
    fn digits_left_of_dot(&self) -> Box<[u32]> {
        if self.is_finite() {
            stringable_to_digits(self.trunc())
        } else {
            Box::new([]) // Infinity & NaN have no digits
        }
    }

    fn digits_right_of_dot(&self) -> Box<[u32]> {
        if self.is_finite() {
            stringable_to_digits(self.fract())
        } else {
            Box::new([]) // Infinity & NaN have no digits
        }
    }
}
