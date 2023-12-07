use approx::Ulps;

use super::bit_type::{BitType, Float};

pub trait Compare: BitType {
    fn equal(&self, other: &Self) -> bool;
}

#[inline]
fn numeric_equal<T: BitType>(first: &T, second: &T) -> bool {
    return first == second;
}

// TODO(danilan): Somehow get max_ulps as configuration
#[inline]
fn float_equal<T>(first: &T, second: &T) -> bool
where
    T: Float
{
    if first.is_nan() && second.is_nan() {
        return true
    }

    if first.is_neg_infinity() && second.is_neg_infinity() {
        return true;
    }

    return Ulps::default().max_ulps(4).eq(&first, &second);
}

impl Compare for f32 {
    #[inline]
    fn equal(&self, other: &Self) -> bool {
        return float_equal(self, other);
    }
}

impl Compare for f64 {
    #[inline]
    fn equal(&self, other: &Self) -> bool {
        return float_equal(self, other);
    }
}

impl Compare for i8 {
    #[inline]
    fn equal(&self, other: &Self) -> bool {
        return numeric_equal(self, other);
    }
}

impl Compare for i16 {
    #[inline]
    fn equal(&self, other: &Self) -> bool {
        return numeric_equal(self, other);
    }
}

impl Compare for i32 {
    #[inline]
    fn equal(&self, other: &Self) -> bool {
        return numeric_equal(self, other);
    }
}

impl Compare for i64 {
    #[inline]
    fn equal(&self, other: &Self) -> bool {
        return numeric_equal(self, other);
    }
}

impl Compare for i128 {
    #[inline]
    fn equal(&self, other: &Self) -> bool {
        return numeric_equal(self, other);
    }
}

impl Compare for u8 {
    #[inline]
    fn equal(&self, other: &Self) -> bool {
        return numeric_equal(self, other);
    }
}

impl Compare for u16 {
    #[inline]
    fn equal(&self, other: &Self) -> bool {
        return numeric_equal(self, other);
    }
}

impl Compare for u32 {
    #[inline]
    fn equal(&self, other: &Self) -> bool {
        return numeric_equal(self, other);
    }
}

impl Compare for u64 {
    #[inline]
    fn equal(&self, other: &Self) -> bool {
        return numeric_equal(self, other);
    }
}

impl Compare for u128 {
    #[inline]
    fn equal(&self, other: &Self) -> bool {
        return numeric_equal(self, other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_equal_f32_not_equal_returns_false() {
        assert!(!float_equal::<f32>(&32.445, &32.0));
        assert!(!float_equal::<f32>(&32.445, &f32::NAN));
        assert!(!float_equal::<f32>(&32.0, &34.0));
        assert!(!float_equal::<f32>(&0.0, &0.0001));

        assert!(!float_equal::<f32>(&f32::NAN, &f32::INFINITY));
        assert!(!float_equal::<f32>(&f32::NAN, &f32::NEG_INFINITY));

        assert!(!float_equal::<f32>(&f32::INFINITY, &f32::NEG_INFINITY));
        assert!(!float_equal::<f32>(&f32::NEG_INFINITY, &f32::INFINITY));

        assert!(!float_equal::<f32>(&0.0, &(f32::EPSILON * 2.0)));
    }

    #[test]
    fn float_equal_f64_not_equal_returns_false() {
        assert!(!float_equal::<f64>(&32.445, &32.0));
        assert!(!float_equal::<f64>(&32.445, &f64::NAN));
        assert!(!float_equal::<f64>(&32.0, &34.0));
        assert!(!float_equal::<f64>(&0.0, &0.0001));

        assert!(!float_equal::<f64>(&f64::NAN, &f64::INFINITY));
        assert!(!float_equal::<f64>(&f64::NAN, &f64::NEG_INFINITY));

        assert!(!float_equal::<f64>(&f64::INFINITY, &f64::NEG_INFINITY));
        assert!(!float_equal::<f64>(&f64::NEG_INFINITY, &f64::INFINITY));

        assert!(!float_equal::<f64>(&0.0, &(f64::EPSILON * 2.0)));
    }

    #[test]
    fn float_equal_f32_equal_returns_true() {
        assert!(float_equal::<f32>(&32.445, &32.445));
        assert!(float_equal::<f32>(&(32.445 + f32::EPSILON), &32.445));
        assert!(float_equal::<f32>(&(32.445 - f32::EPSILON), &32.445));

        assert!(float_equal::<f32>(&-0.0, &0.0));
        assert!(float_equal::<f32>(&0.0, &0.0));

        assert!(float_equal::<f32>(&f32::NAN, &f32::NAN));
        assert!(float_equal::<f32>(&f32::NAN, &-f32::NAN));
        assert!(float_equal::<f32>(&-f32::NAN, &-f32::NAN));
        assert!(float_equal::<f32>(&f32::NAN, &f32::NAN));
    }

    #[test]
    fn float_equal_f32_equal_zeroes_returns_true() {
        assert!(float_equal::<f32>(&-0.0, &0.0));
        assert!(float_equal::<f32>(&0.0, &0.0));
    }

    #[test]
    fn float_equal_f32_equal_special_returns_true() {
        assert!(float_equal::<f32>(&f32::NAN, &f32::NAN));
        assert!(float_equal::<f32>(&f32::NAN, &-f32::NAN));
        assert!(float_equal::<f32>(&-f32::NAN, &-f32::NAN));
        assert!(float_equal::<f32>(&f32::NAN, &-f32::NAN));

        assert!(float_equal::<f32>(&f32::INFINITY, &f32::INFINITY));
        assert!(float_equal::<f32>(&f32::NEG_INFINITY, &f32::NEG_INFINITY));
    }

    #[test]
    fn float_equal_f64_equal_returns_true() {
        assert!(float_equal::<f64>(&32.445, &32.445));
        assert!(float_equal::<f64>(&(32.445 + f64::EPSILON), &32.445));
        assert!(float_equal::<f64>(&(32.445 - f64::EPSILON), &32.445));

        assert!(float_equal::<f64>(&-0.0, &0.0));
        assert!(float_equal::<f64>(&0.0, &0.0));

        assert!(float_equal::<f64>(&f64::NAN, &f64::NAN));
        assert!(float_equal::<f64>(&f64::NAN, &-f64::NAN));
        assert!(float_equal::<f64>(&-f64::NAN, &-f64::NAN));
        assert!(float_equal::<f64>(&f64::NAN, &f64::NAN));
    }

    #[test]
    fn float_equal_f64_equal_zeroes_returns_true() {
        assert!(float_equal::<f64>(&-0.0, &0.0));
        assert!(float_equal::<f64>(&0.0, &0.0));
    }

    #[test]
    fn float_equal_f64_equal_special_returns_true() {
        assert!(float_equal::<f64>(&f64::NAN, &f64::NAN));
        assert!(float_equal::<f64>(&f64::NAN, &-f64::NAN));
        assert!(float_equal::<f64>(&-f64::NAN, &-f64::NAN));
        assert!(float_equal::<f64>(&f64::NAN, &-f64::NAN));

        assert!(float_equal::<f64>(&f64::INFINITY, &f64::INFINITY));
        assert!(float_equal::<f64>(&f64::NEG_INFINITY, &f64::NEG_INFINITY));
    }

    #[test]
    fn numeric_equal_not_equal_returns_false() {
        assert!(!numeric_equal::<u32>(&0, &3));
        assert!(!numeric_equal::<u8>(&60, &44));
        assert!(!numeric_equal::<i64>(&34, &-3));
    }

    #[test]
    fn numeric_equal_equal_returns_true() {
        assert!(numeric_equal::<i32>(&-31, &-31));
        assert!(numeric_equal::<u8>(&40, &40));
        assert!(numeric_equal::<i64>(&0, &0));
    }    
}