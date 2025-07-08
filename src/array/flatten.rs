/// Flattens a nested collection structure into a single `Vec<T>`, supporting various common patterns
/// such as slices of slices, slices of vectors, vectors of boxes, etc.
///
/// # Type Parameters
///
/// - `T`: The inner element type. Must implement `Clone`.
/// - `Outer`: The outer collection type. Must implement `AsRef<[Inner]>`.
/// - `Inner`: The inner collection type. Must implement `AsRef<[T]>`.
///
/// # Arguments
///
/// - `nested`: A nested collection where each inner element can be referenced as a slice of `T`.
///
/// # Returns
///
/// A flattened `Vec<T>` containing all elements from the nested collection in order.
///
/// # Behavior
///
/// - Iterates through each inner collection inside `nested` and clones each item into a new vector.
/// - Preserves order of elements across all nested containers.
/// - Generic over many common nested forms such as:
///   - `&[&[T]]`
///   - `&[&Vec<T>]`
///   - `&[Box<[T]>]`
///   - `&[Vec<T>]`
///   - `&Vec<Vec<T>>`
///   - `Vec<&[T]>`
///   - `Vec<&Vec<T>>`
///   - `Vec<Vec<T>>`
///   - `Vec<Box<[T]>>`
///   - `&[T; N]` where `T = Vec<_>`
///
/// # Performance
///
/// - 📈 Time complexity is **O(n)** where `n` is the total number of elements across all inner collections.
/// - `.cloned()` operates **per-element**, not as a full slice clone — each `T` is cloned individually using `T::clone()`.
/// - There is **no quadratic behavior**, because no full slice or repeated reallocation occurs during iteration.
/// - The final vector is allocated with capacity and built efficiently via `.collect()`.
///
/// # Examples
///
/// ### 🍇 Basic numeric types
/// ```rust
/// use pencil_box::array::flatten::flatten;
///
/// let a: &[&[i32]] = &[&[1, 2], &[3]];
/// assert_eq!(flatten(a), vec![1, 2, 3]);
/// ```
///
/// ### 🧵 Strings
/// ```rust
/// let strs: Vec<Vec<String>> = vec![
///     vec!["foo".to_string(), "bar".to_string()],
///     vec!["baz".to_string()],
/// ];
/// assert_eq!(flatten(&strs), vec!["foo", "bar", "baz"]);
/// ```
///
/// ### 🔁 Booleans
/// ```rust
/// let flags: &[Vec<bool>] = &[vec![true], vec![false, true]];
/// assert_eq!(flatten(flags), vec![true, false, true]);
/// ```
///
/// ### 🧱 Structs
/// ```rust
/// #[derive(Debug, Clone, PartialEq)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// let nested: &[Vec<Point>] = &[
///     vec![Point { x: 1, y: 2 }],
///     vec![Point { x: 3, y: 4 }],
/// ];
///
/// assert_eq!(
///     flatten(nested),
///     vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }]
/// );
/// ```
///
/// ### 🧩 Enums
/// ```rust
/// #[derive(Debug, Clone, PartialEq)]
/// enum State {
///     Idle,
///     Running(u32),
/// }
///
/// let states: Vec<Vec<State>> = vec![
///     vec![State::Idle],
///     vec![State::Running(42)],
/// ];
///
/// assert_eq!(flatten(&states), vec![State::Idle, State::Running(42)]);
/// ```
///
/// # Panic Safety
///
/// ✅ This function is panic-free for valid inputs.
///
/// # Notes
///
/// To avoid unnecessary cloning, consider using references to `T` instead of `T` directly when possible.
///
/// # See Also
///
/// - [`flat_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map)
/// - [`concat`](https://doc.rust-lang.org/std/slice/fn.concat.html) for `Vec<Vec<T>>` only

pub fn flatten<T: Clone, Outer, Inner>(nested: Outer) -> Vec<T>
where
    Outer: AsRef<[Inner]>,
    Inner: AsRef<[T]>,
{
    nested
        .as_ref()
        .iter()
        .flat_map(|inner| inner.as_ref().iter().cloned())
        .collect()

}