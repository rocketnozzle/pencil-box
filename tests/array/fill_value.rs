#[cfg(test)]
mod tests {
    use pencil_box::array::fill_value::fill_value;

    /// Tests filling a vector of `i32` with a specific value.
    ///
    /// # Expected
    /// Returns a vector containing repeated copies of `42`.
    #[test]
    fn test_scalar_i32() {
        let values = fill_value(&42, 4);
        assert_eq!(values, vec![42, 42, 42, 42]);
    }

    /// Tests filling a vector of `bool` with `true`.
    ///
    /// # Expected
    /// Returns a vector of `true` values.
    #[test]
    fn test_scalar_bool() {
        let values = fill_value(&true, 3);
        assert_eq!(values, vec![true, true, true]);
    }

    /// Tests filling a vector of `String` with a non-empty value.
    ///
    /// # Expected
    /// All elements are clones of the input string.
    #[test]
    fn test_complex_string() {
        let values = fill_value(&"hi".to_string(), 2);
        assert_eq!(values, vec!["hi".to_string(), "hi".to_string()]);
    }

    /// Tests filling a vector of `Vec<u8>` with a predefined list.
    ///
    /// # Expected
    /// Each element is a clone of the input list.
    #[test]
    fn test_complex_vec_u8() {
        let input = vec![1, 2];
        let values = fill_value(&input, 3);
        assert_eq!(values, vec![input.clone(), input.clone(), input.clone()]);
    }

    /// Custom enum used to verify cloning of complex variants.
    #[derive(Debug, PartialEq, Clone)]
    enum Status {
        Ready,
        Error(String),
    }

    /// Tests filling a vector of a user-defined enum with a non-default variant.
    ///
    /// # Expected
    /// All elements are clones of the same variant and value.
    #[test]
    fn test_enum_value() {
        let value = Status::Error("fail".to_string());
        let values = fill_value(&value, 2);
        assert_eq!(
            values,
            vec![
                Status::Error("fail".to_string()),
                Status::Error("fail".to_string())
            ]
        );
    }

    /// Tests creating a vector of zero length using a complex type.
    ///
    /// # Expected
    /// Returns an empty vector.
    #[test]
    fn test_zero_length() {
        let values = fill_value(&1234, 0);
        assert!(values.is_empty());
    }
}
