/// 🔍 Finds all indices in a slice where the given predicate (matcher) returns true.
///
/// # Type Parameters
/// - `T`: The type of elements in the input slice.
/// - `M`: A predicate function that takes a reference to an element and returns `true` if it matches.
///
/// # Arguments
/// - `values`: A reference to a slice of elements to be scanned.
/// - `matcher`: A closure or function that determines whether an element should be included based on a condition.
///
/// # Returns
/// A `Vec<usize>` containing the indices of all elements that match the predicate.
///
/// # Behavior
/// - Iterates through the entire input slice.
/// - Applies `matcher` to each element.
/// - Collects the `index` of each element for which the matcher returns `true`.
/// - If no element matches, returns an empty vector.
///
/// # Performance
/// - ✅ Linear time complexity **O(n)**, where *n* is the number of elements in the input slice.
/// - ✅ Memory-efficient: pre-allocates capacity equal to the input length (worst-case).
/// - 🚫 No heap allocations for non-matching elements.
///
/// # Examples
/// 🔢 Basic numeric match:
/// ```rust
/// use pencil_box::array::find_indexes::find_indexes;
///
/// let values = [1, 2, 3, 4, 5, 6];
/// let even_indices = find_indexes(&values, |x| x % 2 == 0);
/// assert_eq!(even_indices, vec![1, 3, 5]);
/// ```
///
/// 🔤 String match:
/// ```rust
/// let str_values = ["apple", "banana", "apricot"];
/// let a_indices = find_indexes(&str_values, |s| s.starts_with('a'));
/// assert_eq!(a_indices, vec![0, 2]);
/// ```
///
/// 🧱 Matching on struct fields:
/// ```rust
/// #[derive(Debug)]
/// struct User {
///     id: u32,
///     name: &'static str,
/// }
///
/// let users = [
///     User { id: 1, name: "Alice" },
///     User { id: 2, name: "Bob" },
///     User { id: 3, name: "Alice" },
/// ];
///
/// let alices = find_indexes(&users, |u| u.name == "Alice");
/// assert_eq!(alices, vec![0, 2]);
/// ```
///
/// 🎭 Matching on enum variants:
/// ```rust
/// #[derive(Debug)]
/// enum Status {
///     Ready,
///     Pending,
///     Failed(u32),
/// }
///
/// let states = [
///     Status::Ready,
///     Status::Failed(404),
///     Status::Pending,
///     Status::Failed(500),
/// ];
///
/// let failed_indices = find_indexes(&states, |s| matches!(s, Status::Failed(_)));
/// assert_eq!(failed_indices, vec![1, 3]);
/// ```
///
/// # Panic Safety
/// ✅ This function is panic-free for all valid inputs.
///
pub fn find_indexes<T, M: Fn(&T) -> bool>(values: &[T], matcher: M) -> Vec<usize> {
    let mut indexes = Vec::with_capacity(values.len());
    for (index, value) in values.iter().enumerate() {
        if matcher(value) {
            indexes.push(index);
        }
    }
    indexes
}
