use std::{cell::RefCell, rc::Rc};

use crate::{
    types::{bit_type::BitType, compare::Compare},
    workers::entropy_processor::EntropyProducer,
};

use super::{and::And, entropy::Entropy, equal::Equal, filter::Filter, max::Max, min::Min};

#[derive(Default)]
pub struct Configuration<T: Compare + 'static> {
    pub minimum: Option<T>,
    pub maximum: Option<T>,
    pub literal: Option<T>,
    pub entropy: Option<EntropyConfig>,
}

type BoxedFilter<T> = Box<dyn Filter<T>>;

impl<T: Compare> Configuration<T> {

    #[allow(clippy::option_map_unit_fn)]
    pub fn create_filter(&self) -> Option<BoxedFilter<T>> {
        let mut filters: Vec<BoxedFilter<T>> = Vec::with_capacity(5);

        self.create_equal_filter().map(|f| filters.push(f));
        self.create_max_filter().map(|f| filters.push(f));
        self.create_min_filter().map(|f| filters.push(f));

        self.entropy
            .as_ref()
            .and_then(EntropyConfig::create_filter)
            .map(|f| filters.push(f));

        if filters.is_empty() {
            return None;
        }

        return Some(And::with_box(filters));
    }

    fn create_max_filter(&self) -> Option<BoxedFilter<T>> {
        if let Some(max) = self.maximum {
            return Some(Max::with_box(max));
        }

        return None;
    }

    fn create_min_filter(&self) -> Option<BoxedFilter<T>> {
        if let Some(min) = self.minimum {
            return Some(Min::with_box(min));
        }

        return None;
    }

    fn create_equal_filter(&self) -> Option<BoxedFilter<T>> {
        if let Some(literal) = self.literal {
            return Some(Equal::with_box(literal));
        }

        return None;
    }
}

pub struct EntropyConfig {
    pub max_entropy: f64,
    pub entropy_producer: Rc<RefCell<dyn EntropyProducer>>,
}

impl EntropyConfig {
    fn create_filter<T: BitType>(&self) -> Option<BoxedFilter<T>> {
        let boxed_filter = Entropy::with_box(self.max_entropy, self.entropy_producer.clone());
        return Some(boxed_filter as BoxedFilter<T>);
    }
}
