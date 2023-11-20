use crate::Any;

pub use crate::generated::conv::*;

/// Method for ROMAN()
pub enum RomanStyle {
    /// Only subtract powers of 10, not L or V, and only if the next
    /// number is not more than 10 times greater. A number
    /// following the larger one shall be smaller than the
    /// subtracted number. Also known as classic.
    Classic,
    /// Powers of 10, and L and V may be subtracted, only if the
    /// next number is not more than 10 times greater. A number
    /// following the larger one shall be smaller than the subtracted number.
    SubLXV,
    /// Powers of 10 and L, but not V, may be subtracted, also if
    /// the next number is more than 10 times greater. A number
    /// following the larger one shall be smaller than the subtracted number.
    SubLX,
    /// Powers of 10, and L and V may be subtracted, also if the
    /// next number is more than 10 times greater. A number following the larger
    /// one shall be smaller than the subtracted number.
    SubAllLXV,
    /// Produce the fewest Roman digits possible. Also known as
    /// simplified.
    Simplified,
}

impl Any for RomanStyle {
    fn formula(&self, buf: &mut String) {
        buf.push_str(match self {
            RomanStyle::Classic => "0",
            RomanStyle::SubLXV => "1",
            RomanStyle::SubLX => "2",
            RomanStyle::SubAllLXV => "3",
            RomanStyle::Simplified => "4",
        });
    }
}
