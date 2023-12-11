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
