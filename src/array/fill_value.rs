/// Creates a `Vec<T>` of a given size, filled with the provided value.
///
/// # Type Parameters
/// - `T`: The element type of the vector. Must implement [`Clone`].
///
/// # Arguments
/// - `value`: A reference to the value to fill the vector with. This value is cloned into each slot.
/// - `size`: The number of elements in the resulting vector.
///
/// # Returns
/// A new `Vec<T>` of length `size`, where every element is a clone of `value`.
///
/// # Examples
///
/// ## Scalar Types
/// ```
/// use pencil_box::array::fill_value;
///
/// let vec = fill_value(&42, 4);
/// assert_eq!(vec, vec![42, 42, 42, 42]);
///
/// let vec = fill_value(&true, 3);
/// assert_eq!(vec, vec![true, true, true]);
/// ```
///
/// ## Complex Types
/// ```
/// use pencil_box::array::fill_value;
///
/// let vec = fill_value(&String::from("hi"), 2);
/// assert_eq!(vec, vec!["hi".to_string(), "hi".to_string()]);
///
/// let vec = fill_value(&vec![1, 2], 3);
/// assert_eq!(vec, vec![vec![1, 2], vec![1, 2], vec![1, 2]]);
/// ```
///
/// ## Enum Type
/// ```
/// use pencil_box::array::fill_value;
///
/// #[derive(Debug, PartialEq, Clone)]
/// enum Status {
///     Ready,
///     Error(String),
/// }
///
/// let vec = fill_value(&Status::Error("x".into()), 2);
/// assert_eq!(
///     vec,
///     vec![Status::Error("x".into()), Status::Error("x".into())]
/// );
/// ```
///
/// # Behavior
/// - Returns an empty vector if `size` is `0`.
/// - Clones `value` `size` times into a new vector.
/// - Useful for initializing vectors with a specific repeated value.
///
/// # Time Complexity
/// - **O(n)** where `n` is the `size` of the vector.
///
/// # Memory
/// - Allocates memory for `size` elements on the heap.
///
/// # Panics
/// - Never panics under valid input.
///
/// # Features
/// - Works with any type that implements [`Clone`], including scalars, strings, vectors, and enums.
pub fn fill_value<T: Clone>(value: &T, size: usize) -> Vec<T> {
    vec![value.clone(); size]
}
