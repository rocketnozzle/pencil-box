#[cfg(test)]
mod tests {
    use pencil_box::array::chunk::chunk;

    /// Tests chunking a primitive `Vec<i32>` into even-sized groups.
    ///
    /// # Expected
    /// Returns chunks of size 2, with the last chunk possibly smaller.
    #[test]
    fn test_primitive_types() {
        let data = vec![1, 2, 3, 4, 5];
        let result = chunk(&data, 2).unwrap();
        let expected = vec![vec![1, 2], vec![3, 4], vec![5]];
        assert_eq!(
            result, expected,
            "Primitive chunking failed. Expected {:?}, got {:?}",
            expected, result
        );
    }

    /// Tests chunking a vector of owned `String` values.
    ///
    /// # Expected
    /// Preserves data and order with chunking applied correctly.
    #[test]
    fn test_strings() {
        let data = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = chunk(&data, 2).unwrap();
        let expected = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string()],
        ];
        assert!(
            result == expected,
            "String chunking failed. Expected {:?}, got {:?}",
            expected,
            result
        );
    }

    /// Tests chunking a vector of custom struct instances.
    ///
    /// # Expected
    /// Each chunk retains struct integrity and preserves order.
    #[test]
    fn test_structs() {
        #[derive(Clone, Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        let data = vec![
            Point { x: 1, y: 2 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 6 },
        ];
        let result = chunk(&data, 2).unwrap();
        assert!(
            result.len() == 2,
            "Expected 2 chunks, got {}. Full result: {:?}",
            result.len(),
            result
        );
        assert!(
            result[0][0].x == 1,
            "First struct x field mismatch. Expected 1, got {}",
            result[0][0].x
        );
    }

    /// Tests chunking a vector of enum values with different variants.
    ///
    /// # Expected
    /// Enums should be chunked without losing variant fidelity or order.
    #[test]
    fn test_enums() {
        #[derive(Clone, Debug, PartialEq)]
        enum Status {
            Ok,
            Error(String),
        }

        let data = vec![Status::Ok, Status::Error("fail".into()), Status::Ok];
        let result = chunk(&data, 2).unwrap();
        let expected = vec![
            vec![Status::Ok, Status::Error("fail".into())],
            vec![Status::Ok],
        ];
        assert!(
            result == expected,
            "Enum chunking failed. Expected {:?}, got {:?}",
            expected,
            result
        );
    }

    /// Tests chunking nested collections like `Vec<Vec<_>>`.
    ///
    /// # Expected
    /// Preserves inner vectors and groups them into outer chunks correctly.
    #[test]
    fn test_collections() {
        let data = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let result = chunk(&data, 2).unwrap();
        let expected = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6]]];
        assert!(
            result == expected,
            "Nested collection chunking failed. Expected {:?}, got {:?}",
            expected,
            result
        );
    }

    /// Tests the case where the input is an empty vector.
    ///
    /// # Expected
    /// Returns an empty result without errors.
    #[test]
    fn test_empty_input() {
        let data: Vec<i32> = vec![];
        let result = chunk(&data, 3).unwrap();
        assert!(
            result.is_empty(),
            "Expected empty result for empty input, got {:?}",
            result
        );
    }

    /// Tests the case where the chunk size is 0.
    ///
    /// # Expected
    /// Returns an error due to invalid input.
    #[test]
    fn test_chunk_size_zero() {
        let data = vec![1, 2, 3];
        let result = chunk(&data, 0);
        assert!(
            result.is_err(),
            "Expected error for chunk_size = 0, got: {:?}",
            result
        );
    }

    /// Tests the case where the chunk size is larger than the input length.
    ///
    /// # Expected
    /// Returns a single chunk containing the full input vector.
    #[test]
    fn test_chunk_size_larger_than_input() {
        let data = vec![1, 2, 3];
        let chunk_size = 10;
        let result = chunk(&data, chunk_size).unwrap();
        let expected = vec![vec![1, 2, 3]];
        assert!(
            result == expected,
            "Oversized chunk_size should return full array as one chunk. Expected {:?}, got {:?}",
            expected,
            result
        );
    }
}
