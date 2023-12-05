use super::filter::Filter;

/// Implements an inclusive maximum filter
/// Filters values > max
pub(super) struct Max<T> {
    max: T,
}

impl<T: std::cmp::PartialOrd> Filter<T> for Max<T> {
    fn include(&self, result: T) -> bool {
        return result <= self.max;
    }
}

impl<T> Max<T> {
    pub fn new(max: T) -> Max<T> {
        return Max { max };
    }

    pub fn with_box(max: T) -> Box<Max<T>> {
        return Box::new(Self::new(max));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_greater_returns_false() {
        assert!(Max::new(32.445).include(32.0));
        assert!(Max::new(32).include(31));
        assert!(Max::new(40u8).include(0));
    }

    #[test]
    fn max_equal_returns_true() {
        assert!(Max::new(32.445).include(32.445));
        assert!(Max::new(31).include(31));
        assert!(Max::new(40u8).include(40u8));
    }

    #[test]
    fn max_smaller_returns_false() {
        assert!(!Max::new(30.445).include(32.0));
        assert!(!Max::new(30).include(31));
        assert!(!Max::new(40u8).include(44));
    }
}
