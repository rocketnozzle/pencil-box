use std::collections::HashSet;
use std::hash::Hash;
use ahash::AHashSet;

/// Computes the difference between a primary list and multiple exclusion lists using [`std::collections::HashSet`].
///
/// # Type Parameters
/// - `T`: The element type. Must implement [`Clone`], [`Eq`], and [`Hash`].
///
/// # Arguments
/// - `to_compare`: A vector of values to retain if not found in `others`.
/// - `others`: A reference to a list of reference vectors containing values to exclude.
///
/// # Returns
/// A new `Vec<T>` containing only the values from `to_compare` that are not found in any of the `others`.
///
/// # Behavior
/// - Returns all items from `to_compare` that are not present in any exclusion list in `others`.
/// - Performs equality comparison using `==`, backed by `Eq` + `Hash`.
///
/// # Performance
/// - Uses [`HashSet`] (SipHash): **secure and collision-resistant**, suitable for untrusted input.
/// - Preallocates capacity for efficiency and avoids unnecessary allocations.
/// - Performs at most one `clone()` per included or excluded item.
/// - For large datasets where security is not a concern, see [`difference_performant`].
///
/// # Examples
///
/// ### ✂️ Filter out excluded values
/// ```
/// use pencil_box::array::difference::difference;
///
/// let a = vec![1, 2, 3, 4, 5];
/// let b = vec![2, 4];
/// let c = vec![5];
///
/// let result = difference(&a, &vec![&b, &c]);
/// assert_eq!(result, vec![1, 3]);
/// ```
///
/// ### 🔤 Works with strings
/// ```
/// let a = vec!["apple", "banana", "cherry"];
/// let b = vec!["banana"];
/// let result = difference(&a, &vec![&b]);
/// assert_eq!(result, vec!["apple", "cherry"]);
/// ```
///
/// ### 📭 Handles empty inputs
/// ```
/// let a: Vec<i32> = vec![];
/// let b = vec![1, 2, 3];
/// let result = difference(&a, &vec![&b]);
/// assert!(result.is_empty());
/// ```

pub fn difference<T: Eq + Hash + Clone>(
    to_compare: &Vec<T>,
    others: &Vec<&Vec<T>>,
) -> Vec<T> {
    let capacity = others.iter().map(|sub_array| sub_array.len()).sum();
    let mut set: HashSet<T> = HashSet::with_capacity(capacity);

    for arr in others {
        for item in *arr {
            set.insert(item.clone()); // clone item into the set
        }
    }

    to_compare
        .iter()
        .filter(|item| !set.contains(item))
        .cloned()
        .collect()
}

/// Computes the difference between a primary list and multiple exclusion lists using [`AHashSet`] for maximum performance.
///
/// # Type Parameters
/// - `T`: The element type. Must implement [`Clone`], [`Eq`], and [`Hash`].
///
/// # Arguments
/// - `to_compare`: A vector of values to retain if not found in `others`.
/// - `others`: A reference to a list of reference vectors containing values to exclude.
///
/// # Returns
/// A new `Vec<T>` containing only the values from `to_compare` that are not found in any of the `others`.
///
/// # Behavior
/// - Identical in output to [`difference`], but optimized using [`ahash::AHashSet`] for faster performance.
/// - Equality comparison based on `==` (requires `Eq` + `Hash`).
///
/// # Performance
/// - ⚡ Uses [`AHashSet`], a fast, non-cryptographic hashing algorithm (Blake3-inspired).
/// - 🚀 Significantly faster than `HashSet` for large data, but **not DoS-resistant** (not safe for untrusted input).
/// - Preallocates exclusion set and result vector for efficiency.
/// - Performs at most one `clone()` per unique value processed.
///
/// # Examples
///
/// ### 🚀 Fast difference on large numbers
/// ```
/// use pencil_box::array::difference::difference_performant;
///
/// let a: Vec<_> = (0..100_000).collect();
/// let b: Vec<_> = (50_000..150_000).collect();
/// let result = difference_performant(&a, &vec![&b]);
/// assert_eq!(result.len(), 50_000);
/// ```
///
/// ### ⚠️ Not suitable for hostile input
/// ```text
/// AHashSet is not cryptographically secure. Use `difference` with HashSet if you're handling untrusted or externally-supplied keys.
/// ```
///
/// ### ✅ Identical logic to `difference`
/// ```
/// let a = vec![1, 2, 3, 4];
/// let b = vec![2, 4];
/// let result = difference_performant(&a, &vec![&b]);
/// assert_eq!(result, vec![1, 3]);
/// ```

pub fn difference_performant<T: Eq + Hash + Clone>(
    to_compare: &Vec<T>,
    others: &Vec<&Vec<T>>,
) -> Vec<T> {
    let capacity = others.iter().map(|sub_array| sub_array.len()).sum();
    let mut set: AHashSet<T> = AHashSet::with_capacity(capacity);

    for arr in others {
        for item in *arr {
            set.insert(item.clone()); // clone item into the set
        }
    }

    to_compare
        .iter()
        .filter(|item| !set.contains(item))
        .cloned()
        .collect()
}
