use super::filter::Filter;
use std::{cell::RefCell, rc::Rc};

use crate::{
    types::{bit_type::BitType, compare::Compare},
    workers::entropy_processor::EntropyProducer,
};

type EntropyProducerRef = Rc<RefCell<dyn EntropyProducer>>;

/// Implements a max entropy filter
/// Used to ignore noise from compression/encryption
///
/// Gets the entropy data from a [`EntropyProducer`], which is
/// a processor that calculates entropy on the nearby data (4k)
/// using a sliding window.
///
/// Note: The [`EntropyProcessor.entropy()`] method is relatively compute heavy,
///       (compared to regular filters). This should be in the last part of an [`And`] filter
///       for short circuiting.
pub(super) struct Entropy {
    max_entropy: f64,
    producer: EntropyProducerRef,
}

impl<T: BitType> Filter<T> for Entropy {
    fn include(&self, _: T) -> bool {
        let nearby_entropy = self.producer.borrow().entropy();

        return nearby_entropy.is_some_and(|en| en <= self.max_entropy);
    }
}

impl Entropy {
    #[must_use]
    pub fn new(max_entropy: f64, entropy_producer: EntropyProducerRef) -> Self {
        return Entropy {
            max_entropy,
            producer: entropy_producer,
        };
    }

    #[must_use]
    pub fn with_box(max_entropy: f64, entropy_producer: EntropyProducerRef) -> Box<Self> {
        return Box::new(Self::new(max_entropy, entropy_producer));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeEntropyProducer(Option<f64>);

    impl EntropyProducer for FakeEntropyProducer {
        fn entropy(&self) -> Option<f64> {
            return self.0;
        }
    }

    #[test]
    fn include_none_returns_false() {
        let none_producer = Rc::new(RefCell::new(FakeEntropyProducer(None)));
        let dont_care = 0;

        assert!(!Entropy::new(5.0, none_producer).include(dont_care));
    }

    #[test]
    fn include_entropy_greater_than_max_returns_false() {
        let entropy_producer = Rc::new(RefCell::new(FakeEntropyProducer(Some(3.345678))));
        let dont_care = 0;

        assert!(!Entropy::new(1.0, entropy_producer.clone()).include(dont_care));
        assert!(!Entropy::new(2.0, entropy_producer.clone()).include(dont_care));
        assert!(!Entropy::new(3.0, entropy_producer.clone()).include(dont_care));
        assert!(!Entropy::new(3.34, entropy_producer.clone()).include(dont_care));
    }

    #[test]
    fn include_entropy_less_equal_than_max_returns_true() {
        let entropy_producer = Rc::new(RefCell::new(FakeEntropyProducer(Some(3.345678))));
        let dont_care = 0;

        assert!(Entropy::new(3.345678, entropy_producer.clone()).include(dont_care));
        assert!(Entropy::new(3.5, entropy_producer.clone()).include(dont_care));
        assert!(Entropy::new(4.0, entropy_producer.clone()).include(dont_care));
        assert!(Entropy::new(5.0, entropy_producer.clone()).include(dont_care));
    }
}
