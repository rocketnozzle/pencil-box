#[cfg(test)]
mod tests {
    use pencil_box::array::find_last_index::find_last_index;

    /// ✅ Finds the index of the last even number.
    /// Ensures the last match is returned.
    #[test]
    fn test_find_last_even() {
        let values = [1, 2, 3, 4, 6, 7, 6];
        let result = find_last_index(&values, |x| x % 2 == 0);
        assert_eq!(result, Some(6));
    }

    /// ✅ No element matches the predicate.
    /// Expects `None`.
    #[test]
    fn test_no_match_returns_none() {
        let values = [1, 3, 5, 7];
        let result = find_last_index(&values, |x| x % 2 == 0);
        assert_eq!(result, None);
    }

    /// ✅ Empty input slice.
    /// Returns `None` safely.
    #[test]
    fn test_empty_slice() {
        let values: [i32; 0] = [];
        let result = find_last_index(&values, |_| true);
        assert_eq!(result, None);
    }

    /// ✅ Only one element matches.
    /// Returns its index.
    #[test]
    fn test_single_match() {
        let values = [5, 8, 11];
        let result = find_last_index(&values, |x| *x == 8);
        assert_eq!(result, Some(1));
    }

    /// ✅ First element matches, but not others.
    /// Returns index 0.
    #[test]
    fn test_match_first_only() {
        let values = [10, 1, 2, 3];
        let result = find_last_index(&values, |x| *x == 10);
        assert_eq!(result, Some(0));
    }

    /// ✅ Last element matches the predicate.
    /// Ensures complete traversal.
    #[test]
    fn test_last_element_match() {
        let values = [3, 1, 4, 5];
        let result = find_last_index(&values, |x| *x == 5);
        assert_eq!(result, Some(3));
    }

    /// ✅ Matching by struct field.
    /// Returns last element with matching field.
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

        let result = find_last_index(&users, |u| u.name == "Alice");
        assert_eq!(result, Some(2));
    }

    /// ✅ Matching enum variant.
    /// Returns last matching variant index.
    #[test]
    fn test_enum_variant_match() {
        #[derive(Debug)]
        enum Status {
            Init,
            Error,
            Done,
        }

        let states = [Status::Init, Status::Error, Status::Init, Status::Error];

        let result = find_last_index(&states, |s| matches!(s, Status::Error));
        assert_eq!(result, Some(3));
    }

    /// ✅ All elements match.
    /// Should return index of the last one.
    #[test]
    fn test_all_elements_match() {
        let values = [1, 1, 1];
        let result = find_last_index(&values, |_| true);
        assert_eq!(result, Some(2));
    }
}
