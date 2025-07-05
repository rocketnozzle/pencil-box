/// 🧱 Creates a new vector of the specified size, filled with default values for the type.
///
/// # Type Parameters
/// - `T`: The type of elements in the vector. Must implement [`Default`] and [`Clone`].
///
/// # Arguments
/// - `size`: The number of elements to generate.
///
/// # Returns
/// A `Vec<T>` of length `size`, where each element is a clone of `T::default()`.
///
/// # Behavior
/// - Produces an empty vector if `size == 0`.
/// - Allocates space for `size` elements on the heap.
/// - Each element is a clone of the type’s default value, as returned by [`T::default()`].
///
/// # Performance
/// - ✅ Time complexity: **O(n)** where `n = size`.
/// - ✅ Space complexity: **O(n)** heap-allocated.
/// - ✅ Minimal overhead, efficient default initialization.
/// - 🚫 No in-place mutation; constructs a new vector.
///
/// # Examples
///
/// ## 🔢 Primitive Types
/// ```
/// use pencil_box::array::fill_default::fill_default;
///
/// let vec: Vec<i32> = fill_default(3);
/// assert_eq!(vec, vec![0, 0, 0]);
///
/// let vec: Vec<bool> = fill_default(2);
/// assert_eq!(vec, vec![false, false]);
/// ```
///
/// ## 🧵 Complex Types
/// ```
/// let vec: Vec<String> = fill_default(2);
/// assert_eq!(vec, vec![String::new(), String::new()]);
///
/// let vec: Vec<Vec<u8>> = fill_default(2);
/// assert_eq!(vec, vec![vec![], vec![]]);
/// ```
///
/// ## 🎭 Enum with Default Variant
/// ```
/// #[derive(Debug, PartialEq, Clone, Default)]
/// enum Status {
///     #[default]
///     Idle,
///     Busy,
/// }
///
/// let vec = fill_default::<Status>(3);
/// assert_eq!(vec, vec![Status::Idle, Status::Idle, Status::Idle]);
/// ```
///
/// # Panic Safety
/// ✅ This function is panic-free for all valid `size` inputs.
///
/// # Features
/// - 📦 No dependencies, pure safe Rust.
/// - 🧱 Ideal for zero-initialized buffers or defaulted collections.
/// - 🛠️ Composable with iterator pipelines and higher-level constructors.

pub fn fill_default<T: Clone + Default>(size: usize) -> Vec<T> {
    vec![T::default(); size]
}
