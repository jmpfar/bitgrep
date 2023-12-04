use super::filter::Filter;

type BoxedFilter<T> = Box<dyn Filter<T>>;

/// Boolean And filter
/// Values must match all of the included filters
pub(crate) struct And<T> {
    filters: Vec<BoxedFilter<T>>,
}

impl<T> Filter<T> for And<T>
where
    T: Copy,
{
    fn include(&self, result: T) -> bool {
        return self.filters.iter().all(|x| x.include(result));
    }
}

impl<T> And<T> {
    pub fn new() -> And<T> {
        return And {
            filters: Vec::new(),
        };
    }

    pub fn with_filters(filters: Vec<BoxedFilter<T>>) -> And<T> {
        return And { filters };
    }

    pub fn add(&mut self, filter: Box<dyn Filter<T>>) {
        // TODO(danilan): Consider moving boxing to this function
        self.filters.push(filter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EqualFilter(i32);

    impl Filter<i32> for EqualFilter {
        fn include(&self, result: i32) -> bool {
            return self.0 == result;
        }
    }

    #[test]
    fn and_true_false_returns_false() {
        let and = And::with_filters(vec![Box::new(EqualFilter(2)), Box::new(EqualFilter(3))]);

        assert!(!and.include(2));
        assert!(!and.include(3));
    }

    #[test]
    fn and_true_true_returns_true() {
        let and = And::with_filters(vec![Box::new(EqualFilter(3)), Box::new(EqualFilter(3))]);

        assert!(and.include(3));
    }

    #[test]
    fn and_false_false_returns_false() {
        let and = And::with_filters(vec![Box::new(EqualFilter(2)), Box::new(EqualFilter(1))]);

        assert!(!and.include(3));
    }

    #[test]
    fn and_add_method() {
        let mut and = And::new();
        and.add(Box::new(EqualFilter(2)));
        and.add(Box::new(EqualFilter(2)));
        and.add(Box::new(EqualFilter(2)));

        assert!(!and.include(3));
        assert!(and.include(2));
    }
}
