use super::filter::Filter;
use crate::types::compare::Compare;

/// Implements an equal filter.
/// Includes elements that are equal to a literal.
///
/// In floats uses ULPS of 4 by default.
/// See implementation in types/compare.rs
pub(super) struct Equal<T> {
    literal: T,
}

impl<T: Compare> Filter<T> for Equal<T> {
    fn include(&self, result: T) -> bool {
        return result.equal(&self.literal);
    }
}

impl<T> Equal<T> {
    #[must_use]
    pub fn new(literal: T) -> Self {
        return Equal { literal };
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
    fn filter_not_equal_returns_false() {
        assert!(!Equal::new(32i32).include(34i32));
        assert!(!Equal::new(40u8).include(0));
        assert!(!Equal::new(40u8).include(0));
    }

    #[test]
    fn filter_equal_returns_true() {
        assert!(Equal::new(31i32).include(31i32));
        assert!(Equal::new(40u8).include(40u8));
        assert!(Equal::new(0i64).include(0i64));
    }

    #[test]
    fn filter_float_not_equal_returns_false() {
        assert!(!Equal::<f32>::new(32.445).include(33.445));
        assert!(!Equal::<f64>::new(32.44567).include(32.445));
        assert!(!Equal::<f64>::new(32.44567).include(32.445));

        assert!(!Equal::<f32>::new(-0.0).include(0.00001));
        assert!(!Equal::<f64>::new(0.0).include(0.00001));

        assert!(!Equal::<f64>::new(f64::INFINITY).include(f64::NEG_INFINITY));
        assert!(!Equal::<f32>::new(f32::NEG_INFINITY).include(f32::INFINITY));
    }

    #[test]
    fn filter_float_equal_returns_true() {
        assert!(Equal::<f32>::new(32.445).include(32.445));
        assert!(Equal::<f64>::new(32.445 + f64::EPSILON).include(32.445));
        assert!(Equal::<f64>::new(32.445 - f64::EPSILON).include(32.445));

        assert!(Equal::<f32>::new(-0.0).include(0.0));
        assert!(Equal::<f64>::new(0.0).include(0.0));
    }

    #[test]
    fn filter_float_equal_zeroes_returns_true() {
        assert!(Equal::<f64>::new(-0.0).include(0.0));
        assert!(Equal::<f32>::new(0.0).include(0.0));
    }

    #[test]
    fn filter_float_equal_special_returns_true() {
        assert!(Equal::<f64>::new(f64::NAN).include(f64::NAN));
        assert!(Equal::<f32>::new(f32::NAN).include(-f32::NAN));
        assert!(Equal::<f64>::new(-f64::NAN).include(-f64::NAN));
        assert!(Equal::<f64>::new(f64::NAN).include(f64::NAN));

        assert!(Equal::<f64>::new(f64::INFINITY).include(f64::INFINITY));
        assert!(Equal::<f32>::new(f32::NEG_INFINITY).include(f32::NEG_INFINITY));
    }
}
