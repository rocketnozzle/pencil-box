#[cfg(test)]
mod tests {
    use pencil_box::array::find_indexes::find_indexes;

    /// ✅ Tests finding even numbers in a list of integers.
    /// Expects indices of all elements divisible by 2.
    #[test]
    fn test_find_even_indices() {
        let values = [1, 2, 3, 4, 6];
        let result = find_indexes(&values, |x| x % 2 == 0);
        assert_eq!(result, vec![1, 3, 4]);
    }

    /// ✅ Tests that no elements match the predicate.
    /// Expects an empty result vector.
    #[test]
    fn test_no_match_returns_empty_vec() {
        let values = [1, 3, 5];
        let result = find_indexes(&values, |x| *x > 10);
        assert!(result.is_empty());
    }

    /// ✅ Tests on an empty input slice.
    /// Expects an empty result vector with no panic.
    #[test]
    fn test_empty_input_slice() {
        let values: [i32; 0] = [];
        let result = find_indexes(&values, |_| true);
        assert!(result.is_empty());
    }

    /// ✅ Tests matching based on string prefix.
    /// Expects only the indices of elements that start with 'a'.
    #[test]
    fn test_string_prefix_match() {
        let values = ["alpha", "beta", "apricot"];
        let result = find_indexes(&values, |s| s.starts_with('a'));
        assert_eq!(result, vec![0, 2]);
    }

    /// ✅ Tests matching a field in a struct.
    /// Matches users with a specific name.
    #[test]
    fn test_struct_field_match() {
        #[derive(Debug)]
        struct User {
            id: u32,
            name: &'static str,
        }

        let users = [
            User {
                id: 1,
                name: "Alice",
            },
            User { id: 2, name: "Bob" },
            User {
                id: 3,
                name: "Alice",
            },
        ];

        let result = find_indexes(&users, |u| u.name == "Alice");
        assert_eq!(result, vec![0, 2]);
    }

    /// ✅ Tests matching enum variants.
    /// Matches all `Status::Failed(_)` variants.
    #[test]
    fn test_enum_variant_match() {
        #[derive(Debug)]
        enum Status {
            Ready,
            Pending,
            Failed(u32),
        }

        let states = [
            Status::Ready,
            Status::Failed(404),
            Status::Pending,
            Status::Failed(500),
        ];

        let result = find_indexes(&states, |s| matches!(s, Status::Failed(_)));
        assert_eq!(result, vec![1, 3]);
    }

    /// ✅ Tests that all elements match the predicate.
    /// Expects all indices in the result vector.
    #[test]
    fn test_all_match() {
        let values = [0, 1, 2];
        let result = find_indexes(&values, |_| true);
        assert_eq!(result, vec![0, 1, 2]);
    }
}
