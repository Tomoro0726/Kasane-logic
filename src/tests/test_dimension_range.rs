use crate::id::DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single};

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for DimensionRange Display trait
    #[test]
    fn test_dimension_range_display() {
        assert_eq!(
            Single(5u64).to_string(),
            "5"
        );
        assert_eq!(
            LimitRange(10u64, 20u64).to_string(),
            "10:20"
        );
        assert_eq!(
            BeforeUnLimitRange(15u64).to_string(),
            "-:15"
        );
        assert_eq!(
            AfterUnLimitRange(25u64).to_string(),
            "25:-"
        );
        assert_eq!(
            Any::<u64>.to_string(),
            "-"
        );
    }

    #[test]
    fn test_dimension_range_display_negative() {
        assert_eq!(
            Single(-5i64).to_string(),
            "-5"
        );
        assert_eq!(
            LimitRange(-10i64, -5i64).to_string(),
            "-10:-5"
        );
        assert_eq!(
            BeforeUnLimitRange(-1i64).to_string(),
            "-:-1"
        );
        assert_eq!(
            AfterUnLimitRange(-5i64).to_string(),
            "-5:-"
        );
    }

    #[test]
    fn test_dimension_range_equality() {
        assert_eq!(Single(5u64), Single(5u64));
        assert_eq!(
            LimitRange(1u64, 10u64),
            LimitRange(1u64, 10u64)
        );
        assert_eq!(Any::<u64>, Any::<u64>);
        
        assert_ne!(Single(5u64), Single(6u64));
        assert_ne!(
            LimitRange(1u64, 10u64),
            LimitRange(1u64, 11u64)
        );
    }

    #[test]
    fn test_dimension_range_copy_clone() {
        let range = Single(42u64);
        let copied = range;
        let cloned = range.clone();
        
        assert_eq!(range, copied);
        assert_eq!(range, cloned);
    }
}