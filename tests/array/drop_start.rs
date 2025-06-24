#[cfg(test)]
mod tests {
    use pencil_box::array::drop_start::drop_start;

    /// Tests the case where 0 elements are dropped from a non-empty vector.
    ///
    /// # Expected
    /// The vector remains unchanged.
    #[test]
    fn test_drop_start_zero_elements() {
        let mut data = vec![1, 2, 3];
        drop_start(&mut data, 0);
        assert_eq!(data, vec![1, 2, 3]);
    }

    /// Tests the case where fewer elements than the vector length are dropped.
    ///
    /// # Expected
    /// The first N elements are removed; the rest are preserved.
    #[test]
    fn test_drop_start_less_than_length() {
        let mut data = vec![1, 2, 3, 4, 5];
        drop_start(&mut data, 2);
        assert_eq!(data, vec![3, 4, 5]);
    }

    /// Tests the case where the number of elements dropped equals the vector length.
    ///
    /// # Expected
    /// The entire vector is cleared.
    #[test]
    fn test_drop_start_exact_length() {
        let mut data = vec![1, 2, 3];
        drop_start(&mut data, 3);
        assert!(data.is_empty());
    }

    /// Tests the case where more elements are dropped than the vector contains.
    ///
    /// # Expected
    /// The vector is cleared without panicking.
    #[test]
    fn test_drop_start_more_than_length() {
        let mut data = vec![1, 2, 3];
        drop_start(&mut data, 10);
        assert!(data.is_empty());
    }

    /// Tests dropping elements from an already empty vector.
    ///
    /// # Expected
    /// The vector remains empty; no panic occurs.
    #[test]
    fn test_drop_start_from_empty_vec() {
        let mut data: Vec<i32> = vec![];
        drop_start(&mut data, 5);
        assert!(data.is_empty());
    }

    /// Tests dropping from a singleton vector with both 1 (empties it) and 0 (leaves it unchanged).
    ///
    /// # Expected
    /// Correct behavior in both edge cases.
    #[test]
    fn test_drop_start_from_singleton_vec() {
        let mut data = vec![42];
        drop_start(&mut data, 1);
        assert!(data.is_empty());

        let mut data = vec![42];
        drop_start(&mut data, 0);
        assert_eq!(data, vec![42]);
    }

    /// Tests dropping all elements from a large vector.
    ///
    /// # Expected
    /// Vector is fully cleared and remains empty.
    #[test]
    fn test_drop_start_entire_large_vec() {
        let mut data: Vec<_> = (0..1000).collect();
        drop_start(&mut data, 1000);
        assert!(data.is_empty());
    }

    /// Tests dropping a portion from a large vector.
    ///
    /// # Expected
    /// The correct number of leading elements are removed.
    #[test]
    fn test_drop_start_partially_large_vec() {
        let mut data: Vec<_> = (0..1000).collect();
        drop_start(&mut data, 250);
        assert_eq!(data.len(), 750);
        assert_eq!(data.first(), Some(&250));
    }

    /// Tests dropping from a vector of owned `String`s.
    ///
    /// # Expected
    /// Retains trailing strings and removes the specified number from the start.
    #[test]
    fn test_drop_start_strings() {
        let mut data = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        drop_start(&mut data, 2);
        assert_eq!(data, vec!["c"]);
    }

    /// Tests dropping 0 elements from an empty vector.
    ///
    /// # Expected
    /// The vector remains empty with no side effects.
    #[test]
    fn test_drop_start_zero_on_empty_vec() {
        let mut data: Vec<i32> = vec![];
        drop_start(&mut data, 0);
        assert!(data.is_empty());
    }
}
