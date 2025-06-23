use ahash::AHashSet;
use std::collections::HashSet;
use std::hash::Hash;

/// Removes duplicate elements from a mutable vector using a standard `HashSet`.
///
/// # Type Parameters
/// - `T`: The type of elements in the vector. Must implement `Eq`, `Hash`, and `Clone`.
///
/// # Arguments
/// - `values`: A mutable reference to a vector of elements from which duplicates will be removed.
///
/// # Returns
/// This function does not return a value. It modifies the input vector in place by retaining only
/// the first occurrence of each unique item.
///
/// # Behavior
/// - Retains the first occurrence of each unique element and removes all subsequent duplicates.
/// - Preserves the relative order of the first occurrences.
/// - For empty vectors, no changes are made.
/// - Works with primitive, string, and complex types (e.g., structs, enums) that implement `Eq`, `Hash`, and `Clone`.
///
/// # Performance
/// - Uses the standard `HashSet`, which provides good general-purpose performance.
pub fn uniq<T: Eq + Hash + Clone>(values: &mut Vec<T>) {
    let mut seen = HashSet::with_capacity(values.len());
    values.retain(|item| seen.insert(item.clone()));
}

/// Removes duplicate elements from a mutable vector using `AHashSet` for faster hashing.
///
/// # Type Parameters
/// - `T`: The type of elements in the vector. Must implement `Eq`, `Hash`, and `Clone`.
///
/// # Arguments
/// - `values`: A mutable reference to a vector of elements from which duplicates will be removed.
///
/// # Returns
/// This function does not return a value. It modifies the input vector in place by retaining only
/// the first occurrence of each unique item.
///
/// # Behavior
/// - Retains the first occurrence of each unique element and removes all subsequent duplicates.
/// - Preserves the relative order of the first occurrences.
/// - For empty vectors, no changes are made.
/// - Works with primitive, string, and complex types (e.g., structs, enums) that implement `Eq`, `Hash`, and `Clone`.
///
/// # Performance
/// - Uses `AHashSet` from the `ahash` crate, offering faster hashing and better performance on large datasets or in performance-sensitive scenarios.
pub fn uniq_performant<T: Eq + Hash + Clone>(values: &mut Vec<T>) {
    let mut seen = AHashSet::with_capacity(values.len());
    values.retain(|item| seen.insert(item.clone()));
}


