/// 🧩 Splits a slice into chunks of a specified size, cloning elements into new `Vec`s.
///
/// # Type Parameters
/// - `T`: The type of elements in the input slice. Must implement [`Clone`].
///
/// # Arguments
/// - `array`: A reference to a slice of elements to be chunked.
/// - `chunk_size`: The number of elements per chunk. Must be greater than 0.
///
/// # Returns
/// Returns a [`Result`]:
/// - `Ok(Vec<Vec<T>>)` containing the chunked slices as new vectors.
/// - `Err(&'static str)` if `chunk_size` is `0`.
///
/// # Behavior
/// - If `array` is empty, returns an empty vector (`Ok(vec![])`).
/// - If `chunk_size >= array.len()`, returns a single chunk with all elements cloned.
/// - If `chunk_size < array.len()`, returns multiple chunks of up to `chunk_size` elements each.
/// - If `chunk_size == 0`, returns an error.
///
/// # Performance
/// - Preallocates the outer vector using: `(array.len() + chunk_size - 1) / chunk_size`.
/// - Each chunk is allocated with `Vec::from` (per `chunk.to_vec()`).
/// - Overall time complexity is **O(n)** and memory complexity is **O(n)** where `n = array.len()`.
///
/// # Examples
///
/// ### 📦 Chunk a simple array into fixed-size groups
/// ```
/// use pencil_box::array::chunk::chunk;
///
/// let input = vec![1, 2, 3, 4, 5];
/// let result = chunk(&input, 2).unwrap();
/// assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);
/// ```
///
/// ### 🧪 Chunk size equals array length (single chunk)
/// ```
/// let input = vec![10, 20, 30];
/// let result = chunk(&input, 3).unwrap();
/// assert_eq!(result, vec![vec![10, 20, 30]]);
/// ```
///
/// ### 📭 Empty input returns an empty result
/// ```
/// let input: Vec<i32> = vec![];
/// let result = chunk(&input, 3).unwrap();
/// assert!(result.is_empty());
/// ```
///
/// ### ⚠️ Invalid chunk size returns error
/// ```
/// let input = vec![1, 2, 3];
/// let result = chunk(&input, 0);
/// assert!(result.is_err());
/// ```
///
/// ### 🔤 Works with strings or other clonable types
/// ```
/// let input = vec!["a", "b", "c", "d"];
/// let result = chunk(&input, 2).unwrap();
/// assert_eq!(result, vec![vec!["a", "b"], vec!["c", "d"]]);
/// ```
pub fn chunk<T: Clone>(array: &[T], chunk_size: usize) -> Result<Vec<Vec<T>>, &'static str> {
    if chunk_size == 0 {
        return Err("chunk_size must be greater than 0");
    }

    if array.is_empty() {
        return Ok(vec![]);
    }

    if chunk_size >= array.len() {
        return Ok(vec![array.to_vec()]);
    }

    let mut chunks = Vec::with_capacity((array.len() + chunk_size - 1) / chunk_size);
    for chunk in array.chunks(chunk_size) {
        chunks.push(chunk.to_vec());
    }

    Ok(chunks)
}

