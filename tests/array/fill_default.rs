#[cfg(test)]
mod tests {
    use pencil_box::array::fill_default::fill_default;

    /// Tests filling a vector of `i32` with the default value `0`.
    ///
    /// # Expected
    /// Returns a vector of `0`s of the specified size.
    #[test]
    fn test_scalar_i32() {
        let values = fill_default::<i32>(5);
        assert_eq!(values, vec![0, 0, 0, 0, 0]);
    }

    /// Tests filling a vector of `bool` with the default value `false`.
    ///
    /// # Expected
    /// Returns a vector of `false` of the specified size.
    #[test]
    fn test_scalar_bool() {
        let values = fill_default::<bool>(3);
        assert_eq!(values, vec![false, false, false]);
    }

    /// Tests filling a vector of `String` with the default empty string.
    ///
    /// # Expected
    /// All strings in the vector should be empty.
    #[test]
    fn test_complex_string() {
        let values = fill_default::<String>(4);
        assert_eq!(values, vec!["".to_string(); 4]);
    }

    /// Tests filling a vector of `Vec<u8>` with empty vectors.
    ///
    /// # Expected
    /// Each element is an empty `Vec<u8>`.
    #[test]
    fn test_complex_vec_u8() {
        let values = fill_default::<Vec<u8>>(2);
        assert_eq!(values, vec![vec![], vec![]]);
    }

    /// Custom enum used to verify default behavior on user-defined types.
    #[derive(Debug, PartialEq, Clone)]
    enum Status {
        Ready,
        Error(String),
    }

    impl Default for Status {
        fn default() -> Self {
            Status::Ready
        }
    }

    /// Tests filling a vector with a custom enum that has a `Default` implementation.
    ///
    /// # Expected
    /// All elements are `Status::Ready`.
    #[test]
    fn test_enum_default() {
        let values = fill_default::<Status>(3);
        assert_eq!(
            values,
            vec![Status::Ready, Status::Ready, Status::Ready]
        );
    }

    /// Tests creating a vector of zero length.
    ///
    /// # Expected
    /// Returns an empty vector.
    #[test]
    fn test_zero_length() {
        let values = fill_default::<u64>(0);
        assert!(values.is_empty());
    }
}
