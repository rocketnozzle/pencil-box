use std::collections::HashSet;
use std::hash::Hash;
use ahash::AHashSet;

/// Computes the difference between a primary list and multiple exclusion lists using `HashSet`,
/// returning a new owned vector of elements.
///
/// # Type Parameters
/// - `T`: The element type. Must implement `Clone`, `Eq`, and `Hash`.
///
/// # Arguments
/// - `to_compare`: A vector of values to retain if not found in `others`.
/// - `others`: A reference to a list of reference vectors containing values to exclude.
///
/// # Returns
/// A new `Vec<T>` containing only the values from `to_compare` not found in any of the `others`.
///
/// # Behavior
/// - Returns all values from `to_compare` that are not present in any `others`.
/// - Performs exact equality comparison (`==`) based on `Eq` and `Hash`.
///
/// # Performance
/// - Uses `std::collections::HashSet` (SipHash), which is secure but slower for large collections.
/// - Preallocates both `HashSet` and result vector to avoid extra heap allocations.
/// - Performs at most one `clone()` per item returned or excluded.
///
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

/// Computes the difference between a primary list and multiple exclusion lists using `AHashSet`
/// for faster performance on large datasets.
///
/// # Type Parameters
/// - `T`: The element type. Must implement `Clone`, `Eq`, and `Hash`.
///
/// # Arguments
/// - `to_compare`: A vector of values to retain if not found in `others`.
/// - `others`: A reference to a list of reference vectors containing values to exclude.
///
/// # Returns
/// A new `Vec<T>` containing only the values from `to_compare` not found in any of the `others`.
///
/// # Behavior
/// - Identical to [`difference`] in functionality, but uses `AHashSet` for speed.
/// - Performs exact equality comparison (`==`) based on `Eq` and `Hash`.
///
/// # Performance
/// - Uses [`ahash::AHashSet`], a high-performance, non-cryptographic hash algorithm.
/// - Much faster for large collections compared to `HashSet`, but not DoS-resistant.
/// - Preallocates both the `AHashSet` and result vector for efficiency.
/// - Performs at most one `clone()` per item returned or excluded.
///

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
