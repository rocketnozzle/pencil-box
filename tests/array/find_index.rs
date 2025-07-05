#[cfg(test)]
mod tests {
    use pencil_box::array::find_index::find_index;

    /// ✅ Finds the index of the first even number in a list of integers.
    /// Expects index of the first element divisible by 2.
    #[test]
    fn test_find_first_even() {
        let values = [1, 3, 6, 8];
        let result = find_index(&values, |x| x % 2 == 0);
        assert_eq!(result, Some(2));
    }

    /// ✅ No element matches the predicate.
    /// Expects `None` as the result.
    #[test]
    fn test_no_match_returns_none() {
        let values = [1, 3, 5];
        let result = find_index(&values, |x| x % 2 == 0);
        assert_eq!(result, None);
    }

    /// ✅ Empty input slice.
    /// Expects `None`, and ensures function does not panic.
    #[test]
    fn test_empty_slice() {
        let values: [i32; 0] = [];
        let result = find_index(&values, |_| true);
        assert_eq!(result, None);
    }

    /// ✅ First element matches predicate.
    /// Ensures function exits early and returns index 0.
    #[test]
    fn test_first_element_match() {
        let values = [42, 7, 9];
        let result = find_index(&values, |x| *x == 42);
        assert_eq!(result, Some(0));
    }

    /// ✅ Matching based on a field in a struct.
    /// Expects index of first struct that satisfies the predicate.
    #[test]
    fn test_struct_field_match() {
        #[derive(Debug)]
        struct User {
            id: u32,
            active: bool,
        }

        let users = [
            User {
                id: 1,
                active: false,
            },
            User {
                id: 2,
                active: true,
            },
            User {
                id: 3,
                active: true,
            },
        ];

        let result = find_index(&users, |u| u.active);
        assert_eq!(result, Some(1));
    }

    /// ✅ Matching an enum variant.
    /// Expects the index of the first matching variant.
    #[test]
    fn test_enum_variant_match() {
        #[derive(Debug)]
        enum Status {
            Idle,
            Processing,
            Done,
        }

        let states = [Status::Idle, Status::Processing, Status::Done];
        let result = find_index(&states, |s| matches!(s, Status::Processing));
        assert_eq!(result, Some(1));
    }

    /// ✅ Matching on a string slice.
    /// Expects index of the first element that starts with "A".
    #[test]
    fn test_string_match() {
        let names = ["Bob", "Alice", "Anna"];
        let result = find_index(&names, |name| name.starts_with('A'));
        assert_eq!(result, Some(1));
    }

    /// ✅ Last element matches.
    /// Ensures full traversal and correct index.
    #[test]
    fn test_last_element_match() {
        let values = [0, 1, 2, 3, 99];
        let result = find_index(&values, |x| *x == 99);
        assert_eq!(result, Some(4));
    }
}
