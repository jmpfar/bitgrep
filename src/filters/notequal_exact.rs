use super::filter::Filter;
use crate::types::compare::Compare;

/// Implements a non equal filter.
/// Includes elements that are not equal to a literal.
///
/// In floats does *not* use approximate equal, this is mainly to detect 0
/// will add a more generic implementation later on.
pub(super) struct NotEqualExact<T> {
    literal: T,
}

impl<T: Compare> Filter<T> for NotEqualExact<T> {
    fn include(&self, result: T) -> bool {
        return result != self.literal;
    }
}

impl<T> NotEqualExact<T> {
    #[must_use]
    pub fn new(literal: T) -> Self {
        return NotEqualExact { literal };
    }

    #[must_use]
    pub fn with_box(literal: T) -> Box<Self> {
        return Box::new(Self::new(literal));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_equal_ints_returns_false() {
        assert!(!NotEqualExact::new(32i32).include(32i32));
        assert!(!NotEqualExact::new(-3i8).include(-3));
        assert!(!NotEqualExact::new(0u8).include(0));
    }

    #[test]
    fn filter_equal_floats_returns_false() {
        assert!(!NotEqualExact::<f64>::new(-0.0).include(0.0));
        assert!(!NotEqualExact::<f64>::new(-10.1).include(-10.1));
        assert!(!NotEqualExact::<f32>::new(0.0).include(0.0));
        assert!(!NotEqualExact::<f32>::new(31.3).include(31.3));
    }

    #[test]
    fn filter_notequal_ints_returns_true() {
        assert!(NotEqualExact::new(31i32).include(34i32));
        assert!(NotEqualExact::new(41u8).include(40u8));
        assert!(NotEqualExact::new(0i64).include(2i64));
    }

    #[test]
    fn filter_notequal_floats_returns_true() {
        assert!(NotEqualExact::<f32>::new(0.0).include(32.445));
        assert!(NotEqualExact::<f64>::new(0.0 + f64::EPSILON).include(0.0));
        assert!(NotEqualExact::<f64>::new(10.0).include(32.445));
        assert!(NotEqualExact::<f32>::new(-33.0).include(0.0));
    }
}
