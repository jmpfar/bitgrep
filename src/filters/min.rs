use super::filter::Filter;

/// Implements an inclusive minimum filter
/// Filters values < min
pub(super) struct Min<T> {
    min: T,
}

impl<T: std::cmp::PartialOrd> Filter<T> for Min<T> {
    fn include(&self, result: T) -> bool {
        return self.min <= result;
    }
}

impl<T> Min<T> {
    pub fn new(min: T) -> Min<T> {
        return Min { min };
    }

    pub fn with_box(min: T) -> Box<Min<T>> {
        return Box::new(Self::new(min));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_smaller_returns_false() {
        assert!(!Min::new(32.445).include(32.0));
        assert!(!Min::new(32).include(31));
        assert!(!Min::new(40u8).include(0));
    }

    #[test]
    fn min_equal_returns_true() {
        assert!(Min::new(32.445).include(32.445));
        assert!(Min::new(31).include(31));
        assert!(Min::new(40u8).include(40u8));
    }

    #[test]
    fn min_greater_returns_false() {
        assert!(Min::new(30.445).include(32.0));
        assert!(Min::new(30).include(31));
        assert!(Min::new(40u8).include(44));
    }
}
