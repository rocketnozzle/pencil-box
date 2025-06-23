/// Splits a slice into chunks of a specified size, cloning elements into new `Vec`s.
///
/// # Type Parameters
/// - `T`: The type of elements in the input slice. Must implement `Clone`.
///
/// # Arguments
/// - `array`: A reference to a slice of elements to be chunked.
/// - `chunk_size`: The number of elements per chunk. Must be greater than 0.
///
/// # Returns
/// Returns a `Result`:
/// - `Ok(Vec<Vec<T>>)` containing the chunked slices as vectors.
/// - `Err(&'static str)` if `chunk_size` is 0.
///
/// # Behavior
/// - If `array` is empty, returns an empty vector: `Ok(vec![])`.
/// - If `chunk_size >= array.len()`, returns a single chunk with all elements cloned.
/// - If `chunk_size < array.len()`, returns multiple chunks of up to `chunk_size` elements.
/// - If `chunk_size == 0`, returns an `Err`.
///
/// # Performance
/// Preallocates space for the outer vector using:
/// `(array.len() + chunk_size - 1) / chunk_size`
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

