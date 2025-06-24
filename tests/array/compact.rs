#[cfg(test)]
mod tests {
    use pencil_box::array::compact::compact;
    use pencil_box::array::compact::IsEmpty;

    // --- Direct IsEmpty Trait Implementation Tests ---

    /// Tests `IsEmpty` for owned `String` values.
    ///
    /// # Expected
    /// An empty string returns true, non-empty returns false.
    #[test]
    fn test_string_is_empty_impl() {
        assert!("".to_string().is_empty());
        assert!(!"hello".to_string().is_empty());
    }

    /// Tests `IsEmpty` for string slices (`&str`).
    ///
    /// # Expected
    /// An empty slice returns true, non-empty returns false.
    #[test]
    fn test_str_is_empty_impl() {
        assert!("".is_empty());
        assert!(!"world".is_empty());
    }

    /// Tests `IsEmpty` for `Vec<T>`, including empty and non-empty cases.
    ///
    /// # Expected
    /// A vector with zero elements is empty; with any elements, it's not.
    #[test]
    fn test_vec_is_empty_impl() {
        let v_empty: Vec<i32> = vec![];
        assert!(v_empty.is_empty());

        let v_full = vec![1, 2, 3];
        assert!(!v_full.is_empty());

        let v_with_empty_elements = vec![0, 0, 0];
        assert!(!v_with_empty_elements.is_empty());
    }

    /// Tests `IsEmpty` for `bool`.
    ///
    /// # Expected
    /// `false` is considered empty; `true` is not.
    #[test]
    fn test_bool_is_empty_impl() {
        assert!(false.is_empty());
        assert!(!true.is_empty());
    }

    /// Tests `IsEmpty` for `Option<T>` with empty, non-empty, and nested values.
    ///
    /// # Expected
    /// `None` and `Some(empty)` are empty. `Some(non-empty)` is not.
    #[test]
    fn test_option_is_empty_impl() {
        assert!(None::<i32>.is_empty());
        assert!(Some(0).is_empty());
        assert!(Some(0.0).is_empty());
        assert!(Some("".to_string()).is_empty());
        assert!(Some("").is_empty());
        assert!(Some(false).is_empty());
        assert!(Some(None::<i32>).is_empty());

        assert!(!Some(1).is_empty());
        assert!(!Some(3.14).is_empty());
        assert!(!Some("hello".to_string()).is_empty());
        assert!(!Some("world").is_empty());
        assert!(!Some(true).is_empty());
        assert!(!Some(Some(1)).is_empty());
    }

    /// Tests `IsEmpty` for various numeric types.
    ///
    /// # Expected
    /// `0` and `0.0` are considered empty, non-zero values are not.
    #[test]
    fn test_numeric_is_empty_impls() {
        assert!(0_i8.is_empty());
        assert!(!1_i8.is_empty());

        assert!(0_u8.is_empty());
        assert!(!1_u8.is_empty());

        assert!(0_isize.is_empty());
        assert!(!10_isize.is_empty());

        assert!(0.0_f32.is_empty());
        assert!(!1.0_f32.is_empty());
        assert!(!3.14_f64.is_empty());
    }

    // --- Compact Function Tests ---

    /// Tests `compact` on integers, including zeros.
    ///
    /// # Expected
    /// Removes all `0` values.
    #[test]
    fn test_compact_ints() {
        let mut v = vec![1, 0, 2, 3, 0, 4];
        compact(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    /// Tests `compact` on floats, including `0.0`.
    ///
    /// # Expected
    /// Removes all `0.0` values.
    #[test]
    fn test_compact_floats() {
        let mut v = vec![1.1, 0.0, 2.2, 3.5, 0.0001, 4.5, 0.0];
        compact(&mut v);
        assert_eq!(v, vec![1.1, 2.2, 3.5, 0.0001, 4.5]);
    }

    /// Tests `compact` on string slices.
    ///
    /// # Expected
    /// Removes all empty `&str` values.
    #[test]
    fn test_compact_strings_refs() {
        let mut v = vec!["a", "", "b", "c", "", "d"];
        compact(&mut v);
        assert_eq!(v, vec!["a", "b", "c", "d"]);
    }

    /// Tests `compact` on owned `String`s.
    ///
    /// # Expected
    /// Removes all empty strings.
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

    /// Tests `compact` on a vector of booleans.
    ///
    /// # Expected
    /// Removes all `false` values.
    #[test]
    fn test_compact_bools() {
        let mut v = vec![true, false, true, false, true, false];
        compact(&mut v);
        assert_eq!(v, vec![true, true, true]);
    }

    /// Tests `compact` on an initially empty vector.
    ///
    /// # Expected
    /// Leaves the vector unchanged.
    #[test]
    fn test_compact_empty_vec() {
        let mut v: Vec<i32> = vec![];
        compact(&mut v);
        assert_eq!(v, vec![]);
    }

    /// Tests `compact` when all elements are empty.
    ///
    /// # Expected
    /// Results in an empty vector.
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

    /// Tests `compact` when no elements are empty.
    ///
    /// # Expected
    /// Vector remains unchanged.
    #[test]
    fn test_compact_no_empty_elements() {
        let mut v = vec![1, 2, 3, 4];
        compact(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4]);

        let mut v_str = vec!["hello".to_string(), "world".to_string()];
        compact(&mut v_str);
        assert_eq!(v_str, vec!["hello".to_string(), "world".to_string()]);
    }

    /// Tests `compact` on a vector of `Option<T>`.
    ///
    /// # Expected
    /// Removes `None` and `Some(empty)` values.
    #[test]
    fn test_compact_options() {
        let mut v = vec![
            Some(1),
            None,
            Some(0),
            Some(2),
            None,
            Some(10),
            Some(0),
        ];
        compact(&mut v);
        assert_eq!(v, vec![Some(1), Some(2), Some(10)]);

        let mut v_nested = vec![
            Some(Some(1)),
            Some(None),
            None,
            Some(Some(0)),
            Some(Some(2)),
        ];
        compact(&mut v_nested);
        assert_eq!(v_nested, vec![Some(Some(1)), Some(Some(2))]);
    }

    /// Tests `compact` with `usize` and `isize` values.
    ///
    /// # Expected
    /// Removes all `0` values.
    #[test]
    fn test_compact_usize_isize() {
        let mut v_usize: Vec<usize> = vec![10, 0, 5, 0, 1];
        compact(&mut v_usize);
        assert_eq!(v_usize, vec![10, 5, 1]);

        let mut v_isize: Vec<isize> = vec![-1, 0, 2, -3, 0];
        compact(&mut v_isize);
        assert_eq!(v_isize, vec![-1, 2, -3]);
    }

    /// Tests `compact` on small integer types (`u8`, `i8`).
    ///
    /// # Expected
    /// Removes all `0` values.
    #[test]
    fn test_compact_small_ints() {
        let mut v_u8: Vec<u8> = vec![255, 0, 10, 0];
        compact(&mut v_u8);
        assert_eq!(v_u8, vec![255, 10]);

        let mut v_i8: Vec<i8> = vec![127, -128, 0, 5, 0];
        compact(&mut v_i8);
        assert_eq!(v_i8, vec![127, -128, 5]);
    }

    /// Tests `compact` on a vector of vectors.
    ///
    /// # Expected
    /// Removes inner empty vectors only; non-empty ones are kept regardless of content.
    #[test]
    fn test_compact_vec_of_vecs() {
        let mut v = vec![
            vec![1, 2],
            vec![],
            vec![0, 0],
            vec![3],
            vec![],
        ];
        compact(&mut v);
        assert_eq!(v, vec![vec![1, 2], vec![0, 0], vec![3]]);
    }
}
