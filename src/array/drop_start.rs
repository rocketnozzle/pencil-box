/// Truncates elements from the start of a vector, dropping the specified number of items in place.
///
/// # Type Parameters
/// - `T`: The element type contained in the vector.
///
/// # Arguments
/// - `values`: A mutable reference to the vector from which elements will be removed.
/// - `no_of_elements_to_drop`: The number of elements to remove from the start of the vector.
///
/// # Returns
/// This function does not return a value. It modifies the input vector in place.
///
/// # Behavior
/// - Removes the first `no_of_elements_to_drop` elements from the vector.
/// - If `no_of_elements_to_drop` is `0`, the vector remains unchanged.
/// - If `no_of_elements_to_drop` is greater than or equal to the vector’s length, the vector is cleared.
///
/// # Performance
/// - Time complexity is **O(n - k)** where `k` is the number of elements dropped,
///   since remaining elements must be shifted left.
/// - Performs in-place mutation using `drain` without reallocating or cloning.
/// - For frequent truncation from the start, consider using [`VecDeque`] for O(1) behavior.
///
/// # Examples
///
/// ## ✅ Drop a few elements from the start
/// ```
/// use pencil_box::array::drop_start::drop_start;
///
/// let mut data = vec![10, 20, 30, 40];
/// drop_start(&mut data, 2);
/// assert_eq!(data, vec![30, 40]);
/// ```
///
/// ## ⛔ Drop more elements than present (clears the vector)
/// ```
/// let mut data = vec![1, 2, 3];
/// drop_start(&mut data, 10);
/// assert!(data.is_empty());
/// ```
///
/// ## 🔁 Drop exactly the length (clears the vector)
/// ```
/// let mut data = vec![7, 8, 9];
/// drop_start(&mut data, 3);
/// assert!(data.is_empty());
/// ```
///
/// ## ⚙️ Drop zero elements (no-op)
/// ```
/// let mut data = vec![1, 2, 3];
/// drop_start(&mut data, 0);
/// assert_eq!(data, vec![1, 2, 3]);
/// ```
///
/// ## 🧵 Works with `String`s and owned types
/// ```
/// let mut data = vec!["a".to_string(), "b".to_string(), "c".to_string()];
/// drop_start(&mut data, 1);
/// assert_eq!(data, vec!["b", "c"]);
/// ```
///
/// ## 📭 Empty input vector
/// ```
/// let mut data: Vec<i32> = vec![];
/// drop_start(&mut data, 3); // no panic
/// assert!(data.is_empty());
/// ```

pub fn drop_start<T>(values: &mut Vec<T>, no_of_elements_to_drop: usize) {
    if no_of_elements_to_drop == 0 {
        return;
    }

    if no_of_elements_to_drop >= values.len() {
        values.clear();
    } else {
        values.drain(0..no_of_elements_to_drop);
    }
}
