use std::fmt::Display;

pub trait Digits: Display {
    /// Returns the digits of a primitive integer type,
    /// like a u8, or i16.
    /// The sign is not included in the returned digits.
    fn digits(&self) -> Box<[u32]> {
        unsafe {
            self.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap_unchecked())
                .collect()
        }
    }
}
