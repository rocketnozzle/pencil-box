use ahash::AHashSet;
use std::collections::HashSet;
use std::hash::Hash;

/// 🔁 Removes duplicate elements from a mutable vector using the standard [`HashSet`] (SipHash).
///
/// # Type Parameters
/// - `T`: The type of elements in the vector. Must implement [`Eq`], [`Hash`], and [`Clone`].
///
/// # Arguments
/// - `values`: A mutable reference to the vector to deduplicate.
///
/// # Returns
/// This function returns no value. It modifies the input vector in-place by retaining only the
/// **first occurrence** of each unique item.
///
/// # Behavior
/// - Duplicates are identified using `Eq` + `Hash`.
/// - The **first** occurrence of each item is kept; subsequent duplicates are removed.
/// - Preserves the **original order** of retained elements.
/// - Empty vectors are left unchanged.
/// - Works with primitives, strings, enums, and any type that implements `Eq`, `Hash`, and `Clone`.
///
/// # Performance
/// - Uses [`std::collections::HashSet`] (SipHash), secure and collision-resistant.
/// - Slower than [`AHashSet`] on large datasets, but safer for untrusted input.
///
/// # Examples
///
/// ### 🔢 Remove duplicate integers
/// ```
/// use pencil_box::array::uniq::uniq;
///
/// let mut nums = vec![1, 2, 2, 3, 1];
/// uniq(&mut nums);
/// assert_eq!(nums, vec![1, 2, 3]);
/// ```
///
/// ### 🔤 Remove duplicate strings
/// ```
/// let mut words = vec!["hi".to_string(), "hi".to_string(), "there".to_string()];
/// uniq(&mut words);
/// assert_eq!(words, vec!["hi", "there"]);
/// ```
///
/// ### 🧱 Works with enums or custom types (if they implement `Eq`, `Hash`, `Clone`)
/// ```
/// #[derive(Hash, Eq, PartialEq, Clone, Debug)]
/// enum Fruit { Apple, Banana, Apple }
///
/// let mut fruits = vec![Fruit::Apple, Fruit::Banana, Fruit::Apple];
/// uniq(&mut fruits);
/// assert_eq!(fruits.len(), 2);
/// ```
pub fn uniq<T: Eq + Hash + Clone>(values: &mut Vec<T>) {
    let mut seen = HashSet::with_capacity(values.len());
    values.retain(|item| seen.insert(item.clone()));
}

/// ⚡ Removes duplicate elements from a mutable vector using [`AHashSet`] for faster hashing.
///
/// # Type Parameters
/// - `T`: The type of elements in the vector. Must implement [`Eq`], [`Hash`], and [`Clone`].
///
/// # Arguments
/// - `values`: A mutable reference to the vector to deduplicate.
///
/// # Returns
/// This function returns no value. It modifies the input vector in-place by retaining only the
/// **first occurrence** of each unique item.
///
/// # Behavior
/// - Identical to [`uniq`], but uses a faster hash implementation (`AHashSet`).
/// - Retains the first instance, removes subsequent duplicates.
/// - Preserves input order of retained items.
///
/// # Performance
/// - Uses [`ahash::AHashSet`], a fast, non-cryptographic hashing algorithm.
/// - ⚠️ Not resistant to hash collision attacks — do **not** use with untrusted input.
/// - Excellent for large vectors in performance-critical paths.
///
/// # Examples
///
/// ### 🚀 Deduplicate large vectors efficiently
/// ```
/// use pencil_box::array::uniq::uniq_performant;
///
/// let mut data = (0..1_000_000).chain(0..500_000).collect::<Vec<_>>();
/// uniq_performant(&mut data);
/// assert_eq!(data.len(), 1_000_000);
/// ```
///
/// ### 💡 Identical logic to `uniq`
/// ```
/// let mut input = vec![1, 1, 2, 3];
/// uniq_performant(&mut input);
/// assert_eq!(input, vec![1, 2, 3]);
/// ```
///
/// ### 📭 No-op on empty vector
/// ```
/// let mut empty: Vec<i32> = vec![];
/// uniq_performant(&mut empty);
/// assert!(empty.is_empty());
/// ```
pub fn uniq_performant<T: Eq + Hash + Clone>(values: &mut Vec<T>) {
    let mut seen = AHashSet::with_capacity(values.len());
    values.retain(|item| seen.insert(item.clone()));
}


