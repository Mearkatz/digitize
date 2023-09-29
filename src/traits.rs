use std::fmt::Display;

use crate::stringable_to_digits;

pub trait Digits: Display {
    /// Returns the digits of a primitive integer type,
    /// like a u8, or i16.
    /// The sign is not included in the returned digits.
    fn digits(&self) -> Box<[u32]> {
        stringable_to_digits(self)
    }
}

pub trait FloatDigits {
    /// Digits left of the decimal point
    fn digits_left_of_dot(&self) -> Box<[u32]>;

    /// Digits right of the decimal point
    fn digits_right_of_dot(&self) -> Box<[u32]>;

    /// Digits left of the decimal point, and right of the decimal point in that order.
    fn digits_left_then_right_of_dot(&self) -> [Box<[u32]>; 2] {
        [self.digits_left_of_dot(), self.digits_right_of_dot()]
    }
}
