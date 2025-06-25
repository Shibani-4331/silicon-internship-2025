use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2,3);
        // result is original value
        // 5 is expected value
        // 3rd is fallback message
        assert_eq!(result, 5);
        // assert_eq!(result, 5, "Result must be 5");
    }

    #[test]
    fn test_even() {
        let result = is_even(4);
        // assert!(result);
        // assert!(result, "4 should be even");
        // assert_eq!(result, true);
        assert_ne!(result, false);
    }
}