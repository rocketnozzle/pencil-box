/// Creates a new vector of the specified size, filled with clones of the given value.
///
/// # Type Parameters
/// - `T`: The type of elements in the vector. Must implement [`Clone`].
///
/// # Arguments
/// - `value`: A reference to the value to repeat. This value will be cloned for each element.
/// - `size`: The number of elements to generate.
///
/// # Returns
/// A `Vec<T>` of length `size`, where each element is a clone of the provided `value`.
///
/// # Behavior
/// - Produces an empty vector if `size == 0`.
/// - Allocates space for `size` elements and fills them using repeated clones of `value`.
/// - Supports scalars, strings, vectors, enums, and other types implementing [`Clone`].
///
/// # Performance
/// - ✅ Linear time complexity **O(n)** where `n = size`.
/// - ✅ Allocates exactly `size` elements on the heap.
/// - ✅ Efficient and minimal memory overhead.
/// - 🚫 No in-place mutation; constructs a new vector.
///
/// # Examples
///
/// ## 🔢 Scalar Types
/// ```
/// use pencil_box::array::fill_value;
///
/// let vec = fill_value(&42, 4);
/// assert_eq!(vec, vec![42, 42, 42, 42]);
/// ```
///
/// ## ✅ Boolean
/// ```
/// let vec = fill_value(&true, 3);
/// assert_eq!(vec, vec![true, true, true]);
/// ```
///
/// ## 🧵 Complex Types
/// ```
/// let vec = fill_value(&String::from("hi"), 2);
/// assert_eq!(vec, vec!["hi".to_string(), "hi".to_string()]);
///
/// let vec = fill_value(&vec![1, 2], 3);
/// assert_eq!(vec, vec![vec![1, 2], vec![1, 2], vec![1, 2]]);
/// ```
///
/// ## 🎭 Enum Types
/// ```
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
/// # Panic Safety
/// - ✅ This function is panic-free under all valid inputs.
///
/// # Features
/// - 📦 No dependencies, pure safe Rust.
/// - 🧱 Suitable for initializing collections with fixed repeated values.
/// - 🛠️ Composable with iterator pipelines or collection builders.
pub fn fill_default<T: Clone + Default>(size: usize) -> Vec<T> {
    vec![T::default(); size]
}
