use std::collections::VecDeque;

/// Ring buffer using a limited size VecDeque
/// Could probably do fancy u8 specific optimizations, such as multiple values push/pop
pub(crate) struct RingBuffer<T> {
    buffer: VecDeque<T>,
    max_size: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(max_size: usize) -> Self {
        assert!(max_size > 0, "max_size must greater than zero");

        RingBuffer {
            buffer: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push_front(&mut self, value: T) -> Option<T> {
        let removed = if self.is_full() {
            self.pop_back()
        } else {
            None
        };

        self.buffer.push_front(value);
        return removed;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        return self.buffer.pop_back();
    }

    pub fn is_full(&self) -> bool {
        return self.len() >= self.max_size;
    }

    pub fn len(&self) -> usize {
        return self.buffer.len();
    }
}

#[cfg(test)]
mod tests {
    use assertor::{assert_that, BooleanAssertion, EqualityAssertion, OptionAssertion};

    use super::*;

    #[test]
    fn len_returns_length() {
        let mut buf = RingBuffer::new(2);
        assert_that!(buf.len()).is_equal_to(0);

        buf.push_front(1);
        assert_that!(buf.len()).is_equal_to(1);

        buf.push_front(1);
        buf.push_front(1);
        assert_that!(buf.len()).is_equal_to(2);
    }

    #[test]
    fn is_full() {
        let mut buf = RingBuffer::new(1);
        assert_that!(buf.is_full()).is_false();

        buf.push_front(1);
        assert_that!(buf.is_full()).is_true();
    }

    #[test]
    fn push_front_exceeds_size_remove_existing() {
        let mut buf = RingBuffer::new(1);

        assert_that!(buf.push_front(1)).is_none();
        assert_that!(buf.push_front(2)).has_value(1);
        assert_that!(buf.push_front(3)).has_value(2);
    }

    #[test]
    fn push_front_pop_back() {
        let mut buf = RingBuffer::new(2);

        buf.push_front(1);
        buf.push_front(2);

        assert_that!(buf.pop_back()).has_value(1);
        assert_that!(buf.pop_back()).has_value(2);
    }

    #[test]
    fn pop_back_empty_return_none() {
        let mut buf: RingBuffer<i32> = RingBuffer::new(2);

        assert_that!(buf.pop_back()).is_none();
    }
}
