#[cfg(test)]
mod tests {
    use pencil_box::array::difference::{difference, difference_performant};

    /// Shared test helper to compare results between `difference` and `difference_performant`.
    fn assert_both_equal<T: Eq + std::hash::Hash + Clone + std::fmt::Debug>(
        to_compare: Vec<T>,
        others: Vec<&Vec<T>>,
        expected: Vec<T>,
    ) {
        let result_std = difference(&to_compare, &others);
        let result_ahash = difference_performant(&to_compare, &others);
        assert_eq!(result_std, expected);
        assert_eq!(result_ahash, expected);
    }

    /// Tests the case where both `to_compare` and `others` are empty.
    ///
    /// # Expected
    /// Returns an empty vector from both implementations.
    #[test]
    fn test_empty_array() {
        assert_both_equal::<i32>(vec![], vec![], vec![]);
    }

    /// Tests the case where `others` is empty but `to_compare` has elements.
    ///
    /// # Expected
    /// Returns a clone of all elements from `to_compare`.
    #[test]
    fn test_empty_others() {
        let input = vec![1, 2, 3];
        assert_both_equal(input.clone(), vec![], input);
    }

    /// Tests integer comparison with overlapping values.
    ///
    /// # Expected
    /// Excludes matching values from `others`.
    #[test]
    fn test_with_integers() {
        let to_compare = vec![1, 2, 3, 4];
        let b1 = vec![2, 4];
        let b2 = vec![5];
        let expected = vec![1, 3];
        assert_both_equal(to_compare, vec![&b1, &b2], expected);
    }

    /// Tests string comparison using owned `String` values.
    ///
    /// # Expected
    /// Returns only non-overlapping strings.
    #[test]
    fn test_with_strings() {
        let to_compare = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let skip = vec!["b".to_string()];
        let expected = vec!["a".to_string(), "c".to_string()];
        assert_both_equal(to_compare, vec![&skip], expected);
    }

    /// Tests with string slices (`&str`).
    ///
    /// # Expected
    /// Same behavior as with owned `String`.
    #[test]
    fn test_with_str_refs() {
        let to_compare = vec!["x", "y", "z"];
        let skip = vec!["y"];
        let expected = vec!["x", "z"];
        assert_both_equal(to_compare, vec![&skip], expected);
    }

    /// Tests boolean filtering.
    ///
    /// # Expected
    /// Returns only unmatched boolean values.
    #[test]
    fn test_with_bools() {
        let to_compare = vec![true, false];
        let skip = vec![false];
        let expected = vec![true];
        assert_both_equal(to_compare, vec![&skip], expected);
    }

    /// Tests floats using a safe wrapper around `f64`.
    ///
    /// # Notes
    /// `f64` does not implement `Hash` by default. We use a simple wrapper.
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct HashableF64(f64);

    impl Eq for HashableF64 {}
    impl std::hash::Hash for HashableF64 {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.to_bits().hash(state);
        }
    }

    #[test]
    fn test_with_floats() {
        let to_compare = vec![HashableF64(1.0), HashableF64(2.0), HashableF64(3.0)];
        let skip = vec![HashableF64(2.0)];
        let expected = vec![HashableF64(1.0), HashableF64(3.0)];
        assert_both_equal(to_compare, vec![&skip], expected);
    }

    /// Tests custom struct comparison.
    ///
    /// # Expected
    /// Returns only unique `MyStruct` values not found in `others`.
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct MyStruct {
        id: u32,
        name: String,
    }

    #[test]
    fn test_with_structs() {
        let a = MyStruct {
            id: 1,
            name: "A".into(),
        };
        let b = MyStruct {
            id: 2,
            name: "B".into(),
        };
        let c = MyStruct {
            id: 3,
            name: "C".into(),
        };
        let to_compare = vec![a.clone(), b.clone(), c.clone()];
        let skip = vec![b.clone()];
        let expected = vec![a, c];
        assert_both_equal(to_compare, vec![&skip], expected);
    }

    /// Tests custom enum comparison.
    ///
    /// # Expected
    /// Filters out the matching enum variant.
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    enum MyEnum {
        A,
        B,
        C,
    }

    #[test]
    fn test_with_enums() {
        use MyEnum::*;
        let to_compare = vec![A, B, C];
        let skip = vec![B];
        let expected = vec![A, C];
        assert_both_equal(to_compare, vec![&skip], expected);
    }
}
