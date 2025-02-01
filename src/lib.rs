//! Traits for iterating over the digits of any primitive integer or float

use tinyvec::ArrayVec;

pub trait Digits {
    const MAX_NUMBER_OF_DIGITS: usize;
    type R;

    fn digits(self) -> Self::R;
}

pub trait FloatDigits {
    /// Digits left of the decimal point
    fn digits_left_of_dot(&self) -> Box<[u8]>;

    /// Digits right of the decimal point
    fn digits_right_of_dot(&self) -> Box<[u8]>;

    /// Digits left of the decimal point, and right of the decimal point in that order.
    fn digits_left_then_right_of_dot(&self) -> [Box<[u8]>; 2] {
        [self.digits_left_of_dot(), self.digits_right_of_dot()]
    }
}

macro_rules! first_power_of_two_gte {
    ($x: expr) => {
        if $x.is_power_of_two() {
            $x
        } else {
            $x.next_power_of_two()
        }
    };
}

#[macro_export]
macro_rules! impl_digits {
    ($t: ty, $max: expr, $digits_of_min: expr, $is_signed_type: expr) => {
        impl Digits for $t {
            const MAX_NUMBER_OF_DIGITS: usize = $max.ilog10() as usize + 1;

            type R = ArrayVec<[u8; first_power_of_two_gte!(Self::MAX_NUMBER_OF_DIGITS)]>;

            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                unused_comparisons
            )]
            fn digits(mut self) -> Self::R {
                let mut v = ArrayVec::new();

                if $is_signed_type {
                    if self == Self::MIN {
                        v.extend($digits_of_min.into_iter());
                        return v;
                    }
                    if self < 0 {
                        0 - self
                    } else {
                        self
                    };
                }

                while self > 10 {
                    v.push((self % 10) as u8);
                    self /= 10;
                }
                v.push((self % 10) as u8);
                v.reverse();
                v
            }
        }
    };
}

impl_digits!(u8, u8::MAX, [0], false);
impl_digits!(u16, u16::MAX, [0], false);
impl_digits!(u32, u32::MAX, [0], false);
impl_digits!(u64, u64::MAX, [0], false);
impl_digits!(u128, u128::MAX, [0], false);
impl_digits!(usize, usize::MAX, [0], false);

impl_digits!(i8, i8::MAX, [1, 2, 8], true);
impl_digits!(i16, i16::MAX, [3, 2, 7, 6, 8], true);
impl_digits!(i32, i32::MAX, [2, 1, 4, 7, 4, 8, 3, 6, 4, 8], true);
impl_digits!(
    i64,
    i64::MAX,
    [9, 2, 2, 3, 3, 7, 2, 0, 3, 6, 8, 5, 4, 7, 7, 5, 8, 0, 8],
    true
);
impl_digits!(
    i128,
    i128::MAX,
    [
        1, 7, 0, 1, 4, 1, 1, 8, 3, 4, 6, 0, 4, 6, 9, 2, 3, 1, 7, 3, 1, 6, 8, 7, 3, 0, 3, 7, 1, 5,
        8, 8, 4, 1, 0, 5, 7, 2, 8
    ],
    true
);
impl_digits!(
    isize,
    isize::MAX,
    [9, 2, 2, 3, 3, 7, 2, 0, 3, 6, 8, 5, 4, 7, 7, 5, 8, 0, 8],
    true
);

/// Unsafely casts the characters in `x`'s string representation to u32's.
/// # Safety
/// Only use if `x`'s String representation only contains digits in base 10
#[inline(always)]
unsafe fn stringable_to_digits<T: ToString>(x: T) -> Box<[u8]> {
    x.to_string()
        .chars()
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            _ => 9,
        })
        .collect()
}

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
