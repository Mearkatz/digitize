//! Traits for iterating over the digits of any primitive integer or float

use traits::Digits;

pub mod traits;

impl Digits for u8 {}
impl Digits for u16 {}
impl Digits for u32 {}
impl Digits for u64 {}
impl Digits for u128 {}
impl Digits for usize {}

impl Digits for i8 {
    fn digits(&self) -> Box<[u32]> {
        self.unsigned_abs().digits()
    }
}
impl Digits for i16 {
    fn digits(&self) -> Box<[u32]> {
        self.unsigned_abs().digits()
    }
}
impl Digits for i32 {
    fn digits(&self) -> Box<[u32]> {
        self.unsigned_abs().digits()
    }
}
impl Digits for i64 {
    fn digits(&self) -> Box<[u32]> {
        self.unsigned_abs().digits()
    }
}
impl Digits for i128 {
    fn digits(&self) -> Box<[u32]> {
        self.unsigned_abs().digits()
    }
}
impl Digits for isize {
    fn digits(&self) -> Box<[u32]> {
        self.unsigned_abs().digits()
    }
}

pub trait FloatDigits {
    fn digits_left_of_dot(&self) -> Box<[u32]>;

    fn digits_right_of_dot(&self) -> Box<[u32]>;

    fn digits_left_then_right_of_dot(&self) -> [Box<[u32]>; 2] {
        [self.digits_left_of_dot(), self.digits_right_of_dot()]
    }
}

impl FloatDigits for f64 {
    fn digits_left_of_dot(&self) -> Box<[u32]> {
        if !self.is_finite() {
            [].into()
        } else {
            self.trunc()
                .to_string()
                .chars()
                .map(|c| unsafe { c.to_digit(10).unwrap_unchecked() })
                .collect()
        }
    }

    fn digits_right_of_dot(&self) -> Box<[u32]> {
        // Infinity and NaN don't have any digits in my opinion
        if !self.is_finite() {
            [].into()
        } else {
            self.fract()
                .to_string()
                .chars()
                .map(|c| unsafe { c.to_digit(10).unwrap_unchecked() })
                .collect()
        }
    }
}
