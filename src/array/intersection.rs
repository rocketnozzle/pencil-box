use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Computes the intersection of multiple collections, returning only elements common to **all** inputs.
///
/// # Type Parameters
///
/// - `T`: The element type. Must implement `Clone`, `Eq`, and `Hash`.
/// - `U`: A slice-like container that implements `AsRef<[T]>`.
///
/// # Arguments
///
/// - `values`: A slice of collections (`&[U]`) to be intersected, where each `U` can be converted into a slice of `T`.
///
/// # Returns
///
/// A `Vec<T>` containing only those elements that appear in **every** input collection.
/// The result does **not** preserve the original order and contains **no duplicates**.
///
/// # Supported Input Types
///
/// Any slice-like container that implements `AsRef<[T]>`, including:
///
/// - `&[&[T]]`
/// - `&[Vec<T>]`
/// - `&[Box<[T]>]`
/// - `&Vec<Vec<T>>`
/// - `Vec<&[T]>`
/// - `Vec<&Vec<T>>`
/// - `Vec<Vec<T>>`
/// - `Vec<Box<[T]>>`
/// - `&[[T; N]]` and `&[T; N]`
///
/// # Behavior
///
/// - Each collection is scanned for distinct elements.
/// - Elements are counted only once per collection (i.e., duplicates within a single collection are ignored).
/// - Only elements that appear in **all** collections are returned.
/// - If `values` is empty, returns an empty vector.
/// - If any single input collection is empty, the result is also empty.
/// - The output contains no duplicates.
///
/// # Performance
///
/// - **Time Complexity**: O(n × m), where `n` is the number of input collections and `m` is the average length of each collection.
/// - **Space Complexity**: O(u), where `u` is the number of unique elements across all collections.
///
/// # Panic Safety
///
/// This function is 100% panic-free on valid input.
///
/// # Examples
///
/// 🧪 `&[&[T]]`
/// ```
/// use pencil_box::array::intersection::intersection;
/// let a = &[1, 2, 3][..];
/// let b = &[2, 3, 4][..];
/// let c = &[2, 3, 5][..];
/// let result = intersection(&[a, b, c]);
/// assert_eq!(result, vec![2, 3]);
/// ```
///
/// 🧪 `&[Vec<T>]`
/// ```
/// let a = vec![1, 2, 3];
/// let b = vec![2, 3, 4];
/// let c = vec![2, 3, 5];
/// let result = intersection(&[a, b, c]);
/// assert_eq!(result, vec![2, 3]);
/// ```
///
/// 🧪 `&[Box<[T]>]`
/// ```
/// let a: Box<[i32]> = Box::new([1, 2, 3]);
/// let b: Box<[i32]> = Box::new([2, 3, 4]);
/// let c: Box<[i32]> = Box::new([2, 3, 5]);
/// let result = intersection(&[a, b, c]);
/// assert_eq!(result, vec![2, 3]);
/// ```
///
/// 🧪 `&Vec<Vec<T>>`
/// ```
/// let input = vec![
///     vec![1, 2, 3],
///     vec![2, 3, 4],
///     vec![2, 3, 5],
/// ];
/// let result = intersection(&input);
/// assert_eq!(result, vec![2, 3]);
/// ```
///
/// 🧪 `Vec<&[T]>`
/// ```
/// let a = &[1, 2, 3][..];
/// let b = &[2, 3, 4][..];
/// let c = &[2, 3, 5][..];
/// let input: Vec<&[i32]> = vec![a, b, c];
/// let result = intersection(&input);
/// assert_eq!(result, vec![2, 3]);
/// ```
///
/// 🧪 `Vec<&Vec<T>>`
/// ```
/// let a = vec![1, 2, 3];
/// let b = vec![2, 3, 4];
/// let c = vec![2, 3, 5];
/// let input: Vec<&Vec<i32>> = vec![&a, &b, &c];
/// let result = intersection(&input);
/// assert_eq!(result, vec![2, 3]);
/// ```
///
/// 🧪 `Vec<Vec<T>>`
/// ```
/// let input = vec![
///     vec![1, 2, 3],
///     vec![2, 3, 4],
///     vec![2, 3, 5],
/// ];
/// let result = intersection(&input);
/// assert_eq!(result, vec![2, 3]);
/// ```
///
/// 🧪 `Vec<Box<[T]>>`
/// ```
/// let a: Box<[i32]> = Box::new([1, 2, 3]);
/// let b: Box<[i32]> = Box::new([2, 3, 4]);
/// let c: Box<[i32]> = Box::new([2, 3, 5]);
/// let input = vec![a, b, c];
/// let result = intersection(&input);
/// assert_eq!(result, vec![2, 3]);
/// ```
///
/// 🧪 `&[[T; N]]`
/// ```
/// let a: [i32; 3] = [1, 2, 3];
/// let b: [i32; 3] = [2, 3, 4];
/// let c: [i32; 3] = [2, 3, 5];
/// let input: &[[i32; 3]] = &[a, b, c];
/// let result = intersection(input);
/// assert_eq!(result, vec![2, 3]);
/// ```
///
/// 🧪 `&[T; N]`
/// ```
/// let a: [i32; 3] = [1, 2, 3];
/// let b: [i32; 3] = [2, 3, 4];
/// let c: [i32; 3] = [2, 3, 5];
/// let input = [&a[..], &b[..], &c[..]];
/// let result = intersection(&input);
/// assert_eq!(result, vec![2, 3]);
/// ```
///
/// 🧪 `String` (owned)
/// ```
/// let a = vec!["a".to_string(), "b".to_string()];
/// let b = vec!["b".to_string(), "c".to_string()];
/// let c = vec!["b".to_string(), "d".to_string()];
/// let result = intersection(&[a, b, c]);
/// assert_eq!(result, vec!["b"]);
/// ```
///
/// 🧪 `&str` (references)
/// ```
/// let a = ["a", "b"];
/// let b = ["b", "c"];
/// let c = ["b", "d"];
/// let result = intersection(&[&a[..], &b[..], &c[..]]);
/// assert_eq!(result, vec!["b"]);
/// ```
///
/// 🧪 Structs (owned)
/// ```
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Item { id: u8 }
///
/// let a = vec![Item { id: 1 }, Item { id: 2 }];
/// let b = vec![Item { id: 2 }, Item { id: 3 }];
/// let c = vec![Item { id: 2 }, Item { id: 4 }];
/// let result = intersection(&[a, b, c]);
/// assert_eq!(result, vec![Item { id: 2 }]);
/// ```
///
/// 🧪 Structs (references)
/// ```
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct Item { id: u8 }
///
/// let a = [Item { id: 1 }, Item { id: 2 }];
/// let b = [Item { id: 2 }, Item { id: 3 }];
/// let c = [Item { id: 2 }, Item { id: 4 }];
///
/// let va: Vec<&Item> = a.iter().collect();
/// let vb: Vec<&Item> = b.iter().collect();
/// let vc: Vec<&Item> = c.iter().collect();
/// let result = intersection(&[va, vb, vc]);
/// assert_eq!(result.len(), 1);
/// assert_eq!(result[0].id, 2);
/// ```

pub fn intersection<T, U>(values: &[U]) -> Vec<T>
where
    U: AsRef<[T]>,
    T: Clone + Eq + Hash,
{
    let mut count: HashMap<T, usize> = HashMap::new();
    for sub_array in values {
        let mut seen = HashSet::new();
        for item in sub_array.as_ref().iter() {
            if seen.insert(item) {
                count
                    .entry(item.clone())
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }
    }
    count
        .into_iter()
        .filter_map(|(key, value)| {
            if value == values.len() {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect()
}
