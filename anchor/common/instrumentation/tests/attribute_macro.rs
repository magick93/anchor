use instrumentation::*;

#[instrument(name = "add", unit = "ms", description = "Addition function")]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// #[instrument]
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_subtract() {
        let result = subtract(5, 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_add_negative() {
        let result = add(-2, -3);
        assert_eq!(result, -5);
    }

    #[test]
    fn test_subtract_negative() {
        let result = subtract(-5, -3);
        assert_eq!(result, -2);
    }

    #[test]
    fn test_add_zero() {
        let result = add(0, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_subtract_zero() {
        let result = subtract(0, 0);
        assert_eq!(result, 0);
    }
}
