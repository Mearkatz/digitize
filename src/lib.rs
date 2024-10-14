//! Traits for iterating over the digits of any primitive integer or float

use default_impl::default_impl;
use traits::{Digits, FloatDigits};

pub mod traits;

/// Generates an implementation of Digits for `i*` using the corresponding `u*`'s implementation
macro_rules! piggyback {
    ($($x: ty),*) => {
        $(
            impl Digits for $x {
                fn digits(&self) -> Box<[u8]> {
                    self.unsigned_abs().digits()
                }
            }
        )*
    };
}

/// Unsafely casts the characters in `x`'s string representation to u32's.
/// # Safety
/// Only use if `x`'s String representation only contains only digits in base 10
#[inline(always)]
unsafe fn stringable_to_digits<T: ToString>(x: T) -> Box<[u8]> {
    x.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|x| x as u8)
        .collect()
}

default_impl!(Digits, u8, u16, u32, u64, u128, usize);
piggyback!(i8, i16, i32, i64, i128, isize);

impl FloatDigits for f64 {
    fn digits_left_of_dot(&self) -> Box<[u8]> {
        if self.is_finite() {
            unsafe { stringable_to_digits(self.trunc()) }
        } else {
            Box::new([]) // Infinity & NaN have no digits
        }
    }

    fn digits_right_of_dot(&self) -> Box<[u8]> {
        if self.is_finite() {
            unsafe { stringable_to_digits(self.fract()) }
        } else {
            Box::new([]) // Infinity & NaN have no digits
        }
    }
}
