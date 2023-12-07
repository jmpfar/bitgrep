use crate::{types::{bit_type::BitType, compare::Compare}};

use super::{and::And, equal::Equal,max::Max, min::Min};

/// Filters a result according to configuration
pub trait Filter<T> {
    /// Should output the result
    /// true to include, false to exclude.
    fn include(&self, result: T) -> bool;

    fn include_unwrap(&self, result: Option<T>) -> bool {
        if result.is_none() {
            return false;
        }

        return self.include(result.unwrap());
    }
}

pub fn create_filters<T>(
    minimum: Option<T>,
    maximum: Option<T>,
    literal: Option<T>,
) -> Box<dyn Filter<T>>
where
    // static cause T is owned due to being a native type
    T: Compare + 'static
{
    if let Some(unwrapped) = literal {
        return Equal::with_box(unwrapped);
    }

    return create_min_max_filters(minimum, maximum);
}

fn create_min_max_filters<T>(minimum: Option<T>, maximum: Option<T>) -> Box<dyn Filter<T>>
where
    // static cause T is owned due to being a native type
    T: BitType + 'static,
{
    let mut filters: Vec<Box<dyn Filter<T>>> = vec![];
    if let Some(min) = minimum {
        filters.push(Min::with_box(min));
    }

    if let Some(max) = maximum {
        filters.push(Max::with_box(max));
    }

    if filters.len() == 1 {
        return filters.remove(0);
    }

    return Box::new(And::with_filters(filters));
}
