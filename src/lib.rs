/// 2つの数値を加算する。
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// 2つの数値を減算する。
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// 2つの数値を乗算する。
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// 2つの数値を除算する。
///
/// `b` が 0 の場合は `Err` を返す。
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("Division by zero".to_string());
    }
    Ok(a / b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert!((add(2.0, 3.0) - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_add_negative() {
        assert!((add(-1.0, 4.0) - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_add_zero() {
        assert!((add(0.0, 7.0) - 7.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_subtract_positive() {
        assert!((subtract(10.0, 3.0) - 7.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_subtract_result_negative() {
        assert!((subtract(3.0, 10.0) - (-7.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_multiply_positive() {
        assert!((multiply(4.0, 5.0) - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_multiply_zero() {
        assert!((multiply(100.0, 0.0) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_multiply_negative() {
        assert!((multiply(-3.0, 6.0) - (-18.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_divide_positive() {
        let result = divide(10.0, 2.0).unwrap();
        assert!((result - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_divide_fraction() {
        let result = divide(7.0, 2.0).unwrap();
        assert!((result - 3.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }
}
