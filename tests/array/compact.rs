#[cfg(test)]
mod tests {
    use pencil_box::array::compact::compact;
    use pencil_box::array::compact::IsEmpty;

    // --- Direct IsEmpty Trait Implementation Tests ---

    #[test]
    fn test_string_is_empty_impl() {
        assert!("".to_string().is_empty());
        assert!(!"hello".to_string().is_empty());
    }

    #[test]
    fn test_str_is_empty_impl() {
        assert!("".is_empty());
        assert!(!"world".is_empty());
    }

    #[test]
    fn test_vec_is_empty_impl() {
        let v_empty: Vec<i32> = vec![];
        assert!(v_empty.is_empty());
        let v_full = vec![1, 2, 3];
        assert!(!v_full.is_empty());
        // A vec containing empty elements is NOT considered empty by Vec<T>::is_empty()
        // unless it has no elements.
        let v_with_empty_elements = vec![0, 0, 0];
        assert!(!v_with_empty_elements.is_empty());
    }

    #[test]
    fn test_bool_is_empty_impl() {
        assert!(false.is_empty());
        assert!(!true.is_empty());
    }

    #[test]
    fn test_option_is_empty_impl() {
        // Option is None
        assert!(None::<i32>.is_empty());

        // Option is Some(value) where value is empty
        assert!(Some(0).is_empty()); // i32 is empty
        assert!(Some(0.0).is_empty()); // f64 is empty
        assert!(Some("".to_string()).is_empty()); // String is empty
        assert!(Some("").is_empty()); // &str is empty
        assert!(Some(false).is_empty()); // bool is empty
        assert!(Some(None::<i32>).is_empty()); // nested Option is empty

        // Option is Some(value) where value is NOT empty
        assert!(!Some(1).is_empty());
        assert!(!Some(3.14).is_empty());
        assert!(!Some("hello".to_string()).is_empty());
        assert!(!Some("world").is_empty());
        assert!(!Some(true).is_empty());
        assert!(!Some(Some(1)).is_empty()); // nested Option is not empty
    }

    #[test]
    fn test_numeric_is_empty_impls() {
        // Integers
        assert!(0_i8.is_empty());
        assert!(!1_i8.is_empty());

        assert!(0_u8.is_empty());
        assert!(!1_u8.is_empty());

        assert!(0_isize.is_empty());
        assert!(!10_isize.is_empty());

        // Floats
        assert!(0.0_f32.is_empty());
        assert!(!1.0_f32.is_empty());
        assert!(!3.14_f64.is_empty());
    }

    // --- Compact Function Tests (Original and expanded) ---

    /// Tests `compact` with a vector of integers, including zeros.
    /// Expects zeros to be removed.
    #[test]
    fn test_compact_ints() {
        let mut v = vec![1, 0, 2, 3, 0, 4];
        compact(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    /// Tests `compact` with a vector of floats, including 0.0.
    /// Expects 0.0 to be removed.
    #[test]
    fn test_compact_floats() {
        let mut v = vec![1.1, 0.0, 2.2, 3.5, 0.0001, 4.5, 0.0];
        compact(&mut v);
        assert_eq!(v, vec![1.1, 2.2, 3.5, 0.0001, 4.5]);
    }

    /// Tests `compact` with a vector of string slices (`&str`), including empty strings.
    /// Expects empty string slices to be removed.
    #[test]
    fn test_compact_strings_refs() {
        let mut v = vec!["a", "", "b", "c", "", "d"];
        compact(&mut v);
        assert_eq!(v, vec!["a", "b", "c", "d"]);
    }

    /// Tests `compact` with a vector of `String` objects, including empty `String`s.
    /// Expects empty `String`s to be removed.
    #[test]
    fn test_compact_strings() {
        let mut values = vec![
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "d".to_string(),
        ];
        compact(&mut values);
        assert_eq!(
            values,
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ]
        );
    }

    /// Tests `compact` with a vector of booleans.
    /// Expects `false` values to be removed.
    #[test]
    fn test_compact_bools() {
        let mut v = vec![true, false, true, false, true, false];
        compact(&mut v);
        assert_eq!(v, vec![true, true, true]);
    }

    /// Tests `compact` with an initially empty vector of any type.
    /// Expects the vector to remain empty.
    #[test]
    fn test_compact_empty_vec() {
        let mut v: Vec<i32> = vec![];
        compact(&mut v);
        assert_eq!(v, vec![]);
    }

    /// Edge case: Tests `compact` with a vector containing only empty elements.
    /// Expects the vector to become completely empty.
    #[test]
    fn test_compact_all_empty_elements() {
        let mut v = vec![0, 0, 0, 0];
        compact(&mut v);
        assert_eq!(v, vec![]);

        let mut v_str = vec!["".to_string(), "".to_string()];
        compact(&mut v_str);
        assert_eq!(v_str, Vec::<String>::new());

        let mut v_bool = vec![false, false];
        compact(&mut v_bool);
        assert_eq!(v_bool, vec![]);
    }

    /// Edge case: Tests `compact` with a vector where no elements are empty.
    /// Expects the vector to remain unchanged.
    #[test]
    fn test_compact_no_empty_elements() {
        let mut v = vec![1, 2, 3, 4];
        compact(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4]);

        let mut v_str = vec!["hello".to_string(), "world".to_string()];
        compact(&mut v_str);
        assert_eq!(v_str, vec!["hello".to_string(), "world".to_string()]);
    }

    /// Tests `compact` with a vector of `Option<T>` values.
    /// Covers `None` and `Some(empty_value)` cases.
    #[test]
    fn test_compact_options() {
        let mut v = vec![
            Some(1),
            None,
            Some(0), // Inner integer is empty
            Some(2),
            None,
            Some(10),
            Some(0), // Inner boolean is empty
        ];
        compact(&mut v);
        assert_eq!(v, vec![Some(1), Some(2), Some(10)]);

        // Test with nested options
        let mut v_nested = vec![
            Some(Some(1)),
            Some(None), // Inner option is empty
            None,
            Some(Some(0)), // Inner value (i32) is empty
            Some(Some(2)),
        ];
        compact(&mut v_nested);
        assert_eq!(v_nested, vec![Some(Some(1)), Some(Some(2))]);
    }

    /// Test with `usize` and `isize` types in `compact`.
    #[test]
    fn test_compact_usize_isize() {
        let mut v_usize: Vec<usize> = vec![10, 0, 5, 0, 1];
        compact(&mut v_usize);
        assert_eq!(v_usize, vec![10, 5, 1]);

        let mut v_isize: Vec<isize> = vec![-1, 0, 2, -3, 0];
        compact(&mut v_isize);
        assert_eq!(v_isize, vec![-1, 2, -3]);
    }

    /// Test with `u8` and `i8` (smallest integer types) in `compact`.
    #[test]
    fn test_compact_small_ints() {
        let mut v_u8: Vec<u8> = vec![255, 0, 10, 0];
        compact(&mut v_u8);
        assert_eq!(v_u8, vec![255, 10]);

        let mut v_i8: Vec<i8> = vec![127, -128, 0, 5, 0];
        compact(&mut v_i8);
        assert_eq!(v_i8, vec![127, -128, 5]);
    }

    /// Test `compact` with a vector of `Vec<T>`
    /// As per the current `IsEmpty for Vec<T>` implementation, only truly empty vectors are removed.
    #[test]
    fn test_compact_vec_of_vecs() {
        let mut v = vec![
            vec![1, 2],
            vec![],     // This one is empty
            vec![0, 0], // Not empty by Vec<T>::is_empty() definition
            vec![3],
            vec![], // This one is empty
        ];
        compact(&mut v);
        assert_eq!(v, vec![vec![1, 2], vec![0, 0], vec![3]]);
    }
}
