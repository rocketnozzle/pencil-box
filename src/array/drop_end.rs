/// Truncates elements from the **end** of a vector, dropping the specified number of items in place.
///
/// # Type Parameters
/// - `T`: The element type contained in the vector. No specific traits are required.
///
/// # Arguments
/// - `values`: A mutable reference to the vector from which elements will be removed.
/// - `no_of_elements_to_drop`: The number of elements to remove from the end of the vector.
///
/// # Returns
/// This function returns no value. It modifies the input vector in place.
///
/// # Behavior
/// - Removes the last `no_of_elements_to_drop` elements from the vector.
/// - If `no_of_elements_to_drop` is `0`, the vector is left unchanged.
/// - If `no_of_elements_to_drop` is greater than or equal to the vector’s length, the vector is cleared.
///
/// # Performance
/// - ✅ In-place operation with **O(1)** time complexity.
/// - 🚫 No reallocation or element cloning occurs.
/// - ⚡ Very fast: uses `.truncate()` internally, which adjusts the vector’s length without touching memory.
///
/// # Examples
///
/// ### ✂️ Remove the last few elements
/// ```
/// use pencil_box::array::drop_end::drop_end;
///
/// let mut data = vec![10, 20, 30, 40];
/// drop_end(&mut data, 2);
/// assert_eq!(data, vec![10, 20]);
/// ```
///
/// ### 🛑 Drop zero elements (no change)
/// ```
/// let mut data = vec![1, 2, 3];
/// drop_end(&mut data, 0);
/// assert_eq!(data, vec![1, 2, 3]);
/// ```
///
/// ### 💥 Drop more than the vector contains (clears the vector)
/// ```
/// let mut data = vec![5, 6];
/// drop_end(&mut data, 10);
/// assert!(data.is_empty());
/// ```
///
/// ### 🔤 Use with strings or owned types
/// ```
/// let mut data = vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()];
/// drop_end(&mut data, 1);
/// assert_eq!(data, vec!["apple", "banana"]);
/// ```
///
/// ### 📭 Start from an empty vector
/// ```
/// let mut data: Vec<i32> = vec![];
/// drop_end(&mut data, 3);
/// assert!(data.is_empty());
/// ```
pub fn drop_end<T>(values: &mut Vec<T>, no_of_elements_to_drop: usize) {
    if no_of_elements_to_drop == 0 {
        return;
    }
    let no_of_elements_to_drop = values.len().saturating_sub(no_of_elements_to_drop);
    values.truncate(no_of_elements_to_drop);
}
